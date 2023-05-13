use std::marker::PhantomData;

use crate::publisher::Id;

use super::entry::EventEntry;

/// Struct linking a [`LogId`] to the map the entry for the ID was added to.
#[derive(Default, Clone, PartialEq, Eq)]
pub struct IntermediaryEvent<K, T>
where
    K: Id,
    T: EventEntry<K>,
{
    /// [`Entry`] for the [`LogIdEvent`] storing all event information.
    pub(crate) entry: T,
    phantom: PhantomData<K>,
}

impl<K: Id, T: EventEntry<K>> IntermediaryEvent<K, T> {
    pub(crate) fn new(
        id: K,
        crate_name: &str,
        msg: &str,
        filename: &str,
        line_nr: u32,
        module_path: &str,
    ) -> Self {
        Self {
            entry: EventEntry::new(id, msg, crate_name, filename, line_nr, module_path),
            phantom: PhantomData,
        }
    }
}

//TODO: move into macro_rule
// impl<K: Id, T: EventEntry<K>> From<IntermediaryEvent<K, T>> for K {
//     fn from(intermed_event: IntermediaryEvent<K, T>) -> Self {
//         intermed_event.finalize()
//     }
// }

// impl<K: Id, T: EventEntry<K>> PartialEq<IntermediaryEvent<K, T>> for K {
//     fn eq(&self, other: &IntermediaryEvent<K, T>) -> bool {
//         *self == other.entry.get_id()
//     }
// }

// Note: macro needed for drop-impl, where custom static publisher is given.
#[macro_export]
macro_rules! capture_drop {
    ($publisher:ident, $entry_type:ty) => {
        impl Drop for IntermediaryEvent<$entry_type> {
            /// On drop, transforms the [`IntermediaryEvent<T>`] into an [`Event`] that gets sent to the central publisher.
            fn drop(&mut self) {
                publisher.capture(self);
            }
        }
    };
}

// Note: macro needed for drop-impl, where custom static publisher is given.
#[macro_export]
macro_rules! try_capture_drop {
    ($publisher:ident, $entry_type:ty) => {
        impl Drop for IntermediaryEvent<$entry_type> {
            /// On drop, transforms the [`IntermediaryEvent<T>`] into an [`Event`] that gets sent to the central publisher.
            fn drop(&mut self) {
                publisher.try_capture(self);
            }
        }
    };
}

impl<K: Id, T: EventEntry<K>> IntermediaryEvent<K, T> {
    /// Returns the [`LogId`] of this log-id event
    pub fn get_id(&self) -> K {
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

    /// Finalizing a [`LogIdEvent`] converts it back to a [`LogId`].
    /// This prevents any further information to be added to it.
    /// If the event was not created *silently*, it also moves the entry into the [`LogIdMap`] associated with the event.
    pub fn finalize(self) -> K {
        let id = self.entry.get_event_id();
        drop(self);
        id
    }
}
