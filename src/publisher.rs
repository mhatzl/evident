use std::{
    collections::HashMap,
    hash::Hash,
    sync::{
        mpsc::{self, Receiver, SyncSender},
        Arc, RwLock,
    },
    thread,
};

use crate::event::{entry::EventEntry, intermediary::IntermediaryEvent, Event};

pub trait Id: Default + Clone + Hash + PartialEq + Eq {}

pub(crate) type Subscriber<K, T> = HashMap<uuid::Uuid, SubscriptionSender<K, T>>;

#[derive(Default, Clone)]
pub struct Subscription<K, T>
where
    K: Id,
    T: EventEntry<K>,
    Receiver<Event<K, T>>: Default + Clone,
{
    pub(crate) channel_id: uuid::Uuid,
    pub(crate) receiver: Receiver<Event<K, T>>,
    pub(crate) sub_to_all: bool,
}

impl<K, T> PartialEq for Subscription<K, T>
where
    K: Id,
    T: EventEntry<K>,
    Receiver<Event<K, T>>: Default + Clone,
{
    fn eq(&self, other: &Self) -> bool {
        self.channel_id == other.channel_id
    }
}

impl<K, T> Eq for Subscription<K, T>
where
    K: Id,
    T: EventEntry<K>,
    Receiver<Event<K, T>>: Default + Clone,
{
}

impl<K, T> Subscription<K, T>
where
    K: Id,
    T: EventEntry<K>,
    Receiver<Event<K, T>>: Default + Clone,
{
    pub fn get_receiver(&self) -> &Receiver<Event<K, T>> {
        &self.receiver
    }
}

impl<K, T> Hash for Subscription<K, T>
where
    K: Id,
    T: EventEntry<K>,
    Receiver<Event<K, T>>: Default + Clone,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.channel_id.hash(state);
    }
}

#[derive(Default, Clone)]
pub(crate) struct SubscriptionSender<K, T>
where
    K: Id,
    T: EventEntry<K>,
    SyncSender<Event<K, T>>: Default + Clone,
{
    pub(crate) channel_id: uuid::Uuid,
    pub(crate) sender: SyncSender<Event<K, T>>,
}

impl<K, T> PartialEq for SubscriptionSender<K, T>
where
    K: Id,
    T: EventEntry<K>,
    SyncSender<Event<K, T>>: Default + Clone,
{
    fn eq(&self, other: &Self) -> bool {
        self.channel_id == other.channel_id
    }
}

impl<K, T> Eq for SubscriptionSender<K, T>
where
    K: Id,
    T: EventEntry<K>,
    SyncSender<Event<K, T>>: Default + Clone,
{
}

impl<K, T> Hash for SubscriptionSender<K, T>
where
    K: Id,
    T: EventEntry<K>,
    SyncSender<Event<K, T>>: Default + Clone,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.channel_id.hash(state);
    }
}

pub struct EvidentPublisher<
    K: Id,
    T: EventEntry<K>,
    const CAPTURE_CHANNEL_BOUND: usize,
    const SUBSCRIPTION_CHANNEL_BOUND: usize,
> where
    K: Id,
    T: EventEntry<K>,
    Event<K, T>: EventEntry<K>,
    Receiver<Event<K, T>>: Default + Clone,
    SyncSender<Event<K, T>>: Default + Clone,
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
    Event<K, T>: EventEntry<K>,
    Receiver<Event<K, T>>: Default + Clone,
    SyncSender<Event<K, T>>: Default + Clone,
{
    pub fn new(on_event: fn(event: Event<K, T>)) -> Self {
        let (send, recv) = mpsc::sync_channel(CAPTURE_CHANNEL_BOUND);

        thread::spawn(move || loop {
            match recv.recv() {
                Ok(event_msg) => {
                    on_event(event_msg);
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

    pub fn capture(&self, interm_event: &mut IntermediaryEvent<K, T>) {
        let _ = self
            .capturer
            .send(Event::new(std::mem::take(&mut interm_event.entry)));
    }

    pub fn try_capture(&self, interm_event: &mut IntermediaryEvent<K, T>) {
        let _ = self
            .capturer
            .try_send(Event::new(std::mem::take(&mut interm_event.entry)));
    }

    pub fn subscribe(&self, id: K) -> Option<Subscription<K, T>> {
        self.subscribe_to_many(&vec![id])
    }

    pub fn subscribe_to_many(&self, ids: &Vec<K>) -> Option<Subscription<K, T>> {
        // Note: Number of ids to listen to most likely affects the number of received events => number is added to channel bound
        // Addition instead of multiplikation, because even distribution accross events is highly unlikely.
        let (sender, receiver) = mpsc::sync_channel(ids.len() + SUBSCRIPTION_CHANNEL_BOUND);
        let channel_id = uuid::Uuid::new_v4();
        let subscription_sender = SubscriptionSender { channel_id, sender };

        match self.subscriptions.write().ok() {
            Some(mut locked_subs) => {
                for id in ids {
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
            if let Some(sub_senders) = locked_subscriptions.get(&key) {
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
                let mut entry = locked_subscriptions.entry(key);
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

#[macro_export]
macro_rules! create_on_event {
    ($publisher:ident, $entry_type:ty) => {
        fn on_event(event: $crate::event::Event<$entry_type>) {
            publisher.on_event(event);
        }
    };
}

// #[macro_export]
// macro_rules! subscribe {
//     ($logid:ident) => {
//         $crate::publisher::subscribe($crate::logid!($logid), env!("CARGO_PKG_NAME"))
//     };
//     ($logid:expr) => {
//         $crate::publisher::subscribe($crate::logid!($logid), env!("CARGO_PKG_NAME"))
//     };
// }

// pub fn subscribe_to_logs<T>(log_ids: T, crate_name: &'static str) -> Option<Receiver<Event>>
// where
//     T: Iterator<Item = LogId>,
// {
//     let crate_logs = vec![(crate_name, log_ids.collect())];
//     subscribe_to_crates(&crate_logs)
// }

// #[macro_export]
// macro_rules! subscribe_to_logs {
//     ($logids:ident) => {
//         $crate::publisher::subscribe_to_logs($crate::logids!($logids), env!("CARGO_PKG_NAME"))
//     };
//     ($logids:expr) => {
//         $crate::publisher::subscribe_to_logs($crate::logids!($logids), env!("CARGO_PKG_NAME"))
//     };
// }
