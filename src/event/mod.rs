use std::marker::PhantomData;

use crate::publisher::Id;

use self::{entry::EventEntry, intermediary::IntermediaryEvent, origin::Origin};

pub mod entry;
pub mod intermediary;
pub mod origin;

/// Trait to create an [`IntermediaryEvent<K, T>`] that is captured by a publisher once the event is either
/// explicitly `finalized`, or implicitly dropped.
pub trait EventFns<K: Id, T: EventEntry<K>, I: IntermediaryEvent<K, T>> {
    /// Set an event for an [`Id`].
    ///
    /// # Arguments
    ///
    /// * `crate_name` ... Name of the crate the event should be associated with
    /// * `module_path` ... Module path where the event is set (Note: use `module_path!()`)
    /// * `msg` ... Main message that is set for this event (should be a user-centered event description)
    /// * `filename` ... Name of the source file where the event is set (Note: use `file!()`)
    /// * `line_nr` ... Line number where the event is set (Note: use `line!()`)
    fn set_event(
        self,
        msg: &str,
        crate_name: &'static str,
        module_path: &'static str,
        filename: &'static str,
        line_nr: u32,
    ) -> I;
}

impl<K: Id, T: EventEntry<K>, I: IntermediaryEvent<K, T>> EventFns<K, T, I> for K {
    fn set_event(
        self,
        msg: &str,
        crate_name: &'static str,
        module_path: &'static str,
        filename: &'static str,
        line_nr: u32,
    ) -> I {
        I::new(self, msg, crate_name, module_path, filename, line_nr)
    }
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

    pub fn get_entry_id(&self) -> uuid::Uuid {
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
