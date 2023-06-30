use crate::publisher::Id;

use super::{entry::EventEntry, intermediary::IntermediaryEvent};

pub trait FinalizeEvent<K, T>: IntermediaryEvent<K, T>
where
    Self: std::marker::Sized,
    K: Id,
    T: EventEntry<K>,
{
    /// Finalizing the event sends it to the publisher, and returns the [`FinalizedEvent`].
    /// This struct includes the [`Id`] used to set the event, and the id of the specific [`EventEntry`]
    /// associated with this event.
    ///
    /// Note: Finalizing prevents any further information to be added to the event.
    fn finalize(self) -> FinalizedEvent<K>;

    fn into_event_id(self) -> K {
        self.finalize().into_event_id()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct FinalizedEvent<K: Id> {
    pub event_id: K,
    pub entry_nr: u64,
}

impl<K: Id> FinalizedEvent<K> {
    pub fn new(event_id: K, entry_nr: u64) -> Self {
        FinalizedEvent { event_id, entry_nr }
    }

    pub fn into_event_id(self) -> K {
        self.event_id
    }

    pub fn get_event_id(&self) -> &K {
        &self.event_id
    }

    pub fn get_entry_nr(&self) -> u64 {
        self.entry_nr
    }
}

impl<K: Id + std::fmt::Display> std::fmt::Display for FinalizedEvent<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id='{}', entry='{}'", self.event_id, self.entry_nr)
    }
}
