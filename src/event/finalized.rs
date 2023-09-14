//! Contains the [`FinalizedEvent`] struct.

use super::Id;

/// The [`FinalizedEvent`] is returned once a event is captured using [`finalize()`](super::IntermediaryEvent::finalize).
/// It stores only the event and entry IDs to prevent unnecessarry resource cloning.  
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct FinalizedEvent<K: Id> {
    /// The [`Id`] of the associated [`Event`](super::Event).
    pub event_id: K,

    /// The ID of the associated [`EventEntry`](super::entry::EventEntry).
    pub entry_id: crate::uuid::Uuid,
}

impl<K: Id> FinalizedEvent<K> {
    /// Creates a new [`FinalizedEvent`].
    pub fn new(event_id: K, entry_id: crate::uuid::Uuid) -> Self {
        FinalizedEvent { event_id, entry_id }
    }

    /// Converts this [`FinalizedEvent`] into the associated event [`Id`].
    pub fn into_event_id(self) -> K {
        self.event_id
    }

    /// Returns the associated event [`Id].
    pub fn get_event_id(&self) -> &K {
        &self.event_id
    }

    /// Returns the associated [`EventEntry`](super::entry::EventEntry) ID.
    pub fn get_entry_id(&self) -> &crate::uuid::Uuid {
        &self.entry_id
    }
}

impl<K: Id + std::fmt::Display> std::fmt::Display for FinalizedEvent<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id='{}', entry='{}'", self.event_id, self.entry_id)
    }
}
