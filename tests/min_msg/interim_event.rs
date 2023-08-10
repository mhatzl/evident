//! This module contains the minimal required implementation for the [`IntermediaryEvent`] trait.
//!
//! [<req>qa.ux.usage]

use evident::event::{entry::EventEntry, intermediary::IntermediaryEvent, origin::Origin};

use super::{entry::MinEventEntry, id::MinId, msg::MinMsg};

pub struct MinInterimEvent {
    entry: MinEventEntry,
}

impl IntermediaryEvent<MinId, MinMsg, MinEventEntry> for MinInterimEvent {
    fn new(event_id: MinId, msg: Option<impl Into<MinMsg>>, origin: Origin) -> Self {
        MinInterimEvent {
            entry: MinEventEntry::new(event_id, msg, origin),
        }
    }

    fn get_entry(&self) -> &MinEventEntry {
        &self.entry
    }

    fn take_entry(&mut self) -> MinEventEntry {
        std::mem::take(&mut self.entry)
    }
}
