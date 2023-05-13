use evident::event::intermediary::IntermediaryEvent;
use once_cell::sync::Lazy;

use self::{entry::MinEventEntry, id::MinId, interim_event::MinInterimEvent};

pub mod entry;
pub mod id;
pub mod interim_event;

evident::create_on_event!(PUBLISHER, MinId, MinEventEntry);
evident::create_set_event!(MinId, MinEventEntry, MinInterimEvent);


fn _new_min_publisher() -> evident::publisher::EvidentPublisher<MinId, MinEventEntry, 1, 1> {
    evident::publisher::EvidentPublisher::<MinId, MinEventEntry, 1, 1>::new(self::on_event)
}

static PUBLISHER: Lazy<evident::publisher::EvidentPublisher<MinId, MinEventEntry, 1, 1>> =
    Lazy::new(_new_min_publisher);

#[test]
fn setup_minimal_publisher() {
    let some_id = MinId {
        id: 3,
    };
    let msg = "Some msg";

    let sub = PUBLISHER.subscribe(&some_id).unwrap();

    set_event!(some_id, msg).finalize();

    let event = sub.get_receiver().recv_timeout(std::time::Duration::from_millis(100)).unwrap();

    assert_eq!(event.get_id(), &some_id, "Sent and received Ids differ.");
}
