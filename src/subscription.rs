use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    sync::mpsc::{Receiver, SyncSender},
};

use crate::{
    event::{entry::EventEntry, Event},
    publisher::{EvidentPublisher, Id},
};

pub struct Subscription<
    'p,
    K,
    T,
    const CAPTURE_CHANNEL_BOUND: usize,
    const SUBSCRIPTION_CHANNEL_BOUND: usize,
> where
    K: Id,
    T: EventEntry<K>,
{
    pub(crate) channel_id: crate::uuid::Uuid,
    pub(crate) receiver: Receiver<Event<K, T>>,
    pub(crate) sub_to_all: bool,
    pub(crate) subscriptions: Option<HashSet<K>>,
    pub(crate) publisher:
        &'p EvidentPublisher<K, T, { CAPTURE_CHANNEL_BOUND }, { SUBSCRIPTION_CHANNEL_BOUND }>,
}

impl<'p, K, T, const CAPTURE_CHANNEL_BOUND: usize, const SUBSCRIPTION_CHANNEL_BOUND: usize>
    Subscription<'p, K, T, CAPTURE_CHANNEL_BOUND, SUBSCRIPTION_CHANNEL_BOUND>
where
    K: Id,
    T: EventEntry<K>,
{
    pub fn get_receiver(&self) -> &Receiver<Event<K, T>> {
        &self.receiver
    }

    pub fn unsubscribe(self) {
        drop(self)
    }

    pub fn unsubscribe_id(&mut self, id: K) -> Result<(), SubscriptionErr<K>> {
        self.unsubscribe_many(vec![id])
    }

    pub fn unsubscribe_many(&mut self, ids: Vec<K>) -> Result<(), SubscriptionErr<K>> {
        if self.sub_to_all || self.subscriptions.is_none() {
            return Err(SubscriptionErr::AllEventsSubscriptionNotModifiable);
        }

        let subs = self.subscriptions.as_mut().unwrap();

        if ids.len() >= subs.len() {
            return Err(SubscriptionErr::UnsubscribeWouldDeleteSubscription);
        }

        for id in ids.clone() {
            if !subs.contains(&id) {
                return Err(SubscriptionErr::IdNotSubscribed(id));
            }
        }

        match self.publisher.subscriptions.write() {
            Ok(mut publisher_subs) => {
                for id in ids {
                    if let Some(id_sub) = publisher_subs.get_mut(&id) {
                        let _ = id_sub.remove(&self.channel_id);
                    }
                    subs.remove(&id);
                }

                Ok(())
            }
            Err(_) => Err(SubscriptionErr::CouldNotAccessPublisher),
        }
    }

    pub fn subscribe_id(&mut self, id: K) -> Result<(), SubscriptionErr<K>> {
        self.subscribe_many(vec![id])
    }

    pub fn subscribe_many(&mut self, ids: Vec<K>) -> Result<(), SubscriptionErr<K>> {
        if self.sub_to_all || self.subscriptions.is_none() {
            return Err(SubscriptionErr::AllEventsSubscriptionNotModifiable);
        }

        let subs = self.subscriptions.as_mut().unwrap();

        for id in ids.clone() {
            if subs.contains(&id) {
                return Err(SubscriptionErr::IdAlreadySubscribed(id));
            }
        }
        let any_sub_id = match subs.iter().next() {
            Some(id) => id,
            None => {
                return Err(SubscriptionErr::NoSubscriptionChannelAvailable);
            }
        };

        let sender = match self.publisher.subscriptions.read() {
            Ok(publisher_subs) => match publisher_subs.get(any_sub_id) {
                Some(id_subs) => match id_subs.get(&self.channel_id) {
                    Some(sub_sender) => sub_sender.clone(),
                    None => {
                        return Err(SubscriptionErr::NoSubscriptionChannelAvailable);
                    }
                },
                None => {
                    return Err(SubscriptionErr::NoSubscriptionChannelAvailable);
                }
            },
            Err(_) => {
                return Err(SubscriptionErr::CouldNotAccessPublisher);
            }
        };

        match self.publisher.subscriptions.write() {
            Ok(mut publisher_subs) => {
                for id in ids {
                    publisher_subs
                        .entry(id.clone())
                        .and_modify(|id_subs| {
                            id_subs.insert(self.channel_id, sender.clone());
                        })
                        .or_insert_with(|| {
                            let mut map = HashMap::new();
                            map.insert(self.channel_id, sender.clone());
                            map
                        });

                    subs.insert(id);
                }

                Ok(())
            }
            Err(_) => Err(SubscriptionErr::CouldNotAccessPublisher),
        }
    }
}

impl<'p, K, T, const CAPTURE_CHANNEL_BOUND: usize, const SUBSCRIPTION_CHANNEL_BOUND: usize> Drop
    for Subscription<'p, K, T, CAPTURE_CHANNEL_BOUND, SUBSCRIPTION_CHANNEL_BOUND>
where
    K: Id,
    T: EventEntry<K>,
{
    fn drop(&mut self) {
        // Note: We do not want to block the current thread for *unsubscribing*, since publisher also maintains dead channels.
        if self.sub_to_all {
            if let Ok(mut locked_any_event) = self.publisher.any_event.try_write() {
                let _ = locked_any_event.remove(&self.channel_id);
            }
        } else if let Some(self_subs) = &self.subscriptions {
            if let Ok(mut publisher_subs) = self.publisher.subscriptions.try_write() {
                for k in self_subs {
                    if let Some(id_sub) = publisher_subs.get_mut(k) {
                        let _ = id_sub.remove(&self.channel_id);
                    }
                }
            }
        }
    }
}

impl<'p, K, T, const CAPTURE_CHANNEL_BOUND: usize, const SUBSCRIPTION_CHANNEL_BOUND: usize>
    PartialEq for Subscription<'p, K, T, CAPTURE_CHANNEL_BOUND, SUBSCRIPTION_CHANNEL_BOUND>
where
    K: Id,
    T: EventEntry<K>,
{
    fn eq(&self, other: &Self) -> bool {
        self.channel_id == other.channel_id
    }
}

impl<'p, K, T, const CAPTURE_CHANNEL_BOUND: usize, const SUBSCRIPTION_CHANNEL_BOUND: usize> Eq
    for Subscription<'p, K, T, CAPTURE_CHANNEL_BOUND, SUBSCRIPTION_CHANNEL_BOUND>
where
    K: Id,
    T: EventEntry<K>,
{
}

impl<'p, K, T, const CAPTURE_CHANNEL_BOUND: usize, const SUBSCRIPTION_CHANNEL_BOUND: usize> Hash
    for Subscription<'p, K, T, CAPTURE_CHANNEL_BOUND, SUBSCRIPTION_CHANNEL_BOUND>
where
    K: Id,
    T: EventEntry<K>,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.channel_id.hash(state);
    }
}

#[derive(Debug, Clone)]
pub enum SubscriptionErr<K: Id> {
    AllEventsSubscriptionNotModifiable,
    IdNotSubscribed(K),
    IdAlreadySubscribed(K),
    UnsubscribeWouldDeleteSubscription,
    CouldNotAccessPublisher,
    NoSubscriptionChannelAvailable,
}

#[derive(Clone)]
pub(crate) struct SubscriptionSender<K, T>
where
    K: Id,
    T: EventEntry<K>,
    SyncSender<Event<K, T>>: Clone,
{
    pub(crate) channel_id: crate::uuid::Uuid,
    pub(crate) sender: SyncSender<Event<K, T>>,
}

impl<K, T> PartialEq for SubscriptionSender<K, T>
where
    K: Id,
    T: EventEntry<K>,
    SyncSender<Event<K, T>>: Clone,
{
    fn eq(&self, other: &Self) -> bool {
        self.channel_id == other.channel_id
    }
}

impl<K, T> Eq for SubscriptionSender<K, T>
where
    K: Id,
    T: EventEntry<K>,
    SyncSender<Event<K, T>>: Clone,
{
}

impl<K, T> Hash for SubscriptionSender<K, T>
where
    K: Id,
    T: EventEntry<K>,
    SyncSender<Event<K, T>>: Clone,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.channel_id.hash(state);
    }
}
