//! Contains tests for the `set_event!()` macro.

use std::cmp::Ordering;

use evident::event::origin::Origin;

use crate::pub_sub::setup::{
    entry::MinEventEntry, id::MinId, interim_event::MinInterimEvent, TESTS_PUBLISHER,
};

/// [req:event.origin.test.basic](https://github.com/mhatzl/evident/wiki/5-REQ-event.origin#eventorigintestbasic-origin-of-set-event-points-to-set_event-call)
#[test]
fn set_event_has_correct_origin() {
    let id = MinId { id: 1 };
    let msg = "Set first message.";

    let recv = TESTS_PUBLISHER.subscribe(id).unwrap();

    let line_nr = line!() + 1;
    set_event!(id, msg).finalize();

    let event = recv
        .get_receiver()
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();

    assert_eq!(*event.get_event_id(), id, "Ids are not equal.");
    assert_eq!(
        event.get_origin().line_nr,
        line_nr,
        "Line numbers are not equal."
    );
    assert_eq!(
        event.get_origin().filename,
        file!(),
        "Filenames are not equal."
    );
    assert_eq!(
        event.get_origin().module_path,
        module_path!(),
        "Module paths are not equal."
    );
    assert_eq!(
        event.get_origin().to_string(),
        format!(
            "module=\"{}\", file=\"{}\", line={}",
            module_path!(),
            file!(),
            line_nr
        ),
        "Module paths are not equal."
    );
}

/// [req:event.origin.test.two_origins](https://github.com/mhatzl/evident/wiki/5-REQ-event.origin#eventorigintesttwo_origins-origin-of-an-event-set-with-two-set_event-calls-differs)
#[test]
fn set_same_event_twice_with_different_origin() {
    let id = MinId { id: 1 };
    let msg = "Set first message.";

    let recv = TESTS_PUBLISHER.subscribe(id).unwrap();

    let line_1 = line!() + 1;
    set_event!(id, msg).finalize();

    let line_2 = line!() + 1;
    set_event!(id, msg).finalize();

    let event_1 = recv
        .get_receiver()
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();
    let event_2 = recv
        .get_receiver()
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();

    assert_eq!(
        *event_1.get_event_id(),
        id,
        "Set and stored ids are not equal."
    );
    assert_eq!(
        event_1.get_origin().line_nr,
        line_1,
        "Set and stored line numbers are not equal."
    );

    assert_eq!(
        *event_2.get_event_id(),
        id,
        "Set and stored ids are not equal."
    );
    assert_eq!(
        event_2.get_origin().line_nr,
        line_2,
        "Set and stored line numbers are not equal."
    );

    assert_eq!(
        *event_1.get_event_id(),
        *event_2.get_event_id(),
        "Events do not have the same id."
    );
}

/// [req:event.origin.test.same_origin](https://github.com/mhatzl/evident/wiki/5-REQ-event.origin#eventorigintestsame_origin-set-origin-of-an-event-manually-for-two-set_event_with_msg-calls)
#[test]
fn set_same_event_twice_with_same_origin() {
    let id = MinId { id: 1 };
    let msg = "Set first message";
    let line = line!();

    let recv = TESTS_PUBLISHER.subscribe(id).unwrap();

    evident::event::set_event_with_msg::<MinId, String, MinEventEntry, MinInterimEvent>(
        id,
        msg,
        Origin::new(module_path!(), file!(), line),
    )
    .finalize();
    evident::event::set_event_with_msg::<MinId, String, MinEventEntry, MinInterimEvent>(
        id,
        msg,
        Origin::new(module_path!(), file!(), line),
    )
    .finalize();

    let event_1 = recv
        .get_receiver()
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();
    let event_2 = recv
        .get_receiver()
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();

    assert_eq!(*event_1.get_event_id(), id, "Ids are not equal.");
    assert_eq!(
        event_1.get_origin().line_nr,
        line,
        "Line numbers are not equal."
    );

    assert_eq!(*event_2.get_event_id(), id, "Ids are not equal.");
    assert_eq!(
        event_2.get_origin().line_nr,
        line,
        "Line numbers are not equal."
    );

    assert_ne!(event_1, event_2, "Received events are equal.");
}

/// [req:subs.test.mult_subs](https://github.com/mhatzl/evident/wiki/5-REQ-subs#substestmult_subs-multiple-subscribers-to-same-event)
#[test]
fn set_event_received_exactly_once_per_receiver() {
    let id = MinId { id: 1 };
    let msg = "Set event message";

    let recv_1 = TESTS_PUBLISHER.subscribe(id).unwrap();
    let recv_2 = TESTS_PUBLISHER.subscribe(id).unwrap();

    set_event!(id, msg).finalize();

    let recv_1_event_1 = recv_1
        .get_receiver()
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();

    assert_eq!(
        recv_1_event_1.get_msg().unwrap(),
        msg,
        "Event messages are not equal."
    );

    let recv_1_event_2 = recv_1
        .get_receiver()
        .recv_timeout(std::time::Duration::from_millis(10));

    assert!(
        recv_1_event_2.is_err(),
        "Second event received, but only one was set."
    );

    let recv_2_event_1 = recv_2
        .get_receiver()
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();

    assert_eq!(
        recv_2_event_1.get_msg().unwrap(),
        msg,
        "Event messages are not equal."
    );

    let recv_2_event_2 = recv_2
        .get_receiver()
        .recv_timeout(std::time::Duration::from_millis(10));

    assert!(
        recv_2_event_2.is_err(),
        "Second event received, but only one was set."
    );
}

#[test]
fn set_event_with_literal_msg() {
    let id = MinId { id: 1 };

    let recv = TESTS_PUBLISHER.subscribe(id).unwrap();

    set_event!(id, "Set event message").finalize();

    let event = recv
        .get_receiver()
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();

    assert_eq!(
        event.get_msg().unwrap(),
        "Set event message",
        "Event messages are not equal."
    );
}

#[test]
fn set_event_using_msg_expression() {
    let id = MinId { id: 1 };

    let recv = TESTS_PUBLISHER.subscribe(id).unwrap();

    set_event!(id, &format!("Set message with id={}", id)).finalize();

    let event = recv
        .get_receiver()
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();

    assert_eq!(
        event.get_msg().unwrap(),
        &format!("Set message with id={}", id),
        "Event messages are not equal."
    );
}

#[derive(Clone)]
enum TestLogId {
    Id = 1,
}

impl From<TestLogId> for MinId {
    fn from(value: TestLogId) -> Self {
        MinId {
            id: (value as isize),
        }
    }
}

impl From<MinId> for TestLogId {
    fn from(value: MinId) -> Self {
        match value {
            v if v.id == (TestLogId::Id as isize) => TestLogId::Id,
            _ => unimplemented!(),
        }
    }
}

#[test]
fn set_event_with_enum() {
    let msg = "Set first message";

    let recv = TESTS_PUBLISHER.subscribe(TestLogId::Id.into()).unwrap();

    set_event!(TestLogId::Id.into(), msg).finalize();

    let event = recv
        .get_receiver()
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();

    assert_eq!(
        *event.get_event_id(),
        TestLogId::Id.into(),
        "Ids are not equal"
    );
}

#[test]
fn set_event_has_current_thread_id() {
    let msg = "Set first message";

    let recv = TESTS_PUBLISHER.subscribe(TestLogId::Id.into()).unwrap();
    let thread_id = std::thread::current().id();

    set_event!(TestLogId::Id.into(), msg).finalize();

    let event = recv
        .get_receiver()
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();

    assert_eq!(*event.get_thread_id(), thread_id, "ThreadIds are not equal");
}

#[test]
fn spawned_set_event_has_different_thread_id() {
    let recv = TESTS_PUBLISHER.subscribe(TestLogId::Id.into()).unwrap();
    let thread_id = std::thread::current().id();

    std::thread::spawn(|| {
        set_event!(TestLogId::Id.into(), "Set event message").finalize();
    });

    std::thread::sleep(std::time::Duration::from_millis(10));

    let event = recv
        .get_receiver()
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();

    assert_ne!(*event.get_thread_id(), thread_id, "ThreadIds are equal");
}

#[test]
fn datetime_of_second_event_is_greater() {
    let id = MinId { id: 1 };
    let msg = "Set first message.";

    let recv = TESTS_PUBLISHER.subscribe(id).unwrap();

    set_event!(id, msg).finalize();
    set_event!(id, msg).finalize();

    let event_1 = recv
        .get_receiver()
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();
    let event_2 = recv
        .get_receiver()
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();

    assert_eq!(
        event_2
            .get_timestamp()
            .unwrap()
            .cmp(&event_1.get_timestamp().unwrap()),
        Ordering::Greater,
        "Datetime of second event is not greater than first event."
    );
}
