use crate::public_concretise::public_publisher::PUB_PUBLISHER;

use self::id::MinId;

mod entry;
mod id;
mod interim_event;

#[macro_use]
mod public_publisher;

#[test]
fn setup_minimal_public_publisher() {
    let some_id = MinId { id: 3 };
    let msg = "Some msg";

    let sub = PUB_PUBLISHER.subscribe(&some_id).unwrap();

    set_event!(some_id, msg).finalize();

    let event = sub
        .get_receiver()
        .recv_timeout(std::time::Duration::from_millis(100))
        .unwrap();

    assert_eq!(event.get_id(), &some_id, "Sent and received Ids differ.");
}
