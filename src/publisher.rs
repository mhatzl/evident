use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    sync::{
        mpsc::{self, Receiver, SyncSender},
        Arc, RwLock,
    },
    thread,
};

use crate::{
    event::{entry::EventEntry, intermediary::IntermediaryEvent, Event},
    subscription::{Subscription, SubscriptionErr, SubscriptionSender},
};

pub trait Id:
    core::fmt::Debug + Default + Clone + Hash + PartialEq + Eq + Send + Sync + 'static
{
}

/// Trait to implement for [`Id`], to notify the publisher and all listeners to stop capturing events.
pub trait StopCapturing {
    /// Returns `true` if the given [`Id`] is used to signal the end of event capturing.
    ///
    /// **Possible implementation:**
    ///
    /// ```ignore
    /// if id == &STOP_CAPTURING_ID {
    ///     return true;
    /// }
    ///
    /// false
    /// ```
    fn stop_capturing(id: &Self) -> bool;
}

type Subscriber<K, T> = HashMap<crate::uuid::Uuid, SubscriptionSender<K, T>>;
type Capturer<K, T> = Option<SyncSender<Event<K, T>>>;

pub struct EvidentPublisher<K, T>
where
    K: Id + StopCapturing,
    T: EventEntry<K>,
    SyncSender<Event<K, T>>: Clone,
{
    pub(crate) subscriptions: Arc<RwLock<HashMap<K, Subscriber<K, T>>>>,
    pub(crate) any_event: Arc<RwLock<Subscriber<K, T>>>,
    pub(crate) capturer: Arc<RwLock<Capturer<K, T>>>,
    capture_channel_bound: usize,
    subscription_channel_bound: usize,
}

impl<K, T> EvidentPublisher<K, T>
where
    K: Id + StopCapturing,
    T: EventEntry<K>,
    SyncSender<Event<K, T>>: Clone,
{
    pub fn new(
        mut on_event: impl FnMut(Event<K, T>) + std::marker::Send + 'static,
        capture_channel_bound: usize,
        subscription_channel_bound: usize,
    ) -> Self {
        let (send, recv): (SyncSender<Event<K, T>>, Receiver<Event<K, T>>) =
            mpsc::sync_channel(capture_channel_bound);

        thread::spawn(move || {
            while let Ok(event) = recv.recv() {
                let id = event.get_id().clone();

                on_event(event);

                // Note: `on_event` must still be called to notify all listeners to stop aswell
                if StopCapturing::stop_capturing(&id) {
                    break;
                }
            }
        });

        EvidentPublisher {
            subscriptions: Arc::new(RwLock::new(HashMap::new())),
            any_event: Arc::new(RwLock::new(HashMap::new())),
            capturer: Arc::new(RwLock::new(Some(send))),
            capture_channel_bound,
            subscription_channel_bound,
        }
    }

    pub fn capture<I: IntermediaryEvent<K, T>>(&self, interm_event: &mut I) {
        if let Ok(locked_cap) = self.capturer.try_read() {
            if locked_cap.is_some() {
                let _ = locked_cap
                    .as_ref()
                    .unwrap()
                    .send(Event::new(interm_event.take_entry()));
            }
        }
    }

    pub fn try_capture<I: IntermediaryEvent<K, T>>(&self, interm_event: &mut I) {
        if let Ok(locked_cap) = self.capturer.try_read() {
            if locked_cap.is_some() {
                let _ = locked_cap
                    .as_ref()
                    .unwrap()
                    .try_send(Event::new(interm_event.take_entry()));
            }
        }
    }

    pub fn subscribe(&self, id: K) -> Result<Subscription<K, T>, SubscriptionErr<K>> {
        self.subscribe_to_many(vec![id])
    }

    pub fn subscribe_to_many(&self, ids: Vec<K>) -> Result<Subscription<K, T>, SubscriptionErr<K>> {
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
                return Err(SubscriptionErr::CouldNotAccessPublisher);
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

    pub fn subscribe_to_all_events(&self) -> Result<Subscription<K, T>, SubscriptionErr<K>> {
        let (sender, receiver) = mpsc::sync_channel(self.capture_channel_bound);
        let channel_id = crate::uuid::Uuid::new_v4();

        match self.any_event.write().ok() {
            Some(mut locked_vec) => {
                locked_vec.insert(channel_id, SubscriptionSender { channel_id, sender });
            }
            None => {
                return Err(SubscriptionErr::CouldNotAccessPublisher);
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

    pub fn shutdown(&self) {
        if let Ok(mut locked_subscriptions) = self.subscriptions.write() {
            locked_subscriptions.drain();
        }

        if let Ok(mut locked_vec) = self.any_event.write() {
            locked_vec.drain();
        }

        if let Ok(mut locked_cap) = self.capturer.write() {
            *locked_cap = None;
        }
    }

    pub fn on_event(&self, event: Event<K, T>) {
        let key = event.entry.get_event_id();

        let mut bad_subs: Vec<crate::uuid::Uuid> = Vec::new();
        let mut bad_any_event: Vec<crate::uuid::Uuid> = Vec::new();

        if let Ok(locked_subscriptions) = self.subscriptions.read() {
            if let Some(sub_senders) = locked_subscriptions.get(key) {
                for (channel_id, sub_sender) in sub_senders.iter() {
                    if sub_sender.sender.send(event.clone()).is_err() {
                        bad_subs.push(*channel_id);
                    }
                }
            }
        }

        if let Ok(locked_vec) = self.any_event.read() {
            for (channel_id, any_event_sender) in locked_vec.iter() {
                if any_event_sender.sender.send(event.clone()).is_err() {
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
