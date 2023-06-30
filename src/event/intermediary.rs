use crate::publisher::Id;

use super::{entry::EventEntry, origin::Origin};

pub trait IntermediaryEvent<K, T>
where
    Self: std::marker::Sized,
    K: Id,
    T: EventEntry<K>,
{
    fn new(event_id: K, msg: &str, origin: Origin) -> Self;

    fn get_entry(&self) -> &T;

    fn set_entry_nr(&mut self, entry_nr: u64);

    fn take_entry(&mut self) -> T;

    /// Returns the [`Id`] of this event
    fn get_event_id(&self) -> &K {
        self.get_entry().get_event_id()
    }
}
