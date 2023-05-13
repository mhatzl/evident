use std::{
    hash::Hash,
    sync::mpsc::{Receiver, SyncSender},
};

use crate::{
    event::{entry::EventEntry, Event},
    publisher::Id,
};

pub struct Subscription<K, T>
where
    K: Id,
    T: EventEntry<K>,
{
    pub(crate) channel_id: uuid::Uuid,
    pub(crate) receiver: Receiver<Event<K, T>>,
    pub(crate) sub_to_all: bool,
}

impl<K, T> PartialEq for Subscription<K, T>
where
    K: Id,
    T: EventEntry<K>,
{
    fn eq(&self, other: &Self) -> bool {
        self.channel_id == other.channel_id
    }
}

impl<K, T> Eq for Subscription<K, T>
where
    K: Id,
    T: EventEntry<K>,
{
}

impl<K, T> Subscription<K, T>
where
    K: Id,
    T: EventEntry<K>,
{
    pub fn get_receiver(&self) -> &Receiver<Event<K, T>> {
        &self.receiver
    }
}

impl<K, T> Hash for Subscription<K, T>
where
    K: Id,
    T: EventEntry<K>,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.channel_id.hash(state);
    }
}

#[derive(Clone)]
pub(crate) struct SubscriptionSender<K, T>
where
    K: Id,
    T: EventEntry<K>,
    SyncSender<Event<K, T>>: Clone,
{
    pub(crate) channel_id: uuid::Uuid,
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
