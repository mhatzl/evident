//! Contains the [`EventEntry] trait.
//!
//! [req:event.entry](https://github.com/mhatzl/evident/wiki/5-REQ-event.entry#evententry-event-entry)

use std::hash::Hash;

use super::{origin::Origin, Id, Msg};

/// Trait that must be implemented for a custom *evident* event-entry.\
/// This implementation must then be used for implementations of the traits [`EventEntry`] and [`IntermediaryEvent`](super::intermediary::IntermediaryEvent).\
/// All implementations are needed to create an *evident* publisher using the [`create_static_publisher!()`](crate::create_static_publisher) macro.
///
/// The optional [`Filter`](super::filter::Filter) trait must also use the same implementation of this [`Id`] trait.
///
/// **Note:** Since it is a trait, the custom implementation may contain additional fields and functions.
///
/// [req:event.entry](https://github.com/mhatzl/evident/wiki/5-REQ-event.entry#evententry-event-entry), [req:event.entry.generic](https://github.com/mhatzl/evident/wiki/5-REQ-event.entry.generic#evententrygeneric-generic-event-entry)
pub trait EventEntry<K: Id, M: Msg>: Default + Clone + Hash + Send + Sync + 'static {
    /// Creates a new [`EventEntry`].
    ///
    /// **Note:** This function should be called inside the implementation for [`IntermediaryEvent::new`](super::intermediary::IntermediaryEvent::new).
    ///
    /// # Arguments
    ///
    /// * `event_id` ... The ID of the event that was set to create this entry
    /// * `msg` ... Optional main event message
    /// * `origin` ... The [`Origin`] the event was set at
    ///
    /// [req:event.entry](https://github.com/mhatzl/evident/wiki/5-REQ-event.entry#evententry-event-entry), [req:event.id](https://github.com/mhatzl/evident/wiki/5-REQ-event.id#eventid-event-identifier), [req:event.msg](https://github.com/mhatzl/evident/wiki/5-REQ-event.msg#eventmsg-event-message), [req:event.origin](https://github.com/mhatzl/evident/wiki/5-REQ-event.origin#eventorigin-get-the-event-origin)
    fn new(event_id: K, msg: Option<impl Into<M>>, origin: Origin) -> Self;

    /// Returns the [`Id`] of this event.
    ///
    /// [req:event.id](https://github.com/mhatzl/evident/wiki/5-REQ-event.id#eventid-event-identifier)
    fn get_event_id(&self) -> &K;

    /// Convert this [`EventEntry`] into the [`Id`] of this event.
    ///
    /// [req:event.id](https://github.com/mhatzl/evident/wiki/5-REQ-event.id#eventid-event-identifier)
    fn into_event_id(self) -> K;

    /// Get the entry-ID that was generated when the event was set.
    ///
    /// [req:event.entry.id](https://github.com/mhatzl/evident/wiki/5-REQ-event.entry.id#evententryid-unique-event-entry)
    fn get_entry_id(&self) -> crate::uuid::Uuid;

    /// Get the main message that was given when the event was set,
    /// or `None` if no message was given.
    ///
    /// [req:event.msg](https://github.com/mhatzl/evident/wiki/5-REQ-event.msg#eventmsg-event-message)
    fn get_msg(&self) -> Option<&M>;

    /// Get the [`Origin`] the event was set at.
    ///
    /// [req:event.origin](https://github.com/mhatzl/evident/wiki/5-REQ-event.origin#eventorigin-get-the-event-origin)
    fn get_origin(&self) -> &Origin;
}
