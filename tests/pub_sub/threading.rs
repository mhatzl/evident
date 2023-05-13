use std::thread;

use crate::pub_sub::setup::id::MinId;

use super::setup::TESTS_PUBLISHER;

#[test]
fn set_different_events_in_two_threads() {
    let id_side = MinId { id: 1 };
    let msg_side = "Set side thread message";
    let id_main = MinId { id: 2 };
    let msg_main = "Set main thread message";

    let recv_side = TESTS_PUBLISHER.subscribe(&id_side).unwrap();
    let recv_main = TESTS_PUBLISHER.subscribe(&id_main).unwrap();

    let side_thread = thread::spawn(move || {
        set_event!(id_side, msg_side).finalize();
    });

    set_event!(id_main, msg_main).finalize();

    assert!(side_thread.join().is_ok(), "Side thread panicked.");

    let event_side = recv_side
        .get_receiver()
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();

    assert_eq!(
        event_side.get_id(),
        &id_side,
        "Received side event has wrong LogId."
    );
    assert_eq!(
        event_side.get_msg(),
        msg_side,
        "Received side event has wrong msg."
    );

    let event_main = recv_main
        .get_receiver()
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();
    assert_eq!(
        event_main.get_id(),
        &id_main,
        "Received main event has wrong LogId."
    );
    assert_eq!(
        event_main.get_msg(),
        msg_main,
        "Received main event has wrong msg."
    );
}
