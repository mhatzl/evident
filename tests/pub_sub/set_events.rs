use crate::pub_sub::setup::{
    entry::MinEventEntry, id::MinId, interim_event::MinInterimEvent, TESTS_PUBLISHER,
};

#[test]
fn set_event_has_correct_origin() {
    let id = MinId { id: 1 };
    let msg = "Set first log message.";

    let recv = TESTS_PUBLISHER.subscribe(&id).unwrap();

    let line_nr = line!() + 1;
    set_event!(id, msg).finalize();

    let event = recv
        .get_receiver()
        .recv_timeout(std::time::Duration::from_millis(10))
        .unwrap();

    assert_eq!(*event.get_id(), id, "Ids are not equal.");
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
        event.get_origin().crate_name,
        env!("CARGO_PKG_NAME"),
        "Crate names are not equal."
    );
    assert_eq!(
        event.get_origin().module_path,
        module_path!(),
        "Module paths are not equal."
    );
    assert_eq!(
        event.get_origin().to_string(),
        format!(
            "crate=\"{}\", module=\"{}\", file=\"{}\", line={}",
            env!("CARGO_PKG_NAME"),
            module_path!(),
            file!(),
            line_nr
        ),
        "Module paths are not equal."
    );
}

#[test]
fn set_same_event_twice_with_different_origin() {
    let id = MinId { id: 1 };
    let msg = "Set first log message.";

    let recv = TESTS_PUBLISHER.subscribe(&id).unwrap();

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

    assert_eq!(*event_1.get_id(), id, "Set and stored ids are not equal.");
    assert_eq!(
        event_1.get_origin().line_nr,
        line_1,
        "Set and stored line numbers are not equal."
    );

    assert_eq!(*event_2.get_id(), id, "Set and stored ids are not equal.");
    assert_eq!(
        event_2.get_origin().line_nr,
        line_2,
        "Set and stored line numbers are not equal."
    );

    assert_eq!(
        *event_1.get_id(),
        *event_2.get_id(),
        "Events do not have the same id."
    );
}

#[test]
fn set_same_event_twice_with_same_origin() {
    let id = MinId { id: 1 };
    let msg = "Set first message";
    let line = line!();

    let recv = TESTS_PUBLISHER.subscribe(&id).unwrap();

    evident::event::EventFns::<MinId, MinEventEntry, MinInterimEvent>::set_event(
        id,
        env!("CARGO_PKG_NAME"),
        msg,
        file!(),
        line,
        module_path!(),
    )
    .finalize();
    evident::event::EventFns::<MinId, MinEventEntry, MinInterimEvent>::set_event(
        id,
        env!("CARGO_PKG_NAME"),
        msg,
        file!(),
        line,
        module_path!(),
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

    assert_eq!(*event_1.get_id(), id, "Ids are not equal.");
    assert_eq!(
        event_1.get_origin().line_nr,
        line,
        "Line numbers are not equal."
    );

    assert_eq!(*event_2.get_id(), id, "Ids are not equal.");
    assert_eq!(
        event_2.get_origin().line_nr,
        line,
        "Line numbers are not equal."
    );

    assert_ne!(event_1, event_2, "Received events are equal.");
}
