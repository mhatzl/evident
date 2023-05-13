use self::{entry::MinEventEntry, id::MinId, interim_event::MinInterimEvent};

pub mod entry;
pub mod id;
pub mod interim_event;

evident::create_static_publisher!(
    pub TESTS_PUBLISHER,
    MinId,
    MinEventEntry,
    MinInterimEvent,
    CAPTURE_CHANNEL_BOUND = 1,
    SUBSCRIPTION_CHANNEL_BOUND = 1,
    non_blocking = false
);

evident::create_set_event_macro!(
    no_export
    crate::pub_sub::setup::id::MinId,
    crate::pub_sub::setup::entry::MinEventEntry,
    crate::pub_sub::setup::interim_event::MinInterimEvent
);
