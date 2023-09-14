//! Contains the [`IntermediaryEvent`] trait.

use super::{entry::EventEntry, finalized::FinalizedEvent, origin::Origin, Id, Msg};

/// The [`IntermediaryEvent`] trait is used to add information after setting an event,
/// and to be able to automatically capture events once they go out of scope.
pub trait IntermediaryEvent<K, M, T>
where
    Self: std::marker::Sized,
    K: Id,
    M: Msg,
    T: EventEntry<K, M>,
{
    /// Create a new [`IntermediaryEvent`].
    ///
    /// # Arguments
    ///
    /// * `event_id` ... The [`Id`] of the event
    /// * `msg` ... An optional [`Msg`] set for this event
    /// * `origin` ... The [`Origin`] this event was set
    fn new(event_id: K, msg: Option<impl Into<M>>, origin: Origin) -> Self;

    /// Returns the [`EventEntry`] that was created by this [`IntermediaryEvent`].
    fn get_entry(&self) -> &T;

    /// Takes the [`EventEntry`] that was created by this [`IntermediaryEvent`].
    fn take_entry(&mut self) -> T;

    /// Returns the [`Id`] of this event
    fn get_event_id(&self) -> &K {
        self.get_entry().get_event_id()
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

    /// Converts this [`IntermediaryEvent`] into the related event [`Id`].
    fn into_event_id(self) -> K {
        self.finalize().into_event_id()
    }
}
