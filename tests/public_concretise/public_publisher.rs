use crate::public_concretise::{entry::MinEventEntry, id::MinId, interim_event::MinInterimEvent};

// Note: **pub** visibility modifier before the name of the publisher.
// Note: **non_blocking = false** will block on `finalize()` (or implicitly on `drop`) until publisher received the event.
evident::create_static_publisher!(
    pub PUB_PUBLISHER,
    MinId,
    MinEventEntry,
    MinInterimEvent,
    CAPTURE_CHANNEL_BOUND = 1,
    SUBSCRIPTION_CHANNEL_BOUND = 1,
    non_blocking = false
);

// Note: Fully qualified path to access the generated `set_event!()` macro from anywhere.
evident::create_set_event_macro!(
    crate::public_concretise::MinId,
    crate::public_concretise::entry::MinEventEntry,
    crate::public_concretise::interim_event::MinInterimEvent
);
