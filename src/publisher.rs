use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        mpsc::{self, SyncSender, TrySendError},
        Arc, RwLock,
    },
    thread,
};

use crate::{
    event::{entry::EventEntry, filter::Filter, intermediary::IntermediaryEvent, Event},
    subscription::{Subscription, SubscriptionError, SubscriptionSender},
    this_origin,
};

pub trait Id:
    core::fmt::Debug + Default + Clone + Hash + PartialEq + Eq + Send + Sync + 'static
{
}

/// Trait to implement for [`Id`], to control the publisher and all listeners.
pub trait CaptureControl {
    fn start(id: &Self) -> bool;

    fn start_id() -> Self;

    /// Returns `true` if the given [`Id`] is used to signal the end of event capturing.
    ///
    /// **Possible implementation:**
    ///
    /// ```ignore
    /// id == &STOP_CAPTURING_ID
    /// ```
    fn stop(id: &Self) -> bool;

    fn stop_id() -> Self;
}

pub fn is_control_id(id: &impl CaptureControl) -> bool {
    CaptureControl::stop(id) || CaptureControl::start(id)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaptureMode {
    Blocking,
    NonBlocking,
}

type Subscriber<K, T> = HashMap<crate::uuid::Uuid, SubscriptionSender<K, T>>;
type Capturer<K, T> = SyncSender<Event<K, T>>;

pub struct EvidentPublisher<K, T, F>
where
    K: Id + CaptureControl,
    T: EventEntry<K>,
    F: Filter<K, T>,
{
    pub(crate) subscriptions: Arc<RwLock<HashMap<K, Subscriber<K, T>>>>,
    pub(crate) any_event: Arc<RwLock<Subscriber<K, T>>>,
    pub(crate) capturer: Capturer<K, T>,
    filter: Option<F>,
    capturing: Arc<AtomicBool>,
    capture_blocking: Arc<AtomicBool>,
    capture_channel_bound: usize,
    subscription_channel_bound: usize,
    missed_captures: Arc<AtomicUsize>,
}

impl<K, T, F> EvidentPublisher<K, T, F>
where
    K: Id + CaptureControl,
    T: EventEntry<K>,
    F: Filter<K, T>,
{
    fn create(
        mut on_event: impl FnMut(Event<K, T>) + std::marker::Send + 'static,
        filter: Option<F>,
        capture_mode: CaptureMode,
        capture_channel_bound: usize,
        subscription_channel_bound: usize,
    ) -> Self {
        let (send, recv): (SyncSender<Event<K, T>>, _) = mpsc::sync_channel(capture_channel_bound);

        let capturing = Arc::new(AtomicBool::new(true));
        let moved_capturing = capturing.clone();

        thread::spawn(move || {
            let mut channel_closed = false;
            while !channel_closed {
                // Note: Only options for inner loops to exit is either via capturing change, or due to closed channel.
                channel_closed = true;

                if moved_capturing.load(Ordering::Acquire) {
                    while let Ok(mut event) = recv.recv() {
                        let id = event.get_id().clone();
                        event.captured_dt_utc = Some(chrono::Utc::now());

                        on_event(event);

                        // Note: `on_event` must be called before to notify all listeners to stop aswell
                        if CaptureControl::stop(&id) {
                            moved_capturing.store(false, Ordering::Release);
                            // Note: Set to 'false' to indicate that loop did not exit due to closed channel.
                            channel_closed = false;
                            break;
                        }
                    }
                } else {
                    while let Ok(event) = recv.recv() {
                        let id = event.get_id();

                        if CaptureControl::start(id) {
                            // Note: `on_event` must be called to notify all listeners to start aswell
                            on_event(event);

                            moved_capturing.store(true, Ordering::Release);
                            // Note: Set to 'false' to indicate that loop did not exit due to closed channel.
                            channel_closed = false;
                            break;
                        }
                    }
                }
            }
        });

        let mode = match capture_mode {
            CaptureMode::Blocking => Arc::new(AtomicBool::new(true)),
            CaptureMode::NonBlocking => Arc::new(AtomicBool::new(false)),
        };

        EvidentPublisher {
            subscriptions: Arc::new(RwLock::new(HashMap::new())),
            any_event: Arc::new(RwLock::new(HashMap::new())),
            capturer: send,
            filter,
            capturing,
            capture_blocking: mode,
            capture_channel_bound,
            subscription_channel_bound,
            missed_captures: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn new(
        on_event: impl FnMut(Event<K, T>) + std::marker::Send + 'static,
        capture_mode: CaptureMode,
        capture_channel_bound: usize,
        subscription_channel_bound: usize,
    ) -> Self {
        Self::create(
            on_event,
            None,
            capture_mode,
            capture_channel_bound,
            subscription_channel_bound,
        )
    }

    pub fn with(
        on_event: impl FnMut(Event<K, T>) + std::marker::Send + 'static,
        filter: F,
        capture_mode: CaptureMode,
        capture_channel_bound: usize,
        subscription_channel_bound: usize,
    ) -> Self {
        Self::create(
            on_event,
            Some(filter),
            capture_mode,
            capture_channel_bound,
            subscription_channel_bound,
        )
    }

    pub fn get_filter(&self) -> &Option<F> {
        &self.filter
    }

    pub fn capture<I: IntermediaryEvent<K, T>>(&self, interm_event: &mut I) {
        if !is_control_id(interm_event.get_event_id()) {
            if !self.capturing.load(Ordering::Acquire) {
                return;
            }

            if let Some(filter) = &self.filter {
                if !filter.allow_event(interm_event) {
                    return;
                }
            }
        }

        if self.capture_blocking.load(Ordering::Acquire) {
            let _ = self.capturer.send(Event::new(interm_event.take_entry()));
        } else {
            let res = self
                .capturer
                .try_send(Event::new(interm_event.take_entry()));

            if let Err(TrySendError::Full(_)) = res {
                // Note: If another thread has missed captures at the same moment, the count may be inaccurate, because there is no lock.
                // This should still be fine, since
                // - highly unlikely to happen during production with reasonable channel bounds and number of logs captured
                // - count is still increased, and any increase in missed captures is bad (+/- one or two is irrelevant)
                let missed_captures = self.missed_captures.load(Ordering::Relaxed);
                if missed_captures < usize::MAX {
                    self.missed_captures
                        .store(missed_captures + 1, Ordering::Relaxed);
                }
            }
        }
    }

    pub fn get_capture_mode(&self) -> CaptureMode {
        if self.capture_blocking.load(Ordering::Acquire) {
            CaptureMode::Blocking
        } else {
            CaptureMode::NonBlocking
        }
    }

    pub fn set_capture_mode(&self, mode: CaptureMode) {
        match mode {
            CaptureMode::Blocking => self.capture_blocking.store(true, Ordering::Release),
            CaptureMode::NonBlocking => self.capture_blocking.store(false, Ordering::Release),
        }
    }

    pub fn get_missed_captures(&self) -> usize {
        self.missed_captures.load(Ordering::Relaxed)
    }

    pub fn reset_missed_captures(&self) {
        self.missed_captures.store(0, Ordering::Relaxed);
    }

    pub fn subscribe(&self, id: K) -> Result<Subscription<K, T, F>, SubscriptionError<K>> {
        self.subscribe_to_many(vec![id])
    }

    pub fn subscribe_to_many(
        &self,
        ids: Vec<K>,
    ) -> Result<Subscription<K, T, F>, SubscriptionError<K>> {
        // Note: Number of ids to listen to most likely affects the number of received events => number is added to channel bound
        // Addition instead of multiplikation, because even distribution accross events is highly unlikely.
        let (sender, receiver) = mpsc::sync_channel(ids.len() + self.subscription_channel_bound);
        let channel_id = crate::uuid::Uuid::new_v4();
        let subscription_sender = SubscriptionSender { channel_id, sender };

        match self.subscriptions.write().ok() {
            Some(mut locked_subs) => {
                for id in ids.clone() {
                    let entry = locked_subs.entry(id.clone());
                    entry
                        .and_modify(|v| {
                            v.insert(subscription_sender.channel_id, subscription_sender.clone());
                        })
                        .or_insert({
                            let mut h = HashMap::new();
                            h.insert(subscription_sender.channel_id, subscription_sender.clone());
                            h
                        });
                }
            }
            None => {
                return Err(SubscriptionError::CouldNotAccessPublisher);
            }
        }

        Ok(Subscription {
            channel_id,
            receiver,
            sub_to_all: false,
            subscriptions: Some(HashSet::from_iter(ids)),
            publisher: self,
        })
    }

    pub fn subscribe_to_all_events(&self) -> Result<Subscription<K, T, F>, SubscriptionError<K>> {
        let (sender, receiver) = mpsc::sync_channel(self.capture_channel_bound);
        let channel_id = crate::uuid::Uuid::new_v4();

        match self.any_event.write().ok() {
            Some(mut locked_vec) => {
                locked_vec.insert(channel_id, SubscriptionSender { channel_id, sender });
            }
            None => {
                return Err(SubscriptionError::CouldNotAccessPublisher);
            }
        }

        Ok(Subscription {
            channel_id,
            receiver,
            sub_to_all: true,
            subscriptions: None,
            publisher: self,
        })
    }

    pub fn is_capturing(&self) -> bool {
        self.capturing.load(Ordering::Acquire)
    }

    pub fn start_capturing(&self) {
        let start_event = Event::new(EventEntry::new(
            K::start_id(),
            "Starting global capturing.",
            this_origin!(),
        ));

        let _ = self.capturer.send(start_event);
    }

    pub fn stop_capturing(&self) {
        let stop_event = Event::new(EventEntry::new(
            K::stop_id(),
            "Stopping global capturing.",
            this_origin!(),
        ));

        let _ = self.capturer.send(stop_event);
    }

    pub fn on_event(&self, event: Event<K, T>) {
        let arc_event = Arc::new(event);
        let key = arc_event.entry.get_event_id();

        let mut bad_subs: Vec<crate::uuid::Uuid> = Vec::new();
        let mut bad_any_event: Vec<crate::uuid::Uuid> = Vec::new();

        if let Ok(locked_subscriptions) = self.subscriptions.read() {
            if let Some(sub_senders) = locked_subscriptions.get(key) {
                for (channel_id, sub_sender) in sub_senders.iter() {
                    if sub_sender.sender.send(arc_event.clone()).is_err() {
                        bad_subs.push(*channel_id);
                    }
                }
            }
        }

        if let Ok(locked_vec) = self.any_event.read() {
            for (channel_id, any_event_sender) in locked_vec.iter() {
                if any_event_sender.sender.send(arc_event.clone()).is_err() {
                    bad_any_event.push(*channel_id);
                }
            }
        }

        // Remove dead channels
        if !bad_subs.is_empty() {
            if let Ok(mut locked_subscriptions) = self.subscriptions.write() {
                let mut entry = locked_subscriptions.entry(key.clone());
                for i in bad_subs {
                    entry = entry.and_modify(|v| {
                        v.remove(&i);
                    });
                }
            }
        }

        if !bad_any_event.is_empty() {
            if let Ok(mut locked_vec) = self.any_event.write() {
                for i in bad_any_event {
                    locked_vec.remove(&i);
                }
            }
        }
    }
}
