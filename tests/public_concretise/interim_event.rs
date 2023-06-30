use evident::event::{entry::EventEntry, intermediary::IntermediaryEvent, origin::Origin};

use super::{entry::MinEventEntry, id::MinId};

pub struct MinInterimEvent {
    entry: MinEventEntry,
}

impl IntermediaryEvent<MinId, MinEventEntry> for MinInterimEvent {
    fn new(event_id: MinId, msg: &str, origin: Origin) -> Self {
        MinInterimEvent {
            entry: MinEventEntry::new(event_id, msg, origin),
        }
    }

    fn get_entry(&self) -> &MinEventEntry {
        &self.entry
    }

    fn set_entry_nr(&mut self, entry_nr: u64) {
        self.entry.entry_nr = entry_nr;
    }

    fn take_entry(&mut self) -> MinEventEntry {
        std::mem::take(&mut self.entry)
    }
}
