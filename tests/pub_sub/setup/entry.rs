use evident::event::{entry::EventEntry, origin::Origin};

use super::id::MinId;

#[derive(Default, Clone)]
pub struct MinEventEntry {
    event_id: MinId,
    msg: String,

    entry_id: uuid::Uuid,
    origin: Origin,
}

impl EventEntry<MinId> for MinEventEntry {
    fn new(
        event_id: MinId,
        msg: &str,
        crate_name: &str,
        filename: &str,
        line_nr: u32,
        module_path: &str,
    ) -> Self {
        MinEventEntry {
            event_id,
            msg: msg.to_string(),

            entry_id: uuid::Uuid::new_v4(),
            origin: Origin::new(crate_name, module_path, filename, line_nr),
        }
    }

    fn get_event_id(&self) -> &MinId {
        &self.event_id
    }

    fn get_entry_id(&self) -> uuid::Uuid {
        self.entry_id
    }

    fn get_msg(&self) -> &str {
        &self.msg
    }

    fn get_crate_name(&self) -> &str {
        &self.origin.crate_name
    }

    fn get_origin(&self) -> &evident::event::origin::Origin {
        &self.origin
    }
}
