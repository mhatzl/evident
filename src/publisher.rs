use std::{
    collections::HashMap,
    hash::Hash,
    sync::{
        mpsc::{self, SyncSender},
        Arc, RwLock,
    },
    thread,
};

use crate::{
    event::{entry::EventEntry, intermediary::IntermediaryEvent, Event},
    subscription::{Subscription, SubscriptionSender},
};

pub trait Id: Default + Clone + Hash + PartialEq + Eq + Send + Sync + 'static {}

type Subscriber<K, T> = HashMap<uuid::Uuid, SubscriptionSender<K, T>>;

pub struct EvidentPublisher<
    K,
    T,
    const CAPTURE_CHANNEL_BOUND: usize,
    const SUBSCRIPTION_CHANNEL_BOUND: usize,
> where
    K: Id,
    T: EventEntry<K>,
    SyncSender<Event<K, T>>: Clone,
{
    pub(crate) subscriptions: Arc<RwLock<HashMap<K, Subscriber<K, T>>>>,
    pub(crate) any_event: Arc<RwLock<Subscriber<K, T>>>,
    pub(crate) capturer: SyncSender<Event<K, T>>,
}

impl<K, T, const CAPTURE_CHANNEL_BOUND: usize, const SUBSCRIPTION_CHANNEL_BOUND: usize>
    EvidentPublisher<K, T, CAPTURE_CHANNEL_BOUND, SUBSCRIPTION_CHANNEL_BOUND>
where
    K: Id,
    T: EventEntry<K>,
    SyncSender<Event<K, T>>: Clone,
{
    pub fn new(mut on_event: impl FnMut(Event<K, T>) + std::marker::Send + 'static) -> Self {
        let (send, recv) = mpsc::sync_channel(CAPTURE_CHANNEL_BOUND);

        thread::spawn(move || loop {
            match recv.recv() {
                Ok(event) => {
                    on_event(event);
                }
                Err(_) => {
                    // Sender got dropped => Publisher got dropped
                    return;
                }
            }
        });

        EvidentPublisher {
            subscriptions: Arc::new(RwLock::new(HashMap::new())),
            any_event: Arc::new(RwLock::new(HashMap::new())),
            capturer: send,
        }
    }

    pub fn capture<I: IntermediaryEvent<K, T>>(&self, interm_event: &mut I) {
        let _ = self.capturer.send(Event::new(interm_event.take_entry()));
    }

    pub fn try_capture<I: IntermediaryEvent<K, T>>(&self, interm_event: &mut I) {
        let _ = self
            .capturer
            .try_send(Event::new(interm_event.take_entry()));
    }

    pub fn subscribe(&self, id: &K) -> Option<Subscription<K, T>> {
        self.subscribe_to_many(&vec![id])
    }

    pub fn subscribe_to_many(&self, ids: &Vec<&K>) -> Option<Subscription<K, T>> {
        // Note: Number of ids to listen to most likely affects the number of received events => number is added to channel bound
        // Addition instead of multiplikation, because even distribution accross events is highly unlikely.
        let (sender, receiver) = mpsc::sync_channel(ids.len() + SUBSCRIPTION_CHANNEL_BOUND);
        let channel_id = uuid::Uuid::new_v4();
        let subscription_sender = SubscriptionSender { channel_id, sender };

        match self.subscriptions.write().ok() {
            Some(mut locked_subs) => {
                for id in ids {
                    let entry = locked_subs.entry((*id).clone());
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
                return None;
            }
        }

        Some(Subscription {
            channel_id,
            receiver,
            sub_to_all: false,
        })
    }

    pub fn subscribe_to_all_events(&self) -> Option<Subscription<K, T>> {
        let (sender, receiver) = mpsc::sync_channel(CAPTURE_CHANNEL_BOUND);
        let channel_id = uuid::Uuid::new_v4();

        match self.any_event.write().ok() {
            Some(mut locked_vec) => {
                locked_vec.insert(channel_id, SubscriptionSender { channel_id, sender });
            }
            None => {
                return None;
            }
        }

        Some(Subscription {
            channel_id,
            receiver,
            sub_to_all: true,
        })
    }

    pub fn on_event(&self, event: Event<K, T>) {
        let key = event.entry.get_event_id();

        let mut bad_subs: Vec<uuid::Uuid> = Vec::new();
        let mut bad_any_event: Vec<uuid::Uuid> = Vec::new();

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
