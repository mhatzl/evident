use crate::min_filter::id::STOP_CAPTURING;

use self::{entry::MinEventEntry, filter::MinFilter, id::MinId, interim_event::MinInterimEvent};

mod entry;
mod filter;
mod id;
mod interim_event;

evident::create_static_publisher!(
    PUBLISHER,
    id_type = MinId,
    entry_type = MinEventEntry,
    interm_event_type = MinInterimEvent,
    filter_type = MinFilter,
    filter = MinFilter::default(),
    capture_channel_bound = 1,
    subscription_channel_bound = 1,
    non_blocking = true
);

// Note: **no_export** to prevent the macro from adding `#[macro_export]`.
evident::create_set_event_macro!(
    no_export,
    id_type = MinId,
    entry_type = MinEventEntry,
    interm_event_type = MinInterimEvent
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

#[test]
fn stop_capturing_event_not_filtered() {
    let msg = "Some msg";

    let sub = PUBLISHER.subscribe(STOP_CAPTURING).unwrap();

    // Make sure event is captured, even though filter would not allow id
    set_event!(STOP_CAPTURING, msg).finalize();

    let event = sub
        .get_receiver()
        .recv_timeout(std::time::Duration::from_millis(100))
        .unwrap();

    assert_eq!(
        event.get_id(),
        &STOP_CAPTURING,
        "Stop capturing event was filtered."
    );
}
