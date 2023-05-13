use crate::publisher::Id;

use super::entry::EventEntry;


#[allow(drop_bounds)]
pub trait IntermediaryEvent<K, T>: Drop
where
    Self: std::marker::Sized,
    K: Id,
    T: EventEntry<K>,
{
    fn new(
        event_id: K,
        msg: &str,
        crate_name: &str,
        filename: &str,
        line_nr: u32,
        module_path: &str,
    ) -> Self;
    
    fn get_entry(&self) -> &T;

    fn take_entry(&mut self) -> T;

    /// Returns the [`LogId`] of this log-id event
    fn get_id(&self) -> &K {
        self.get_entry().get_event_id()
    }

    /// Returns the name of the associated crate of this log-id event
    fn get_crate_name(&self) -> &str {
        self.get_entry().get_crate_name()
    }

    /// Finalizing a [`LogIdEvent`] converts it back to a [`LogId`].
    /// This prevents any further information to be added to it.
    /// If the event was not created *silently*, it also moves the entry into the [`LogIdMap`] associated with the event.
    fn finalize(self) -> K {
        let id = self.get_entry().get_event_id().clone();
        drop(self);
        id
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
    ($publisher:ident, $iterim_event_type:ty) => {
        impl Drop for $iterim_event_typ {
            /// On drop, transforms the [`IntermediaryEvent<T>`] into an [`Event`] that gets sent to the central publisher.
            fn drop(&mut self) {
                $publisher.capture(self);
            }
        }
    };
}

// Note: macro needed for drop-impl, where custom static publisher is given.
#[macro_export]
macro_rules! try_capture_drop {
    ($publisher:ident, $iterim_event_type:ty) => {
        impl Drop for $iterim_event_type {
            /// On drop, transforms the [`IntermediaryEvent<T>`] into an [`Event`] that gets sent to the central publisher.
            fn drop(&mut self) {
                $publisher.try_capture(self);
            }
        }
    };
}
