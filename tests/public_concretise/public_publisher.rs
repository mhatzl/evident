//! This module creates a public [`EvidentPublisher`](evident::publisher::EvidentPublisher),
//! and the `set_event!()` macros for this publisher.
//!
//! [req:qa.ux.usage]

use evident::publisher::{CaptureMode, EventTimestampKind};

use crate::public_concretise::{entry::MinEventEntry, id::MinId, interim_event::MinInterimEvent};

// Note: **pub** visibility modifier before the name of the publisher.
// Note: **non_blocking = false** will block on `finalize()` (or implicitly on `drop`) until publisher received the event.
evident::create_static_publisher!(
    pub PUB_PUBLISHER,
    id_type = MinId,
    msg_type = String,
    entry_type = MinEventEntry,
    interm_event_type = MinInterimEvent,
    capture_channel_bound = 1,
    subscription_channel_bound = 1,
    capture_mode = CaptureMode::Blocking,
    timestamp_kind = EventTimestampKind::Captured
);

// Note: Fully qualified path to access the generated `set_event!()` macro from anywhere.
evident::create_set_event_macro!(
    id_type = crate::public_concretise::MinId,
    msg_type = String,
    entry_type = crate::public_concretise::entry::MinEventEntry,
    interm_event_type = crate::public_concretise::interim_event::MinInterimEvent
);
