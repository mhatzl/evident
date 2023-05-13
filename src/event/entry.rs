use crate::publisher::Id;

use super::origin::Origin;

pub trait EventEntry<K: Id>: Default + Clone + Send + Sync + 'static {
    fn new(
        event_id: K,
        msg: &str,
        crate_name: &str,
        filename: &str,
        line_nr: u32,
        module_path: &str,
    ) -> Self;

    fn get_event_id(&self) -> &K;

    fn get_entry_id(&self) -> uuid::Uuid;

    /// Get the main message that was set when the event entry was created.
    fn get_msg(&self) -> &str;

    fn get_crate_name(&self) -> &str;

    fn get_origin(&self) -> &Origin;
}
