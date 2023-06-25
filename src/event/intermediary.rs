use crate::publisher::Id;

use super::{entry::EventEntry, finalized::FinalizedEvent, origin::Origin};

pub trait IntermediaryEvent<K, T>
where
    Self: std::marker::Sized,
    K: Id,
    T: EventEntry<K>,
{
    fn new(event_id: K, msg: &str, origin: Origin) -> Self;

    fn get_entry(&self) -> &T;

    fn take_entry(&mut self) -> T;

    /// Returns the [`Id`] of this event
    fn get_event_id(&self) -> &K {
        self.get_entry().get_event_id()
    }

    /// Returns the name of the associated crate of this event
    fn get_crate_name(&self) -> &'static str {
        self.get_entry().get_crate_name()
    }

    /// Finalizing the event sends it to the publisher, and returns the [`FinalizedEvent`].
    /// This struct includes the [`Id`] used to set the event, and the id of the specific [`EventEntry`]
    /// associated with this event.
    ///  
    /// Note: Finalizing prevents any further information to be added to the event.
    fn finalize(self) -> FinalizedEvent<K> {
        let entry_id = self.get_entry().get_entry_id();
        let captured_event = FinalizedEvent::new(
            // Note: Not cloning here would not fully drop the event => no event would be captured.
            self.get_event_id().clone(),
            entry_id,
        );
        drop(self);
        captured_event
    }

    fn into_event_id(self) -> K {
        self.finalize().into_event_id()
    }
}
