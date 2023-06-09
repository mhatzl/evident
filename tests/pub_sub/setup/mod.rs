use evident::publisher::{CaptureMode, EventTimestampKind};

use self::{entry::MinEventEntry, id::MinId, interim_event::MinInterimEvent};

pub mod entry;
pub mod id;
pub mod interim_event;

evident::create_static_publisher!(
    pub TESTS_PUBLISHER,
    id_type = MinId,
    msg_type = String,
    entry_type = MinEventEntry,
    interm_event_type = MinInterimEvent,
    capture_channel_bound = 500,
    subscription_channel_bound = 500,
    capture_mode = CaptureMode::Blocking,
    timestamp_kind = EventTimestampKind::Captured
);

evident::create_set_event_macro!(
    no_export,
    id_type = crate::pub_sub::setup::id::MinId,
    msg_type = String,
    entry_type = crate::pub_sub::setup::entry::MinEventEntry,
    interm_event_type = crate::pub_sub::setup::interim_event::MinInterimEvent
);
