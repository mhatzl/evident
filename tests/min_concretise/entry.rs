//! This module contains the minimal required implementation for the [`EventEntry`] trait.
//!
//! [<req>qa.ux.usage]

use evident::event::{entry::EventEntry, origin::Origin};

use super::id::MinId;

/// Struct used for a minimal [`EventEntry`] trait implementation.
#[derive(Default, Clone)]
pub struct MinEventEntry {
    event_id: MinId,
    msg: Option<String>,
    entry_id: evident::uuid::Uuid,
    origin: Origin,
}

impl EventEntry<MinId, String> for MinEventEntry {
    fn new(event_id: MinId, msg: Option<impl Into<String>>, origin: Origin) -> Self {
        MinEventEntry {
            event_id,
            msg: msg.map(|m| m.into()),
            entry_id: evident::uuid::Uuid::new_v4(),
            origin,
        }
    }

    fn get_event_id(&self) -> &MinId {
        &self.event_id
    }

    fn into_event_id(self) -> MinId {
        self.event_id
    }

    fn get_entry_id(&self) -> evident::uuid::Uuid {
        self.entry_id
    }

    fn get_msg(&self) -> Option<&String> {
        self.msg.as_ref()
    }

    fn get_origin(&self) -> &evident::event::origin::Origin {
        &self.origin
    }
}
