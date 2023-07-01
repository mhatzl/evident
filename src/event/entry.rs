use std::hash::Hash;

use super::{origin::Origin, Id, Msg};

pub trait EventEntry<K: Id, M: Msg>: Default + Clone + Hash + Send + Sync + 'static {
    fn new(event_id: K, msg: Option<impl Into<M>>, origin: Origin) -> Self;

    fn get_event_id(&self) -> &K;

    fn into_event_id(self) -> K;

    fn get_entry_id(&self) -> crate::uuid::Uuid;

    /// Get the main message that was set when the event entry was created.
    fn get_msg(&self) -> Option<&M>;

    fn get_origin(&self) -> &Origin;
}
