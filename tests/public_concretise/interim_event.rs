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
        module_path: &'static str,
        filename: &'static str,
        line_nr: u32,
    ) -> Self {
        MinInterimEvent {
            entry: MinEventEntry::new(event_id, msg, crate_name, module_path, filename, line_nr),
        }
    }

    fn get_entry(&self) -> &MinEventEntry {
        &self.entry
    }

    fn take_entry(&mut self) -> MinEventEntry {
        std::mem::take(&mut self.entry)
    }
}
