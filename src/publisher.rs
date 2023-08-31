use std::{
    collections::{HashMap, HashSet},
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        mpsc::{self, SyncSender, TrySendError},
        Arc, RwLock,
    },
    thread,
};

use crate::{
    event::{entry::EventEntry, filter::Filter, intermediary::IntermediaryEvent, Event, Id, Msg},
    subscription::{Subscription, SubscriptionError, SubscriptionSender},
    this_origin,
};

/// Trait to implement for [`Id`], to control the publisher and all listeners.
///
/// [req:cap.ctrl](https://github.com/mhatzl/evident/wiki/5-REQ-cap.ctrl)
pub trait CaptureControl {
    /// Returns `true` if the given [`Id`] is used to signal the start of event capturing.
    ///
    /// **Possible implementation:**
    ///
    /// ```ignore
    /// id == &START_CAPTURING_ID
    /// ```
    ///
    /// [req:cap.ctrl.start](https://github.com/mhatzl/evident/wiki/5-REQ-cap.ctrl.start#capctrlstart-start-capturing)
    fn start(id: &Self) -> bool;

    /// Returns the *start-ID*.
    ///
    /// [req:cap.ctrl.start](https://github.com/mhatzl/evident/wiki/5-REQ-cap.ctrl.start#capctrlstart-start-capturing)
    fn start_id() -> Self;

    /// Returns `true` if the given [`Id`] is used to signal the end of event capturing.
    ///
    /// **Possible implementation:**
    ///
    /// ```ignore
    /// id == &STOP_CAPTURING_ID
    /// ```
    ///
    /// [req:cap.ctrl.stop](https://github.com/mhatzl/evident/wiki/5-REQ-cap.ctrl.stop#capctrlstop-stop-capturing)
    fn stop(id: &Self) -> bool;

    /// Returns the *stop-ID*.
    ///
    /// [req:cap.ctrl.stop](https://github.com/mhatzl/evident/wiki/5-REQ-cap.ctrl.stop#capctrlstop-stop-capturing)
    fn stop_id() -> Self;
}

/// Returns `true` if the given [`Id`] is used to control capturing.
///
/// [req:cap.ctrl](https://github.com/mhatzl/evident/wiki/5-REQ-cap.ctrl#capctrl-control-capturing)
pub fn is_control_id(id: &impl CaptureControl) -> bool {
    CaptureControl::stop(id) || CaptureControl::start(id)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaptureMode {
    Blocking,
    NonBlocking,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventTimestampKind {
    /// Sets the event time, when the event is captured.
    ///
    /// **Note:** With this setting, event timestamps might show incorrect order in case of concurrent events, because events are buffered before capturing.
    ///
    /// **Note:** This has slightly better performance on the thread setting an event, because system time access is delayed to the capturing thread.
    Captured,
    /// Sets the event time, when the event is created.
    ///
    /// **Note:** This has slightly worse performance on the thread setting an event, because system time access most likely requires a context switch.
    Created,
}

// Types below used for better clarity according to clippy.

type Subscriber<K, M, T> = HashMap<crate::uuid::Uuid, SubscriptionSender<K, M, T>>;
type IdSubscriber<K, M, T> = HashMap<K, Subscriber<K, M, T>>;
type Capturer<K, M, T> = SyncSender<Event<K, M, T>>;

/// An **EvidentPublisher** is used to capture, publish, and manage subscriptions.
///
/// [req:pub](https://github.com/mhatzl/evident/wiki/5-REQ-pub#pub-event-publishing)
pub struct EvidentPublisher<K, M, T, F>
where
    K: Id + CaptureControl,
    M: Msg,
    T: EventEntry<K, M>,
    F: Filter<K, M>,
{
    /// The hashmap of subscribers listening to specific events.
    ///
    /// [req:subs.specific](https://github.com/mhatzl/evident/wiki/5-REQ-subs.specific#subsspecific-subscribe-to-specific-events)
    pub(crate) subscriptions: Arc<RwLock<IdSubscriber<K, M, T>>>,

    /// The hashmap of subscribers listening to all events.
    ///
    /// [req:subs.all](https://github.com/mhatzl/evident/wiki/5-REQ-subs.all#subsall-subscribe-to-all-events)
    pub(crate) any_event: Arc<RwLock<Subscriber<K, M, T>>>,

    /// The send-part of the capturing channel.
    ///
    /// [req:cap](https://github.com/mhatzl/evident/wiki/5-REQ-cap#cap-capturing-events)
    pub(crate) capturer: Capturer<K, M, T>,

    /// Optional filter that is applied when capturing events.
    ///
    /// [req:cap.filter](https://github.com/mhatzl/evident/wiki/5-REQ-cap.filter#capfilter-filter-captured-events)
    filter: Option<F>,

    /// Flag to control if capturing is active or inactive.
    ///
    /// [req:cap.ctrl](https://github.com/mhatzl/evident/wiki/5-REQ-cap.ctrl#capctrl-control-capturing)
    capturing: Arc<AtomicBool>,

    /// Flag to control the capture mode.
    capture_blocking: Arc<AtomicBool>,

    /// Defines the size of the capturing send-buffer.
    ///
    /// [req:cap](https://github.com/mhatzl/evident/wiki/5-REQ-cap#cap-capturing-events)
    capture_channel_bound: usize,

    /// Defines the size of each subscription send-buffer.
    ///
    /// [req:subs](https://github.com/mhatzl/evident/wiki/5-REQ-subs#subs-subscribing-to-events)
    subscription_channel_bound: usize,

    /// Number of missed captures in *non-blocking* capture mode.
    missed_captures: Arc<AtomicUsize>,

    /// Defines at what point the event-timestamp is created.
    timestamp_kind: EventTimestampKind,
}

impl<K, M, T, F> EvidentPublisher<K, M, T, F>
where
    K: Id + CaptureControl,
    M: Msg,
    T: EventEntry<K, M>,
    F: Filter<K, M>,
{
    /// Create a new [`EvidentPublisher`], and spawn a new event handler thread for events captured by the publisher.
    ///
    /// **Note:** You should use the macro [`create_static_publisher`](crate::create_static_publisher) instead.
    ///
    /// [req:pub](https://github.com/mhatzl/evident/wiki/5-REQ-pub#pub-event-publishing)
    fn create(
        mut on_event: impl FnMut(Event<K, M, T>) + std::marker::Send + 'static,
        filter: Option<F>,
        capture_mode: CaptureMode,
        capture_channel_bound: usize,
        subscription_channel_bound: usize,
        timestamp_kind: EventTimestampKind,
    ) -> Self {
        let (send, recv): (SyncSender<Event<K, M, T>>, _) =
            mpsc::sync_channel(capture_channel_bound);

        // [req:pub.threaded](https://github.com/mhatzl/evident/wiki/5-REQ-pub.threaded#pubthreaded-multithreaded-publishing)
        thread::spawn(move || {
            while let Ok(mut event) = recv.recv() {
                if timestamp_kind == EventTimestampKind::Captured {
                    event.timestamp = Some(std::time::SystemTime::now());
                }

                on_event(event);
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
            // [req:cap.ctrl.init](https://github.com/mhatzl/evident/wiki/5-REQ-cap.ctrl.init)
            capturing: Arc::new(AtomicBool::new(true)),
            capture_blocking: mode,
            capture_channel_bound,
            subscription_channel_bound,
            missed_captures: Arc::new(AtomicUsize::new(0)),
            timestamp_kind,
        }
    }

    /// Create a new [`EvidentPublisher`] without an event filter.
    ///
    /// **Note:** You should use the macro [`create_static_publisher`](crate::create_static_publisher) instead.
    ///
    /// [req:pub](https://github.com/mhatzl/evident/wiki/5-REQ-pub#pub-event-publishing)
    pub fn new(
        on_event: impl FnMut(Event<K, M, T>) + std::marker::Send + 'static,
        capture_mode: CaptureMode,
        capture_channel_bound: usize,
        subscription_channel_bound: usize,
        time_stamp_kind: EventTimestampKind,
    ) -> Self {
        Self::create(
            on_event,
            None,
            capture_mode,
            capture_channel_bound,
            subscription_channel_bound,
            time_stamp_kind,
        )
    }

    /// Create a new [`EvidentPublisher`] with an event filter.
    ///
    /// **Note:** You should use the macro [`create_static_publisher`](crate::create_static_publisher) instead.
    ///
    /// [req:pub](https://github.com/mhatzl/evident/wiki/5-REQ-pub#pub-event-publishing), [req:cap.filter](https://github.com/mhatzl/evident/wiki/5-REQ-cap.filter#capfilter-filter-captured-events)
    pub fn with(
        on_event: impl FnMut(Event<K, M, T>) + std::marker::Send + 'static,
        filter: F,
        capture_mode: CaptureMode,
        capture_channel_bound: usize,
        subscription_channel_bound: usize,
        timestamp_kind: EventTimestampKind,
    ) -> Self {
        Self::create(
            on_event,
            Some(filter),
            capture_mode,
            capture_channel_bound,
            subscription_channel_bound,
            timestamp_kind,
        )
    }

    /// Returns the event filter, or `None` if no filter is set.
    ///
    /// [req:cap.filter](https://github.com/mhatzl/evident/wiki/5-REQ-cap.filter#capfilter-filter-captured-events)
    pub fn get_filter(&self) -> &Option<F> {
        &self.filter
    }

    /// Returns `true` if the given event-entry passes the filter, or the event-ID is a control-ID.
    ///
    /// [req:cap.filter](https://github.com/mhatzl/evident/wiki/5-REQ-cap.filter#capfilter-filter-captured-events)
    pub fn entry_allowed(&self, entry: &impl EventEntry<K, M>) -> bool {
        if !is_control_id(entry.get_event_id()) {
            if !self.capturing.load(Ordering::Acquire) {
                return false;
            }

            if let Some(filter) = &self.filter {
                if !filter.allow_entry(entry) {
                    return false;
                }
            }
        }

        true
    }

    /// Captures an intermediary event, and sends the resulting event to the event handler.
    ///
    /// **Note:** This function should **not** be called manually, because it is automatically called on `drop()` of an intermediary event.
    ///
    /// [req:cap](https://github.com/mhatzl/evident/wiki/5-REQ-cap#cap-capturing-events)
    #[doc(hidden)]
    pub fn _capture<I: IntermediaryEvent<K, M, T>>(&self, interm_event: &mut I) {
        let entry = interm_event.take_entry();

        // [req:cap.filter](https://github.com/mhatzl/evident/wiki/5-REQ-cap.filter#capfilter-filter-captured-events)
        if !self.entry_allowed(&entry) {
            return;
        }

        let mut event = Event::new(entry);
        if self.timestamp_kind == EventTimestampKind::Created {
            event.timestamp = Some(std::time::SystemTime::now());
        }

        if self.capture_blocking.load(Ordering::Acquire) {
            let _ = self.capturer.send(event);
        } else {
            let res = self.capturer.try_send(event);

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

    /// Returns the current capture mode.
    pub fn get_capture_mode(&self) -> CaptureMode {
        if self.capture_blocking.load(Ordering::Acquire) {
            CaptureMode::Blocking
        } else {
            CaptureMode::NonBlocking
        }
    }

    /// Allows to change the capture mode.
    pub fn set_capture_mode(&self, mode: CaptureMode) {
        match mode {
            CaptureMode::Blocking => self.capture_blocking.store(true, Ordering::Release),
            CaptureMode::NonBlocking => self.capture_blocking.store(false, Ordering::Release),
        }
    }

    /// Returns the number of missed captures in *non-blocking* mode since last reset.
    pub fn get_missed_captures(&self) -> usize {
        self.missed_captures.load(Ordering::Relaxed)
    }

    /// Resets the number of missed captures in *non-blocking* mode.
    pub fn reset_missed_captures(&self) {
        self.missed_captures.store(0, Ordering::Relaxed);
    }

    /// Returns a subscription to events with the given event-ID,
    /// or a [`SubscriptionError<K>`] if the subscription could not be created.
    ///
    /// [req:subs.specific.one](https://github.com/mhatzl/evident/wiki/5-REQ-subs.specific.one#subsspecificone-subscribe-to-one-specific-event)
    pub fn subscribe(&self, id: K) -> Result<Subscription<K, M, T, F>, SubscriptionError<K>> {
        self.subscribe_to_many(vec![id])
    }

    /// Returns a subscription to events with the given event-IDs,
    /// or a [`SubscriptionError<K>`] if the subscription could not be created.
    ///
    /// [req:subs.specific.mult](https://github.com/mhatzl/evident/wiki/5-REQ-subs.specific.mult#subsspecificmult-subscribe-to-multiple-specific-events)
    pub fn subscribe_to_many(
        &self,
        ids: Vec<K>,
    ) -> Result<Subscription<K, M, T, F>, SubscriptionError<K>> {
        // Note: Number of ids to listen to most likely affects the number of received events => number is added to channel bound
        // Addition instead of multiplication, because even distribution accross events is highly unlikely.
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

    /// Returns a subscription to all events,
    /// or a [`SubscriptionError<K>`] if the subscription could not be created.
    ///
    /// [req:subs.all](https://github.com/mhatzl/evident/wiki/5-REQ-subs.all#subsall-subscribe-to-all-events)
    pub fn subscribe_to_all_events(
        &self,
    ) -> Result<Subscription<K, M, T, F>, SubscriptionError<K>> {
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

    /// Returns `true` if capturing is *active*.
    ///
    /// [req:cap.ctrl.info](https://github.com/mhatzl/evident/wiki/5-REQ-cap.ctrl.info#capctrlinfo-get-capturing-state)
    pub fn is_capturing(&self) -> bool {
        self.capturing.load(Ordering::Acquire)
    }

    /// Start capturing.
    ///
    /// **Note:** Capturing is already started initially, so this function is only needed after manually stopping capturing.
    ///
    /// [req:cap.ctrl.start](https://github.com/mhatzl/evident/wiki/5-REQ-cap.ctrl.start#capctrlstart-start-capturing)
    pub fn start(&self) {
        let empty_msg: Option<M> = None;
        let start_event = Event::new(EventEntry::new(K::start_id(), empty_msg, this_origin!()));

        let _ = self.capturer.send(start_event);

        self.capturing.store(true, Ordering::Release);
    }

    /// Stop capturing.
    ///
    /// [req:cap.ctrl.stop](https://github.com/mhatzl/evident/wiki/5-REQ-cap.ctrl.stop#capctrlstop-stop-capturing)
    pub fn stop(&self) {
        let empty_msg: Option<M> = None;
        let stop_event = Event::new(EventEntry::new(K::stop_id(), empty_msg, this_origin!()));

        let _ = self.capturer.send(stop_event);

        self.capturing.store(false, Ordering::Release);
    }

    /// Send the given event to all subscriber of the event.
    ///
    /// **Note:** This function should **not** be called manually, because it is already called in the event handler.
    ///
    /// [req:cap](https://github.com/mhatzl/evident/wiki/5-REQ-cap#cap-capturing-events)
    #[doc(hidden)]
    pub fn on_event(&self, event: Event<K, M, T>) {
        let arc_event = Arc::new(event);
        let key = arc_event.entry.get_event_id();

        let mut bad_subs: Vec<crate::uuid::Uuid> = Vec::new();
        let mut bad_any_event: Vec<crate::uuid::Uuid> = Vec::new();

        if let Ok(locked_subscriptions) = self.subscriptions.read() {
            if let Some(sub_senders) = locked_subscriptions.get(key) {
                for (channel_id, sub_sender) in sub_senders.iter() {
                    let bad_channel = if self.capture_blocking.load(Ordering::Acquire) {
                        sub_sender.sender.send(arc_event.clone()).is_err()
                    } else {
                        matches!(
                            sub_sender.sender.try_send(arc_event.clone()),
                            Err(TrySendError::Disconnected(_))
                        )
                    };

                    if bad_channel {
                        bad_subs.push(*channel_id);
                    }
                }
            }
        }

        if let Ok(locked_vec) = self.any_event.read() {
            for (channel_id, any_event_sender) in locked_vec.iter() {
                let bad_channel = if self.capture_blocking.load(Ordering::Acquire) {
                    any_event_sender.sender.send(arc_event.clone()).is_err()
                } else {
                    matches!(
                        any_event_sender.sender.try_send(arc_event.clone()),
                        Err(TrySendError::Disconnected(_))
                    )
                };

                if bad_channel {
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
