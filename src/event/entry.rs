use std::hash::Hash;

use crate::publisher::Id;

use super::origin::Origin;

pub trait EventEntry<K: Id>: Default + Clone + Hash + Send + Sync + 'static {
    fn new(event_id: K, msg: &str, origin: Origin) -> Self;

    fn get_event_id(&self) -> &K;

    fn into_event_id(self) -> K;

    fn get_entry_id(&self) -> crate::uuid::Uuid;

    /// Get the main message that was set when the event entry was created.
    fn get_msg(&self) -> &str;

    fn get_crate_name(&self) -> &'static str;

    fn get_origin(&self) -> &Origin;
}
