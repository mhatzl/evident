use std::{fmt::Display, marker::PhantomData};

use crate::publisher::Id;

use self::{entry::EventEntry, intermediary::IntermediaryEvent, origin::Origin};

pub mod entry;
pub mod intermediary;
pub mod origin;

/// Set an event for an [`Id`].
///
/// # Arguments
///
/// * `event_id` ... The [`Id`] used for this event
/// * `msg` ... Main message that is set for this event (should be a user-centered event description)
/// * `origin` ... The origin where the event was set (Note: Use `this_origin!()`)
pub fn set_event_with<K: Id, E: EventEntry<K>, I: IntermediaryEvent<K, E>>(
    event_id: K,
    msg: &str,
    origin: Origin,
) -> I {
    I::new(event_id, msg, origin)
}

/// Set an event for an [`Id`].
///
/// # Arguments
///
/// * `event_id` ... The [`Id`] used for this event (`to_string()` of the given [`Id`] is used for the event message)
/// * `origin` ... The origin where the event was set (Note: Use `this_origin!()`)
pub fn set_event<K: Id + Display, E: EventEntry<K>, I: IntermediaryEvent<K, E>>(
    event_id: K,
    origin: Origin,
) -> I {
    let msg = event_id.to_string();
    I::new(event_id, &msg, origin)
}

#[derive(Default, Clone, PartialEq, Eq)]
pub struct Event<K, T>
where
    K: Id,
    T: EventEntry<K>,
{
    pub(crate) entry: T,
    phantom: PhantomData<K>,
}

impl<K: Id, T: EventEntry<K>> Event<K, T> {
    pub(crate) fn new(entry: T) -> Self {
        Event {
            entry,
            phantom: PhantomData,
        }
    }

    /// Returns the [`Id`] of this event
    pub fn get_id(&self) -> &K {
        self.entry.get_event_id()
    }

    /// Returns the name of the associated crate of this event
    pub fn get_crate_name(&self) -> &'static str {
        self.entry.get_crate_name()
    }

    /// Returns the [`EventEntry`] of this event
    pub fn get_entry(&self) -> &T {
        &self.entry
    }

    pub fn get_entry_id(&self) -> crate::uuid::Uuid {
        self.entry.get_entry_id()
    }

    /// Get the main message that was set when the event entry was created.
    pub fn get_msg(&self) -> &str {
        self.entry.get_msg()
    }

    pub fn get_origin(&self) -> &Origin {
        self.entry.get_origin()
    }
}

impl<K: Id, T: EventEntry<K>> core::fmt::Debug for Event<K, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Event")
            .field("id", &self.entry.get_event_id())
            .field("entry_id", &self.entry.get_entry_id())
            .field("origin", &self.entry.get_origin())
            .finish()
    }
}
