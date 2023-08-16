//! Contains the *evident* [`Event`], and all related traits, structures, and functions.
//!
//! [req:event]

use std::marker::PhantomData;

use self::{entry::EventEntry, intermediary::IntermediaryEvent, origin::Origin};

pub mod entry;
pub mod filter;
pub mod finalized;
pub mod intermediary;
pub mod origin;

/// Trait that must be implemented for a custom *evident* ID.\
/// This implementation must then be used for implementations of the traits [`EventEntry`] and [`IntermediaryEvent`].\
/// All implementations are needed to create an *evident* publisher using the [`create_static_publisher!()`](crate::create_static_publisher) macro.
///
/// The optional [`Filter`](self::filter::Filter) trait must also use the same implementation of this [`Id`] trait.
///
/// [req:event.id], [req:event.id.generic]
pub trait Id:
    core::fmt::Debug + Default + Clone + std::hash::Hash + PartialEq + Eq + Send + Sync + 'static
{
}

/// Trait that must be implemented for a custom event message.\
/// This implementation must then be used for implementations of the traits [`EventEntry`] and [`IntermediaryEvent`].\
/// All implementations are needed to create an *evident* publisher using the [`create_static_publisher!()`](crate::create_static_publisher) macro.
///
/// The optional [`Filter`](self::filter::Filter) trait must also use the same implementation of this [`Msg`] trait.
///
/// **Note:** This trait is already implemented for [`String`].
///
/// [req:event.msg]
pub trait Msg: core::fmt::Debug + Clone + Send + Sync + 'static {}

impl Msg for String {}

/// Set an event for an [`Id`] with an explicit message.
/// You may want to use [`create_set_event_macro`](crate::create_set_event_macro) to create a convenient wrapper for the `set_event` functions.
///
/// # Arguments
///
/// * `event_id` ... The [`Id`] used for this event
/// * `msg` ... Main message that is set for this event
/// * `origin` ... The [`Origin`] the event was set at (Note: Use macro [`this_origin`](crate::this_origin))
///
/// [req:event.set], [req:event.origin]
pub fn set_event_with_msg<K: Id, M: Msg, E: EventEntry<K, M>, I: IntermediaryEvent<K, M, E>>(
    event_id: K,
    msg: impl Into<M>,
    origin: Origin,
) -> I {
    I::new(event_id, Some(msg), origin)
}

/// Set an event for an [`Id`] without a message.
/// You may want to use [`create_set_event_macro`](crate::create_set_event_macro) to create a convenient wrapper for the `set_event` functions.
///
/// # Arguments
///
/// * `event_id` ... The [`Id`] used for this event
/// * `origin` ... The [`Origin`] the event was set at (Note: Use macro [`this_origin`](crate::this_origin))
///
/// [req:event.set], [req:event.origin]
pub fn set_event<K: Id, M: Msg, E: EventEntry<K, M>, I: IntermediaryEvent<K, M, E>>(
    event_id: K,
    origin: Origin,
) -> I {
    let empty_msg: Option<M> = None;
    I::new(event_id, empty_msg, origin)
}

/// *evident* event that is sent to subscribers if they are subscribed to the [`Id`] of this event.
///
/// [req:event]
#[derive(Clone, PartialEq, Eq)]
pub struct Event<K, M, T>
where
    K: Id,
    M: Msg,
    T: EventEntry<K, M>,
{
    /// [`EventEntry`] of the event.
    ///
    /// [req:event.entry]
    pub(crate) entry: T,

    // PahmtomData needed for unused generics
    phantom_k: PhantomData<K>,
    phantom_m: PhantomData<M>,

    /// The [`ThreadId`](std::thread::ThreadId) of the thread the event was set in.
    thread_id: std::thread::ThreadId,
    /// The name of the thread the event was set in if a name exists.
    /// Otherwise: `None`
    thread_name: Option<String>,

    /// The [`SystemTime`](std::time::SystemTime) when the event was set.
    pub(crate) timestamp: Option<std::time::SystemTime>,
}

impl<K: Id, M: Msg, T: EventEntry<K, M>> Event<K, M, T> {
    /// Creates a new [`Event`] from an [`EventEntry`].
    pub fn new(entry: T) -> Self {
        let curr_thread = std::thread::current();

        Event {
            entry,
            phantom_k: PhantomData,
            phantom_m: PhantomData,

            thread_id: curr_thread.id(),
            thread_name: curr_thread.name().map(|s| s.to_string()),

            timestamp: None,
        }
    }

    /// Returns the [`Id`] of this event.
    ///
    /// [req:event.id]
    pub fn get_event_id(&self) -> &K {
        self.entry.get_event_id()
    }

    /// Returns the [`EventEntry`] of this event.
    ///
    /// [req:event.entry]
    pub fn get_entry(&self) -> &T {
        &self.entry
    }

    /// Get the entry-ID that was generated when the event was set.
    ///
    /// [req:event.entry.id]
    pub fn get_entry_id(&self) -> crate::uuid::Uuid {
        self.entry.get_entry_id()
    }

    /// Get the main message that was given when the event was set,
    /// or `None` if no message was given.
    ///
    /// [req:event.msg]
    pub fn get_msg(&self) -> Option<&M> {
        self.entry.get_msg()
    }

    /// Get the [`Origin`] the event was set at.
    ///
    /// [req:event.origin]
    pub fn get_origin(&self) -> &Origin {
        self.entry.get_origin()
    }

    /// Get the [`ThreadId`](std::thread::ThreadId) of the thread the event was set in.
    pub fn get_thread_id(&self) -> &std::thread::ThreadId {
        &self.thread_id
    }

    /// Get the name of the thread the event was set in.
    pub fn get_thread_name(&self) -> Option<&str> {
        self.thread_name.as_deref()
    }

    /// Get the [`SystemTime`](std::time::SystemTime) timestamp of the event.
    pub fn get_timestamp(&self) -> &Option<std::time::SystemTime> {
        &self.timestamp
    }
}

impl<K: Id, M: Msg, T: EventEntry<K, M>> core::fmt::Debug for Event<K, M, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Event")
            .field("id", &self.entry.get_event_id())
            .field("entry_id", &self.entry.get_entry_id())
            .field("origin", &self.entry.get_origin())
            .finish()
    }
}
