use crate::pub_sub::setup::{id::MinId, TESTS_PUBLISHER};

#[test]
fn two_ids_separate_receiver() {
    let id_1 = MinId { id: 1 };
    let msg_1 = "Set first message";
    let id_2 = MinId { id: 2 };
    let msg_2 = "Set second message";

    let recv_1 = TESTS_PUBLISHER.subscribe(id_1).unwrap();
    let recv_2 = TESTS_PUBLISHER.subscribe(id_2).unwrap();

    set_event!(id_1, msg_1).finalize();
    set_event!(id_2, msg_2).finalize();

    let event_1 = recv_1
        .get_receiver()
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();
    assert_eq!(
        event_1.get_event_id(),
        &id_1,
        "Received event 1 has wrong Id."
    );
    assert_eq!(
        event_1.get_msg().unwrap(),
        msg_1,
        "Received event 1 has wrong msg."
    );

    let event_2 = recv_2
        .get_receiver()
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();
    assert_eq!(
        event_2.get_event_id(),
        &id_2,
        "Received event 2 has wrong Id."
    );
    assert_eq!(
        event_2.get_msg().unwrap(),
        msg_2,
        "Received event 2 has wrong msg."
    );
}

#[test]
fn one_id_separate_receiver() {
    let id = MinId { id: 1 };
    let msg = "Set message";

    let recv_1 = TESTS_PUBLISHER.subscribe(id).unwrap();
    let recv_2 = TESTS_PUBLISHER.subscribe(id).unwrap();

    set_event!(id, msg).finalize();

    let event_1 = recv_1
        .get_receiver()
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();
    assert_eq!(
        event_1.get_event_id(),
        &id,
        "Received event 1 has wrong Id."
    );
    assert_eq!(
        event_1.get_msg().unwrap(),
        msg,
        "Received event 1 has wrong msg."
    );

    let event_2 = recv_2
        .get_receiver()
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();
    assert_eq!(
        event_2.get_event_id(),
        &id,
        "Received event 2 has wrong Id."
    );
    assert_eq!(
        event_2.get_msg().unwrap(),
        msg,
        "Received event 2 has wrong msg."
    );

    assert_eq!(event_1, event_2, "Received events are not equal.");
}

#[test]
fn subscribe_to_two_ids_at_once() {
    let id_1 = MinId { id: 1 };
    let msg_1 = "Set first message";
    let id_2 = MinId { id: 2 };
    let msg_2 = "Set second message";

    let recv = TESTS_PUBLISHER.subscribe_to_many(vec![id_1, id_2]).unwrap();

    set_event!(id_1, msg_1).finalize();
    set_event!(id_2, msg_2).finalize();

    let event_1 = recv
        .get_receiver()
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();
    assert!(
        event_1.get_event_id() == &id_1 || event_1.get_event_id() == &id_2,
        "Received event 1 has wrong Id."
    );
    assert!(
        event_1.get_msg().unwrap() == msg_1 || event_1.get_msg().unwrap() == msg_2,
        "Received event 1 has wrong msg."
    );

    let event_2 = recv
        .get_receiver()
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();
    assert!(
        event_2.get_event_id() == &id_1 || event_2.get_event_id() == &id_2,
        "Received event 2 has wrong Id."
    );
    assert!(
        event_2.get_msg().unwrap() == msg_1 || event_2.get_msg().unwrap() == msg_2,
        "Received event 2 has wrong msg."
    );
    assert_ne!(
        event_1.get_event_id(),
        event_2.get_event_id(),
        "Both events received the same id."
    );
}

/// [req:cap.test.mult]
#[test]
fn receiver_for_all_events_two_events_set() {
    let id_1 = MinId { id: 1 };
    let msg_1 = "Set first message";
    let id_2 = MinId { id: 2 };
    let msg_2 = "Set second message";

    let recv_all = TESTS_PUBLISHER.subscribe_to_all_events().unwrap();

    set_event!(id_1, msg_1).finalize();
    set_event!(id_2, msg_2).finalize();

    let event_1 = recv_all
        .get_receiver()
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();
    assert_eq!(
        event_1.get_event_id(),
        &id_1,
        "Received event 1 has wrong Id."
    );
    assert_eq!(
        event_1.get_msg().unwrap(),
        msg_1,
        "Received event 1 has wrong msg."
    );

    let event_2 = recv_all
        .get_receiver()
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();
    assert_eq!(
        event_2.get_event_id(),
        &id_2,
        "Received event 2 has wrong Id."
    );
    assert_eq!(
        event_2.get_msg().unwrap(),
        msg_2,
        "Received event 2 has wrong msg."
    );
}

#[test]
fn receiver_unsubscribes_single_id() {
    let id_1 = MinId { id: 1 };
    let msg_1 = "Set first message";
    let id_2 = MinId { id: 2 };

    let mut recv = TESTS_PUBLISHER.subscribe_to_many(vec![id_1, id_2]).unwrap();

    set_event!(id_1, msg_1).finalize();

    let event_1 = recv
        .get_receiver()
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();
    assert_eq!(
        event_1.get_event_id(),
        &id_1,
        "Received event 1 has wrong Id."
    );
    assert_eq!(
        event_1.get_msg().unwrap(),
        msg_1,
        "Received event 1 has wrong msg."
    );

    recv.unsubscribe_id(id_1).unwrap();

    set_event!(id_1, msg_1).finalize();

    let event_2 = recv
        .get_receiver()
        .recv_timeout(std::time::Duration::from_millis(10));

    assert!(event_2.is_err(), "Unsubscribed event received.");
}

#[test]
fn unsubscribe_of_last_left_subscription() {
    let id = MinId { id: 1 };

    let mut recv = TESTS_PUBLISHER.subscribe(id).unwrap();
    let res = recv.unsubscribe_id(id);

    assert!(
        res.is_err(),
        "Unsubscribe of last left subscription allowed."
    );
}

#[test]
fn receiver_subscribes_to_new_id() {
    let id_1 = MinId { id: 1 };
    let msg_1 = "Set first message";
    let id_2 = MinId { id: 2 };
    let msg_2 = "Set second message";

    let mut recv = TESTS_PUBLISHER.subscribe(id_1).unwrap();

    set_event!(id_1, msg_1).finalize();
    set_event!(id_2, "Id not yet subscribed").finalize();

    let event_1 = recv
        .get_receiver()
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();
    assert_eq!(
        event_1.get_event_id(),
        &id_1,
        "Received event 1 has wrong Id."
    );
    assert_eq!(
        event_1.get_msg().unwrap(),
        msg_1,
        "Received event 1 has wrong msg."
    );

    // Note: Sleep guarantees that `on_event` thread processed all previous events.
    std::thread::sleep(std::time::Duration::from_millis(100));

    recv.subscribe_id(id_2).unwrap();

    set_event!(id_2, msg_2).finalize();

    let event_2 = recv
        .get_receiver()
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();

    assert_eq!(
        event_2.get_event_id(),
        &id_2,
        "Received event 2 has wrong Id."
    );
    assert_eq!(
        event_2.get_msg().unwrap(),
        msg_2,
        "Received event 2 has wrong msg."
    );
}

#[test]
fn receiver_for_all_events_tries_unsubscribe_id() {
    let id = MinId { id: 1 };
    let mut recv_all = TESTS_PUBLISHER.subscribe_to_all_events().unwrap();

    let res = recv_all.unsubscribe_id(id);

    assert!(
        res.is_err(),
        "All events subscription allowed unsubscribe of Id."
    );
}

#[test]
fn receiver_for_all_events_tries_subscribe_id() {
    let id = MinId { id: 1 };
    let mut recv_all = TESTS_PUBLISHER.subscribe_to_all_events().unwrap();

    let res = recv_all.subscribe_id(id);

    assert!(
        res.is_err(),
        "All events subscription allowed subscription to Id."
    );
}
