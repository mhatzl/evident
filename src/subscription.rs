use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    sync::{
        mpsc::{Receiver, SyncSender},
        Arc,
    },
};

use crate::{
    event::{entry::EventEntry, filter::Filter, Event, Id, Msg},
    publisher::{CaptureControl, EvidentPublisher},
};

/// Subscription that is returned when subscribing to events captured by an [`EvidentPublisher`].
///
///[req:subs]
pub struct Subscription<'p, K, M, T, F>
where
    K: Id + CaptureControl,
    M: Msg,
    T: EventEntry<K, M>,
    F: Filter<K, M>,
{
    /// The ID of the channel used to send events from the [`EvidentPublisher`] to the [`Subscription`].
    pub(crate) channel_id: crate::uuid::Uuid,

    /// The channel [`Receiver`] used to receive captured events from the [`EvidentPublisher`].
    pub(crate) receiver: Receiver<Arc<Event<K, M, T>>>,

    /// Flag set to `true` if this [`Subscription`] is subscribed to receive all captured events.
    pub(crate) sub_to_all: bool,

    /// Optional set of event-IDs this [`Subscription`] is subscribed to.
    ///
    /// **Note:** Only relevant for subscriptions to specific event-IDs.
    pub(crate) subscriptions: Option<HashSet<K>>,

    /// A reference to the [`EvidentPublisher`] the [`Subscription`] was created from.
    pub(crate) publisher: &'p EvidentPublisher<K, M, T, F>,
}

impl<'p, K, M, T, F> Subscription<'p, K, M, T, F>
where
    K: Id + CaptureControl,
    M: Msg,
    T: EventEntry<K, M>,
    F: Filter<K, M>,
{
    /// Get the [`Receiver`] of the subscription channel.
    pub fn get_receiver(&self) -> &Receiver<Arc<Event<K, M, T>>> {
        &self.receiver
    }

    /// Unsubscribes this subscription.
    pub fn unsubscribe(self) {
        drop(self)
    }

    /// Unsubscribes from the given event-ID.
    ///
    /// **Note:** Only possible for subscriptions to specific IDs.
    ///
    /// # Arguments
    ///
    /// * `id` ... Event-ID the subscription should be unsubscribed from
    ///
    /// # Possible Errors
    ///
    /// * [`SubscriptionError::IdNotSubscribed`] ... If athe given ID was not subscribed,
    /// * [`SubscriptionError::UnsubscribeWouldDeleteSubscription`] ... If the [`Subscription`] would not be subscribed to any ID afterwards
    /// * [`SubscriptionError::AllEventsSubscriptionNotModifiable`] ... If the [`Subscription`] was created to receive all events
    pub fn unsubscribe_id(&mut self, id: K) -> Result<(), SubscriptionError<K>> {
        self.unsubscribe_many(vec![id])
    }

    /// Unsubscribes from the given list of event-IDs.
    ///
    /// **Note:** Only possible for subscriptions to specific IDs.
    ///
    /// # Arguments
    ///
    /// * `ids` ... List of event-IDs the subscription should be unsubscribed from
    ///
    /// # Possible Errors
    ///
    /// * [`SubscriptionError::IdNotSubscribed`] ... If any of the given IDs was not subscribed,
    /// * [`SubscriptionError::UnsubscribeWouldDeleteSubscription`] ... If the [`Subscription`] would not be subscribed to any ID afterwards
    /// * [`SubscriptionError::AllEventsSubscriptionNotModifiable`] ... If the [`Subscription`] was created to receive all events
    pub fn unsubscribe_many(&mut self, ids: Vec<K>) -> Result<(), SubscriptionError<K>> {
        if self.sub_to_all || self.subscriptions.is_none() {
            return Err(SubscriptionError::AllEventsSubscriptionNotModifiable);
        }

        let subs = self.subscriptions.as_mut().unwrap();

        if ids.len() >= subs.len() {
            return Err(SubscriptionError::UnsubscribeWouldDeleteSubscription);
        }

        for id in &ids {
            if !subs.contains(id) {
                return Err(SubscriptionError::IdNotSubscribed(id.clone()));
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
            Err(_) => Err(SubscriptionError::CouldNotAccessPublisher),
        }
    }

    /// Subscribes to the given event-ID.
    ///
    /// **Note:** Only possible for subscriptions to specific IDs.
    ///
    /// # Arguments
    ///
    /// * `id` ... Event-ID that should be added to the subscribed IDs by the [`Subscription`]
    ///
    /// # Possible Errors
    ///
    /// * [`SubscriptionError::IdAlreadySubscribed`] ... If the given ID is already subscribed,
    /// * [`SubscriptionError::CouldNotAccessPublisher`] ... If the [`Subscription`] has no connection to the [`EvidentPublisher`]
    /// * [`SubscriptionError::NoSubscriptionChannelAvailable`] ... If the [`EvidentPublisher`] has no stored channel to this [`Subscription`]
    /// * [`SubscriptionError::AllEventsSubscriptionNotModifiable`] ... If the [`Subscription`] was created to receive all events
    ///
    /// [req:subs.specific.one]
    pub fn subscribe_id(&mut self, id: K) -> Result<(), SubscriptionError<K>> {
        self.subscribe_many(vec![id])
    }

    /// Subscribes to the given list of event-IDs.
    ///
    /// **Note:** Only possible for subscriptions to specific IDs.
    ///
    /// # Arguments
    ///
    /// * `ids` ... List of event-IDs that should be added to the subscribed IDs by the [`Subscription`]
    ///
    /// # Possible Errors
    ///
    /// * [`SubscriptionError::IdAlreadySubscribed`] ... If one of the given IDs is already subscribed,
    /// * [`SubscriptionError::CouldNotAccessPublisher`] ... If the [`Subscription`] has no connection to the [`EvidentPublisher`]
    /// * [`SubscriptionError::NoSubscriptionChannelAvailable`] ... If the [`EvidentPublisher`] has no stored channel to this [`Subscription`]
    /// * [`SubscriptionError::AllEventsSubscriptionNotModifiable`] ... If the [`Subscription`] was created to receive all events
    ///
    /// [req:subs.specific.mult]
    pub fn subscribe_many(&mut self, ids: Vec<K>) -> Result<(), SubscriptionError<K>> {
        if self.sub_to_all || self.subscriptions.is_none() {
            return Err(SubscriptionError::AllEventsSubscriptionNotModifiable);
        }

        let subs = self.subscriptions.as_mut().unwrap();

        for id in &ids {
            if subs.contains(id) {
                return Err(SubscriptionError::IdAlreadySubscribed(id.clone()));
            }
        }
        // Needed to clone the *sender* of the subscription channel, which is stored in the publisher.
        let any_sub_id = match subs.iter().next() {
            Some(id) => id,
            None => {
                return Err(SubscriptionError::NoSubscriptionChannelAvailable);
            }
        };

        let sender = match self.publisher.subscriptions.read() {
            Ok(publisher_subs) => match publisher_subs.get(any_sub_id) {
                Some(id_subs) => match id_subs.get(&self.channel_id) {
                    Some(sub_sender) => sub_sender.clone(),
                    None => {
                        return Err(SubscriptionError::NoSubscriptionChannelAvailable);
                    }
                },
                None => {
                    return Err(SubscriptionError::NoSubscriptionChannelAvailable);
                }
            },
            Err(_) => {
                return Err(SubscriptionError::CouldNotAccessPublisher);
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
            Err(_) => Err(SubscriptionError::CouldNotAccessPublisher),
        }
    }
}

impl<'p, K, M, T, F> Drop for Subscription<'p, K, M, T, F>
where
    K: Id + CaptureControl,
    M: Msg,
    T: EventEntry<K, M>,
    F: Filter<K, M>,
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

impl<'p, K, M, T, F> PartialEq for Subscription<'p, K, M, T, F>
where
    K: Id + CaptureControl,
    M: Msg,
    T: EventEntry<K, M>,
    F: Filter<K, M>,
{
    fn eq(&self, other: &Self) -> bool {
        self.channel_id == other.channel_id
    }
}

impl<'p, K, M, T, F> Eq for Subscription<'p, K, M, T, F>
where
    K: Id + CaptureControl,
    M: Msg,
    T: EventEntry<K, M>,
    F: Filter<K, M>,
{
}

impl<'p, K, M, T, F> Hash for Subscription<'p, K, M, T, F>
where
    K: Id + CaptureControl,
    M: Msg,
    T: EventEntry<K, M>,
    F: Filter<K, M>,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.channel_id.hash(state);
    }
}

/// Possible errors for (un)subscribe functions.
#[derive(Debug, Clone)]
pub enum SubscriptionError<K: Id> {
    /// This [`Subscription`] was created to listen to all events, which cannot be modified afterwards.
    AllEventsSubscriptionNotModifiable,

    /// Event-ID is not subscribed.
    /// Therefore, the ID cannot be unsubscribed.
    ///
    /// The problematic ID may be accessed at tuple position 0.
    IdNotSubscribed(K),

    /// Event-ID is already subscribed.
    /// Therefore, the ID cannot be re-subscribed.
    ///
    /// The problematic ID may be accessed at tuple position 0.
    IdAlreadySubscribed(K),

    /// Unsubscribing would remove all subscriptions to specific event-IDs.
    /// This would remove all conntections between the [`Subscription`] and the [`EvidentPublisher`], making it impossible to modify the subscription at a later point.
    UnsubscribeWouldDeleteSubscription,

    /// Could not lock access to the [`EvidentPublisher`].
    CouldNotAccessPublisher,

    /// No *sender-part* of the subscription-channel between this [`Subscription`] and the [`EvidentPublisher`] is available in the [`EvidentPublisher`].
    NoSubscriptionChannelAvailable,
}

/// *Sender-part* of the subscription-channel between a [`Subscription`] and an [`EvidentPublisher`].
///
/// [req:subs]
#[derive(Clone)]
pub(crate) struct SubscriptionSender<K, M, T>
where
    K: Id,
    M: Msg,
    T: EventEntry<K, M>,
{
    /// ID to identify the *sender-part* in the [`EvidentPublisher`].
    pub(crate) channel_id: crate::uuid::Uuid,

    /// [`SyncSender`] of the [`sync_channel`](std::sync::mpsc::sync_channel) between [`Subscription`] and [`EvidentPublisher`].
    pub(crate) sender: SyncSender<Arc<Event<K, M, T>>>,
}

impl<K, M, T> PartialEq for SubscriptionSender<K, M, T>
where
    K: Id,
    M: Msg,
    T: EventEntry<K, M>,
{
    fn eq(&self, other: &Self) -> bool {
        self.channel_id == other.channel_id
    }
}

impl<K, M, T> Eq for SubscriptionSender<K, M, T>
where
    K: Id,
    M: Msg,
    T: EventEntry<K, M>,
{
}

impl<K, M, T> Hash for SubscriptionSender<K, M, T>
where
    K: Id,
    M: Msg,
    T: EventEntry<K, M>,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.channel_id.hash(state);
    }
}
