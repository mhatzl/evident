use self::{entry::MinEventEntry, id::MinId, interim_event::MinInterimEvent};

mod entry;
mod id;
mod interim_event;

evident::create_static_publisher!(
    PUBLISHER,
    MinId,
    MinEventEntry,
    MinInterimEvent,
    CAPTURE_CHANNEL_BOUND = 1,
    SUBSCRIPTION_CHANNEL_BOUND = 1,
    non_blocking = true
);

// Note: **no_export** to prevent the macro from adding `#[macro_export]`.
evident::create_set_event_macro!(
    no_export
    MinId,
    MinEventEntry,
    MinInterimEvent
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

    assert_eq!(event.get_id(), &some_id, "Sent and received Ids differ.");
}
