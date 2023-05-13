// use logid::{
//     log_id::{get_log_id, LogLevel},
//     logid, publisher, set_event, subscribe, subscribe_to_logs,
// };

// #[test]
// fn subscribe_to_one_logid() {
//     let log_id = get_log_id(0, 0, LogLevel::Error, 2);
//     let msg = "Set first log message";

//     let recv = publisher::subscribe(log_id, env!("CARGO_PKG_NAME")).unwrap();

//     set_event!(log_id, msg).finalize();

//     let event = recv
//         .recv_timeout(std::time::Duration::from_millis(10))
//         .unwrap();
//     assert_eq!(
//         event.get_crate_name(),
//         env!("CARGO_PKG_NAME"),
//         "Event received from wrong crate."
//     );
//     assert_eq!(
//         event.get_entry().get_id(),
//         &log_id,
//         "Received event has wrong LogId."
//     );
//     assert_eq!(
//         event.get_entry().get_msg(),
//         msg,
//         "Received event has wrong msg."
//     );
// }

// #[test]
// fn subscribe_macro() {
//     let log_id = get_log_id(0, 0, LogLevel::Error, 2);
//     let msg = "Set first log message";

//     let recv = subscribe!(log_id).unwrap();

//     set_event!(log_id, msg).finalize();

//     let event = recv
//         .recv_timeout(std::time::Duration::from_millis(10))
//         .unwrap();
//     assert_eq!(
//         event.get_crate_name(),
//         env!("CARGO_PKG_NAME"),
//         "Event received from wrong crate."
//     );
//     assert_eq!(
//         event.get_entry().get_id(),
//         &log_id,
//         "Received event has wrong LogId."
//     );
//     assert_eq!(
//         event.get_entry().get_msg(),
//         msg,
//         "Received event has wrong msg."
//     );
// }

// enum TestLogId {
//     Id = get_log_id(0, 0, LogLevel::Error, 2),
// }

// #[test]
// fn subscribe_macro_with_logid_enum() {
//     let msg = "Set first log message";

//     let recv = subscribe!(TestLogId::Id).unwrap();

//     set_event!(TestLogId::Id, msg).finalize();

//     let event = recv
//         .recv_timeout(std::time::Duration::from_millis(10))
//         .unwrap();
//     assert_eq!(
//         event.get_crate_name(),
//         env!("CARGO_PKG_NAME"),
//         "Event received from wrong crate."
//     );
//     assert_eq!(
//         event.get_entry().get_id(),
//         &logid!(TestLogId::Id),
//         "Received event has wrong LogId."
//     );
//     assert_eq!(
//         event.get_entry().get_msg(),
//         msg,
//         "Received event has wrong msg."
//     );
// }

// #[test]
// fn two_log_ids_separate_receiver() {
//     let log_id_1 = get_log_id(0, 0, LogLevel::Error, 2);
//     let msg_1 = "Set first log message";
//     let log_id_2 = get_log_id(0, 0, LogLevel::Error, 3);
//     let msg_2 = "Set second log message";

//     let recv_1 = subscribe!(log_id_1).unwrap();
//     let recv_2 = subscribe!(log_id_2).unwrap();

//     set_event!(log_id_1, msg_1).finalize();
//     set_event!(log_id_2, msg_2).finalize();

//     let event_1 = recv_1
//         .recv_timeout(std::time::Duration::from_millis(10))
//         .unwrap();
//     assert_eq!(
//         event_1.get_entry().get_id(),
//         &log_id_1,
//         "Received event 1 has wrong LogId."
//     );
//     assert_eq!(
//         event_1.get_entry().get_msg(),
//         msg_1,
//         "Received event 1 has wrong msg."
//     );

//     let event_2 = recv_2
//         .recv_timeout(std::time::Duration::from_millis(10))
//         .unwrap();
//     assert_eq!(
//         event_2.get_entry().get_id(),
//         &log_id_2,
//         "Received event 2 has wrong LogId."
//     );
//     assert_eq!(
//         event_2.get_entry().get_msg(),
//         msg_2,
//         "Received event 2 has wrong msg."
//     );
// }

// #[test]
// fn one_log_id_separate_receiver() {
//     let log_id = get_log_id(0, 0, LogLevel::Error, 2);
//     let msg = "Set first log message";

//     let recv_1 = subscribe!(log_id).unwrap();
//     let recv_2 = subscribe!(log_id).unwrap();

//     set_event!(log_id, msg).finalize();

//     let event_1 = recv_1
//         .recv_timeout(std::time::Duration::from_millis(10))
//         .unwrap();
//     assert_eq!(
//         event_1.get_entry().get_id(),
//         &log_id,
//         "Received event 1 has wrong LogId."
//     );
//     assert_eq!(
//         event_1.get_entry().get_msg(),
//         msg,
//         "Received event 1 has wrong msg."
//     );

//     let event_2 = recv_2
//         .recv_timeout(std::time::Duration::from_millis(10))
//         .unwrap();
//     assert_eq!(
//         event_2.get_entry().get_id(),
//         &log_id,
//         "Received event 2 has wrong LogId."
//     );
//     assert_eq!(
//         event_2.get_entry().get_msg(),
//         msg,
//         "Received event 2 has wrong msg."
//     );
// }

// #[test]
// fn subscribe_to_two_log_ids_at_once() {
//     let log_id_1 = get_log_id(0, 0, LogLevel::Error, 2);
//     let msg_1 = "Set first log message";
//     let log_id_2 = get_log_id(0, 0, LogLevel::Error, 3);
//     let msg_2 = "Set second log message";

//     let recv = publisher::subscribe_to_logs(
//         vec![log_id_1, log_id_2].iter().copied(),
//         env!("CARGO_PKG_NAME"),
//     )
//     .unwrap();

//     set_event!(log_id_1, msg_1).finalize();
//     set_event!(log_id_2, msg_2).finalize();

//     let event_1 = recv
//         .recv_timeout(std::time::Duration::from_millis(10))
//         .unwrap();
//     assert!(
//         event_1.get_entry().get_id() == &log_id_1 || event_1.get_entry().get_id() == &log_id_2,
//         "Received event 1 has wrong LogId."
//     );
//     assert!(
//         event_1.get_entry().get_msg() == msg_1 || event_1.get_entry().get_msg() == msg_2,
//         "Received event 1 has wrong msg."
//     );

//     let event_2 = recv
//         .recv_timeout(std::time::Duration::from_millis(10))
//         .unwrap();
//     assert!(
//         event_2.get_entry().get_id() == &log_id_1 || event_2.get_entry().get_id() == &log_id_2,
//         "Received event 2 has wrong LogId."
//     );
//     assert!(
//         event_2.get_entry().get_msg() == msg_1 || event_2.get_entry().get_msg() == msg_2,
//         "Received event 2 has wrong msg."
//     );
//     assert_ne!(
//         event_1.get_entry().get_id(),
//         event_2.get_entry().get_id(),
//         "Both events received the same id."
//     );
// }

// #[test]
// fn subscribe_to_two_log_ids_at_once_via_macro() {
//     let log_id_1 = get_log_id(0, 0, LogLevel::Error, 2);
//     let msg_1 = "Set first log message";
//     let log_id_2 = get_log_id(0, 0, LogLevel::Error, 3);
//     let msg_2 = "Set second log message";

//     let recv = subscribe_to_logs!(vec![log_id_1, log_id_2].iter()).unwrap();

//     set_event!(log_id_1, msg_1).finalize();
//     set_event!(log_id_2, msg_2).finalize();

//     let event_1 = recv
//         .recv_timeout(std::time::Duration::from_millis(10))
//         .unwrap();
//     assert!(
//         event_1.get_entry().get_id() == &log_id_1 || event_1.get_entry().get_id() == &log_id_2,
//         "Received event 1 has wrong LogId."
//     );
//     assert!(
//         event_1.get_entry().get_msg() == msg_1 || event_1.get_entry().get_msg() == msg_2,
//         "Received event 1 has wrong msg."
//     );

//     let event_2 = recv
//         .recv_timeout(std::time::Duration::from_millis(10))
//         .unwrap();
//     assert!(
//         event_2.get_entry().get_id() == &log_id_1 || event_2.get_entry().get_id() == &log_id_2,
//         "Received event 2 has wrong LogId."
//     );
//     assert!(
//         event_2.get_entry().get_msg() == msg_1 || event_2.get_entry().get_msg() == msg_2,
//         "Received event 2 has wrong msg."
//     );
//     assert_ne!(
//         event_1.get_entry().get_id(),
//         event_2.get_entry().get_id(),
//         "Both events received the same id."
//     );
// }

// // #[test]
// // fn receive_timeouted_any_of_two_log_id_events() {
// //     let log_id_1 = get_log_id(0, 0, LogLevel::Error, 2);
// //     let msg_1 = "Set first log message";
// //     let log_id_2 = get_log_id(0, 0, LogLevel::Error, 3);
// //     let msg_2 = "Set second log message";

// //     let recvs = subscribe_to_logs!(vec![log_id_1, log_id_2].iter()).unwrap();

// //     set_event!(log_id_1, msg_1).finalize();
// //     set_event!(log_id_2, msg_2).finalize();

// //     let any_event = publisher::receive_any(
// //         &recvs,
// //         ReceiveKind::SelectTimeout(std::time::Duration::from_millis(10)),
// //     )
// //     .unwrap();
// //     assert!(
// //         any_event.get_entry().get_id() == &log_id_1 || any_event.get_entry().get_id() == &log_id_2,
// //         "Received event LogId matched neither log_id_1 nor log_id_2."
// //     );
// //     assert!(
// //         any_event.get_entry().get_msg() == msg_1 || any_event.get_entry().get_msg() == msg_2,
// //         "Received event msg matched neither msg_1 nor msg_2."
// //     );
// // }

// // #[test]
// // fn receive_any_of_two_log_id_events() {
// //     let log_id_1 = get_log_id(0, 0, LogLevel::Error, 2);
// //     let msg_1 = "Set first log message";
// //     let log_id_2 = get_log_id(0, 0, LogLevel::Error, 3);
// //     let msg_2 = "Set second log message";

// //     let recvs = subscribe_to_logs!(vec![log_id_1, log_id_2].iter()).unwrap();

// //     set_event!(log_id_1, msg_1).finalize();
// //     set_event!(log_id_2, msg_2).finalize();

// //     let any_event = publisher::receive_any(&recvs, ReceiveKind::Select).unwrap();
// //     assert!(
// //         any_event.get_entry().get_id() == &log_id_1 || any_event.get_entry().get_id() == &log_id_2,
// //         "Received event LogId matched neither log_id_1 nor log_id_2."
// //     );
// //     assert!(
// //         any_event.get_entry().get_msg() == msg_1 || any_event.get_entry().get_msg() == msg_2,
// //         "Received event msg matched neither msg_1 nor msg_2."
// //     );
// // }

// // #[test]
// // fn receive_ready_timeouted_any_of_two_log_id_events() {
// //     let log_id_1 = get_log_id(0, 0, LogLevel::Error, 2);
// //     let msg_1 = "Set first log message";
// //     let log_id_2 = get_log_id(0, 0, LogLevel::Error, 3);
// //     let msg_2 = "Set second log message";

// //     let recvs = subscribe_to_logs!(vec![log_id_1, log_id_2].iter()).unwrap();

// //     set_event!(log_id_1, msg_1).finalize();
// //     set_event!(log_id_2, msg_2).finalize();

// //     let any_event = publisher::receive_any(
// //         &recvs,
// //         ReceiveKind::ReadyTimeout(std::time::Duration::from_millis(10)),
// //     )
// //     .unwrap();
// //     assert!(
// //         any_event.get_entry().get_id() == &log_id_1 || any_event.get_entry().get_id() == &log_id_2,
// //         "Received event LogId matched neither log_id_1 nor log_id_2."
// //     );
// //     assert!(
// //         any_event.get_entry().get_msg() == msg_1 || any_event.get_entry().get_msg() == msg_2,
// //         "Received event msg matched neither msg_1 nor msg_2."
// //     );
// // }

// // #[test]
// // fn receive_any_ready_of_two_log_id_events() {
// //     let log_id_1 = get_log_id(0, 0, LogLevel::Error, 2);
// //     let msg_1 = "Set first log message";
// //     let log_id_2 = get_log_id(0, 0, LogLevel::Error, 3);
// //     let msg_2 = "Set second log message";

// //     let recvs = subscribe_to_logs!(vec![log_id_1, log_id_2].iter()).unwrap();

// //     set_event!(log_id_1, msg_1).finalize();
// //     set_event!(log_id_2, msg_2).finalize();

// //     let any_event = publisher::receive_any(&recvs, ReceiveKind::Ready).unwrap();
// //     assert!(
// //         any_event.get_entry().get_id() == &log_id_1 || any_event.get_entry().get_id() == &log_id_2,
// //         "Received event LogId matched neither log_id_1 nor log_id_2."
// //     );
// //     assert!(
// //         any_event.get_entry().get_msg() == msg_1 || any_event.get_entry().get_msg() == msg_2,
// //         "Received event msg matched neither msg_1 nor msg_2."
// //     );
// // }
