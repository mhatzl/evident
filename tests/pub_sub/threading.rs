use std::thread;

use crate::pub_sub::setup::id::MinId;

use super::setup::TESTS_PUBLISHER;

#[test]
fn set_different_events_in_two_threads() {
    let id_side = MinId { id: 1 };
    let msg_side = "Set side thread message";
    let id_main = MinId { id: 2 };
    let msg_main = "Set main thread message";

    let recv_side = TESTS_PUBLISHER.subscribe(id_side).unwrap();
    let recv_main = TESTS_PUBLISHER.subscribe(id_main).unwrap();

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
        event_side.get_event_id(),
        &id_side,
        "Received side event has wrong Id."
    );
    assert_eq!(
        event_side.get_msg().unwrap(),
        msg_side,
        "Received side event has wrong msg."
    );

    let event_main = recv_main
        .get_receiver()
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();
    assert_eq!(
        event_main.get_event_id(),
        &id_main,
        "Received main event has wrong Id."
    );
    assert_eq!(
        event_main.get_msg().unwrap(),
        msg_main,
        "Received main event has wrong msg."
    );
}

#[test]
fn set_same_event_in_two_threads() {
    let id = MinId { id: 1 };
    let msg_side = "Set side thread message";
    let msg_main = "Set main thread message";

    let recv = TESTS_PUBLISHER.subscribe(id).unwrap();

    let side_thread = thread::spawn(move || {
        set_event!(id, msg_side).finalize();
    });

    set_event!(id, msg_main).finalize();

    assert!(side_thread.join().is_ok(), "Side thread panicked.");

    let event_1 = recv
        .get_receiver()
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();
    assert_eq!(
        event_1.get_event_id(),
        &id,
        "Received event 1 has wrong Id."
    );
    assert!(
        event_1.get_msg().unwrap() == msg_main || event_1.get_msg().unwrap() == msg_side,
        "Received event 1 has wrong msg."
    );

    let event_2 = recv
        .get_receiver()
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();
    assert_eq!(
        event_2.get_event_id(),
        &id,
        "Received event 2 has wrong Id."
    );
    assert!(
        event_2.get_msg().unwrap() == msg_main || event_2.get_msg().unwrap() == msg_side,
        "Received event 2 has wrong msg."
    );

    assert_ne!(
        event_1.get_msg(),
        event_2.get_msg(),
        "Both events have the same msg."
    );
}

#[test]
fn set_events_in_many_threads() {
    // Note: This value should be 2x lower than the channel bounds set for the publisher.
    // 2x lower is to make sure that the channel buffer is not the reason for this test to fail.
    const THREAD_CNT: isize = 100;
    let base_id = MinId { id: 1 };
    let msg = "Set event message";

    let mut recvs = Vec::new();
    for i in 1..=THREAD_CNT {
        let loop_id = MinId { id: i };
        recvs.push(TESTS_PUBLISHER.subscribe(loop_id).unwrap());
    }

    set_event!(base_id, msg).finalize();

    rayon::scope(|s| {
        // start at 2 to jump over base_id
        for i in 2..=THREAD_CNT {
            s.spawn(move |_| {
                let loop_id = MinId { id: i };

                // Note: `finalize()` would not be needed, since events are finalized on drop, but it makes this test easier to read
                set_event!(base_id, msg).finalize();
                set_event!(loop_id, msg).finalize();
            });
        }
    });

    for i in 1..=THREAD_CNT {
        let id = MinId { id: i };

        let event = recvs[(i - 1) as usize]
            .get_receiver()
            .recv_timeout(std::time::Duration::from_millis(10))
            .unwrap();
        assert_eq!(
            event.get_event_id(),
            &id,
            "Received event {} has wrong Id.",
            i
        );
    }

    // Note: Starting at "2", because one recv was already consumed in loop above
    for i in 2..=THREAD_CNT {
        let event = recvs[0]
            .get_receiver()
            .recv_timeout(std::time::Duration::from_millis(10))
            .unwrap();
        assert_eq!(
            event.get_event_id(),
            &base_id,
            "Received event {} has wrong Id.",
            i
        );
    }
}

/// [<req>pub.threaded.test]
#[test]
fn set_events_in_many_threads_for_one_subscriber() {
    // Note: This value should be at least 2x lower than the channel bounds set for the publisher.
    // 2x lower is to make sure that the channel buffer is not the reason for this test to fail.
    const THREAD_CNT: isize = 10;
    let base_id = MinId { id: 1 };
    let msg = "Set event message";

    let mut subs = TESTS_PUBLISHER.subscribe(base_id).unwrap();
    // start at 2 to jump over base_id
    for i in 2..=THREAD_CNT {
        let loop_id = MinId { id: i };
        subs.subscribe_id(loop_id).unwrap();
    }

    set_event!(base_id, msg).finalize();

    rayon::scope(|s| {
        // start at 2 to jump over base_id
        for i in 2..=THREAD_CNT {
            s.spawn(move |_| {
                let loop_id = MinId { id: i };

                // Note: `finalize()` would not be needed, since events are finalized on drop, but it makes this test easier to read
                set_event!(loop_id, msg).finalize();
            });
        }
    });

    // Note: IDs might be received in any order => capture all received events, and then check if all set events are received.

    let mut recv_ids = Vec::new();
    for _ in 1..=THREAD_CNT {
        let event = subs
            .get_receiver()
            .recv_timeout(std::time::Duration::from_millis(10))
            .unwrap();

        recv_ids.push(event.get_event_id().clone());
    }

    for i in 1..=THREAD_CNT {
        let id = MinId { id: i };

        assert!(recv_ids.contains(&id), "Received event {} has wrong Id.", i);
    }
}
