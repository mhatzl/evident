use evident::publisher::{CaptureMode, EventTimestampKind};

use self::{entry::MinEventEntry, id::MinId, interim_event::MinInterimEvent};

mod entry;
mod id;
mod interim_event;

evident::create_static_publisher!(
    PUBLISHER,
    id_type = MinId,
    entry_type = MinEventEntry,
    interm_event_type = MinInterimEvent,
    capture_channel_bound = 1,
    subscription_channel_bound = 1,
    capture_mode = CaptureMode::Blocking,
    timestamp_kind = EventTimestampKind::Created
);

// Note: **no_export** to prevent the macro from adding `#[macro_export]`.
evident::create_set_event_macro!(
    no_export,
    id_type = MinId,
    entry_type = MinEventEntry,
    interm_event_type = MinInterimEvent
);

#[test]
fn setup_minimal_publisher() {
    let some_id = MinId { id: 3 };
    let msg = "Some msg";

    let sub = PUBLISHER.subscribe(some_id).unwrap();

    set_event!(some_id, msg).finalize();

    let event = sub
        .get_receiver()
        .recv_timeout(std::time::Duration::from_millis(100))
        .unwrap();

    assert_eq!(
        event.get_event_id(),
        &some_id,
        "Sent and received Ids differ."
    );
}
