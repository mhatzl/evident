use std::marker::PhantomData;

use crate::publisher::Id;

use self::{entry::EventEntry, intermediary::IntermediaryEvent};

pub mod entry;
pub mod intermediary;
pub mod origin;

/// Trait to use [`LogId`] for tracing.
pub trait EventFns<K: Id, T: EventEntry<K>, I: IntermediaryEvent<K, T>> {
    /// Set an event for a [`LogId`], and storing it inside the [`LogIdMap`] of the given crate name.
    ///
    /// # Arguments
    ///
    /// * `crate_name` ... Name of the crate to identify the [`LogIdMap`]
    /// * `msg` ... Main message that is set for this log-id (should be a user-centered event description)
    /// * `filename` ... Name of the source file where the event is set (Note: use `file!()`)
    /// * `line_nr` ... Line number where the event is set (Note: use `line!()`)
    /// * `module_path` ... Module path where the event is set (Note: use `module_path!()`)
    fn set_event(
        self,
        crate_name: &str,
        msg: &str,
        filename: &str,
        line_nr: u32,
        module_path: &str,
    ) -> I;
}

impl<K: Id, T: EventEntry<K>, I: IntermediaryEvent<K, T>> EventFns<K, T, I> for K {
    fn set_event(
        self,
        crate_name: &str,
        msg: &str,
        filename: &str,
        line_nr: u32,
        module_path: &str,
    ) -> I {
        I::new(self, crate_name, msg, filename, line_nr, module_path)
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

    /// Returns the [`LogId`] of this log-id event
    pub fn get_id(&self) -> &K {
        self.entry.get_event_id()
    }

    /// Returns the name of the associated crate of this log-id event
    pub fn get_crate_name(&self) -> &str {
        self.entry.get_crate_name()
    }

    /// Returns the [`Entry`] of this log-id event
    pub fn get_entry(&self) -> &T {
        &self.entry
    }
}
