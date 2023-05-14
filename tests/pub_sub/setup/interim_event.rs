use evident::event::{entry::EventEntry, intermediary::IntermediaryEvent};

use super::{entry::MinEventEntry, id::MinId};

pub struct MinInterimEvent {
    entry: MinEventEntry,
}

impl IntermediaryEvent<MinId, MinEventEntry> for MinInterimEvent {
    fn new(
        event_id: MinId,
        msg: &str,
        crate_name: &'static str,
        filename: &str,
        line_nr: u32,
        module_path: &str,
    ) -> Self {
        MinInterimEvent {
            entry: MinEventEntry::new(event_id, msg, crate_name, filename, line_nr, module_path),
        }
    }

    fn get_entry(&self) -> &MinEventEntry {
        &self.entry
    }

    fn take_entry(&mut self) -> MinEventEntry {
        std::mem::take(&mut self.entry)
    }
}
