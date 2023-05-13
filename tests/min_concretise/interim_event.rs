use evident::event::{intermediary::IntermediaryEvent, entry::EventEntry};

use super::{id::MinId, entry::MinEventEntry, PUBLISHER};


pub struct MinInterimEvent {
    entry: MinEventEntry,
}

impl IntermediaryEvent<MinId, MinEventEntry> for MinInterimEvent {
    fn new(
        event_id: MinId,
        msg: &str,
        crate_name: &str,
        filename: &str,
        line_nr: u32,
        module_path: &str,
    ) -> Self {
        MinInterimEvent { entry: MinEventEntry::new(event_id, msg, crate_name, filename, line_nr, module_path) }
    }

    fn get_entry(&self) -> &MinEventEntry {
        &self.entry
    }

    fn take_entry(&mut self) -> MinEventEntry {
        std::mem::take(&mut self.entry)
    }
}

evident::try_capture_drop!(PUBLISHER, MinInterimEvent);

