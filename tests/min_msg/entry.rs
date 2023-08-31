//! This module contains the minimal required implementation for the [`EventEntry`] trait.
//!
//! [req:qa.ux.usage](https://github.com/mhatzl/evident/wiki/5-REQ-qa.ux.usage#qauxusage-provide-usage-examples)

use evident::event::{entry::EventEntry, origin::Origin};

use super::{id::MinId, msg::MinMsg};

#[derive(Default, Clone)]
pub struct MinEventEntry {
    event_id: MinId,
    msg: Option<MinMsg>,
    entry_id: evident::uuid::Uuid,
    origin: Origin,
}

impl EventEntry<MinId, MinMsg> for MinEventEntry {
    fn new(event_id: MinId, msg: Option<impl Into<MinMsg>>, origin: Origin) -> Self {
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

    fn get_msg(&self) -> Option<&MinMsg> {
        self.msg.as_ref()
    }

    fn get_origin(&self) -> &evident::event::origin::Origin {
        &self.origin
    }
}
