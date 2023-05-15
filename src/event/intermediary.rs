use crate::publisher::Id;

use super::{entry::EventEntry, origin::Origin};

#[allow(drop_bounds)]
pub trait IntermediaryEvent<K, T>: Drop
where
    Self: std::marker::Sized,
    K: Id,
    T: EventEntry<K>,
{
    fn new(event_id: K, msg: &str, origin: Origin) -> Self;

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

    /// Finalizing the event sends it to the publisher, and returns the [`CapturedEvent`].
    /// This struct includes the [`Id`] used to set the event, and the id of the specific [`EventEntry`]
    /// associated with this event.
    ///  
    /// Note: Finalizing prevents any further information to be added to the event.
    fn finalize(self) -> CapturedEvent<K> {
        let entry_id = self.get_entry().get_entry_id();
        let captured_event = CapturedEvent {
            // Note: Not cloning here would not fully drop the event => no event would be captured.
            event_id: self.get_entry().get_event_id().clone(),
            entry_id,
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

    pub fn into_event_id(self) -> K {
        self.event_id
    }

    pub fn get_entry_id(&self) -> crate::uuid::Uuid {
        self.entry_id
    }
}
