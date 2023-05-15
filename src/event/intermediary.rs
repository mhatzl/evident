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
        crate_name: &'static str,
        module_path: &'static str,
        filename: &'static str,
        line_nr: u32,
    ) -> Self;

    fn get_entry(&self) -> &T;

    fn take_entry(&mut self) -> T;

    /// Returns the [`Id`] of this event
    fn get_id(&self) -> &K {
        self.get_entry().get_event_id()
    }

    /// Returns the name of the associated crate of this event
    fn get_crate_name(&self) -> &'static str {
        self.get_entry().get_crate_name()
    }

    /// Finalizing the event sends it to the publisher, and returns the [`Id`] of the event.
    ///  
    /// Note: Finalizing prevents any further information to be added to the event.
    fn finalize(self) -> CapturedEvent<K> {
        let captured_event = CapturedEvent {
            event_id: self.get_entry().get_event_id().clone(),
            entry_id: self.get_entry().get_entry_id(),
        };
        drop(self);
        captured_event
    }
}

pub struct CapturedEvent<K> {
    pub(crate) event_id: K,
    pub(crate) entry_id: crate::uuid::Uuid,
}

impl<K: Id> CapturedEvent<K> {
    pub fn get_event_id(&self) -> &K {
        &self.event_id
    }

    pub fn get_emtry_id(&self) -> crate::uuid::Uuid {
        self.entry_id
    }
}
