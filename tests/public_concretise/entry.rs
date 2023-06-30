use evident::event::{entry::EventEntry, origin::Origin};

use super::id::MinId;

#[derive(Default, Clone)]
pub struct MinEventEntry {
    pub(crate) entry_nr: u64,

    event_id: MinId,
    msg: String,
    origin: Origin,
}

impl EventEntry<MinId> for MinEventEntry {
    fn new(event_id: MinId, msg: &str, origin: Origin) -> Self {
        MinEventEntry {
            entry_nr: 0,

            event_id,
            msg: msg.to_string(),
            origin,
        }
    }

    fn get_event_id(&self) -> &MinId {
        &self.event_id
    }

    fn into_event_id(self) -> MinId {
        self.event_id
    }

    fn get_entry_nr(&self) -> u64 {
        self.entry_nr
    }

    fn get_msg(&self) -> &str {
        &self.msg
    }

    fn get_origin(&self) -> &evident::event::origin::Origin {
        &self.origin
    }
}
