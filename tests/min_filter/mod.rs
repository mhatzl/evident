use self::{entry::MinEventEntry, filter::MinFilter, id::MinId, interim_event::MinInterimEvent};

mod entry;
mod filter;
mod id;
mod interim_event;

evident::create_static_publisher!(
    PUBLISHER,
    MinId,
    MinEventEntry,
    MinInterimEvent,
    filter_type = MinFilter,
    filter = MinFilter::default(),
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
fn setup_minimal_filtered_publisher() {
    let allowed_id = MinId { id: 2 };
    let filtered_id = MinId { id: 3 };
    let msg = "Some msg";

    let sub = PUBLISHER
        .subscribe_to_many(vec![filtered_id, allowed_id])
        .unwrap();

    // This event is not captured
    set_event!(filtered_id, msg).finalize();

    // This event is captured
    set_event!(allowed_id, msg).finalize();

    let event = sub
        .get_receiver()
        .recv_timeout(std::time::Duration::from_millis(100))
        .unwrap();

    assert_eq!(event.get_id(), &allowed_id, "Allowed Id was not captured.");
}
