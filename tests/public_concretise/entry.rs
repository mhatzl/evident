use evident::event::{entry::EventEntry, origin::Origin};

use super::id::MinId;

#[derive(Default, Clone)]
pub struct MinEventEntry {
    event_id: MinId,
    msg: String,

    entry_id: evident::uuid::Uuid,
    origin: Origin,
}

impl EventEntry<MinId> for MinEventEntry {
    fn new(
        event_id: MinId,
        msg: &str,
        crate_name: &'static str,
        module_path: &'static str,
        filename: &'static str,
        line_nr: u32,
    ) -> Self {
        MinEventEntry {
            event_id,
            msg: msg.to_string(),

            entry_id: evident::uuid::Uuid::new_v4(),
            origin: Origin::new(crate_name, module_path, filename, line_nr),
        }
    }

    fn get_event_id(&self) -> &MinId {
        &self.event_id
    }

    fn get_entry_id(&self) -> evident::uuid::Uuid {
        self.entry_id
    }

    fn get_msg(&self) -> &str {
        &self.msg
    }

    fn get_crate_name(&self) -> &'static str {
        &self.origin.crate_name
    }

    fn get_origin(&self) -> &evident::event::origin::Origin {
        &self.origin
    }
}
