// use std::{sync::mpsc::Receiver, thread};

// use logid::{
//     event::Event,
//     log_id::{get_log_id, LogLevel},
//     set_event, subscribe,
// };

// #[test]
// fn set_different_events_in_two_threads() {
//     let log_id_side = get_log_id(0, 0, LogLevel::Error, 1);
//     let msg_side = "Set side thread log message";
//     let log_id_main = get_log_id(0, 0, LogLevel::Error, 2);
//     let msg_main = "Set main thread message";

//     let recv_side = subscribe!(log_id_side).unwrap();
//     let recv_main = subscribe!(log_id_main).unwrap();

//     let side_thread = thread::spawn(move || {
//         set_event!(log_id_side, msg_side).finalize();
//     });

//     set_event!(log_id_main, msg_main).finalize();

//     assert!(side_thread.join().is_ok(), "Side thread panicked.");

//     let event_side = recv_side
//         .recv_timeout(std::time::Duration::from_millis(10))
//         .unwrap();
//     assert_eq!(
//         event_side.get_entry().get_id(),
//         &log_id_side,
//         "Received side event has wrong LogId."
//     );
//     assert_eq!(
//         event_side.get_entry().get_msg(),
//         msg_side,
//         "Received side event has wrong msg."
//     );

//     let event_main = recv_main
//         .recv_timeout(std::time::Duration::from_millis(10))
//         .unwrap();
//     assert_eq!(
//         event_main.get_entry().get_id(),
//         &log_id_main,
//         "Received main event has wrong LogId."
//     );
//     assert_eq!(
//         event_main.get_entry().get_msg(),
//         msg_main,
//         "Received main event has wrong msg."
//     );
// }

// #[test]
// fn set_same_logid_in_two_threads() {
//     let log_id = get_log_id(0, 0, LogLevel::Error, 1);
//     let msg_side = "Set side thread log message";
//     let msg_main = "Set main thread message";

//     let recv = subscribe!(log_id).unwrap();

//     let side_thread = thread::spawn(move || {
//         set_event!(log_id, msg_side).finalize();
//     });

//     set_event!(log_id, msg_main).finalize();

//     assert!(side_thread.join().is_ok(), "Side thread panicked.");

//     let event_1 = recv
//         .recv_timeout(std::time::Duration::from_millis(10))
//         .unwrap();
//     assert_eq!(
//         event_1.get_entry().get_id(),
//         &log_id,
//         "Received event 1 has wrong LogId."
//     );
//     assert!(
//         event_1.get_entry().get_msg() == msg_main || event_1.get_entry().get_msg() == msg_side,
//         "Received event 1 has wrong msg."
//     );

//     let event_2 = recv
//         .recv_timeout(std::time::Duration::from_millis(10))
//         .unwrap();
//     assert_eq!(
//         event_2.get_entry().get_id(),
//         &log_id,
//         "Received event 2 has wrong LogId."
//     );
//     assert!(
//         event_2.get_entry().get_msg() == msg_main || event_2.get_entry().get_msg() == msg_side,
//         "Received event 2 has wrong msg."
//     );

//     assert_ne!(
//         event_1.get_entry().get_msg(),
//         event_2.get_entry().get_msg(),
//         "Both events have the same msg."
//     );
// }

// #[test]
// fn set_events_in_many_threads() {
//     const THREAD_CNT: u8 = 63; // Note: 63 is the maximum for the local nr of a LogId
//     let base_log_id = get_log_id(0, 0, LogLevel::Error, 1);
//     let msg = "Set log message";

//     let mut recvs: Vec<Receiver<Event>> = Vec::new();
//     for i in 1..=THREAD_CNT {
//         let loop_id = get_log_id(0, 0, LogLevel::Error, i);
//         recvs.push(subscribe!(loop_id).unwrap());
//     }

//     set_event!(base_log_id, msg).finalize();

//     rayon::scope(|s| {
//         // start at 2 to jump over base_log_id
//         for i in 2..=THREAD_CNT {
//             s.spawn(move |_| {
//                 let loop_id = get_log_id(0, 0, LogLevel::Error, i);

//                 // Note: `finalize()` would not be needed, since events are finalized on drop, but it makes this test easier to read
//                 set_event!(base_log_id, msg).finalize();
//                 set_event!(loop_id, msg).finalize();
//             });
//         }
//     });

//     for i in 1..=THREAD_CNT {
//         let log_id = get_log_id(0, 0, LogLevel::Error, i);

//         let event = recvs[(i - 1) as usize]
//             .recv_timeout(std::time::Duration::from_millis(10))
//             .unwrap();
//         assert_eq!(
//             event.get_entry().get_id(),
//             &log_id,
//             "Received event {} has wrong LogId.",
//             i
//         );
//     }

//     // Note: Starting at "2", because one rcv was already consumed in loop above
//     for i in 2..=THREAD_CNT {
//         let event = recvs[0]
//             .recv_timeout(std::time::Duration::from_millis(10))
//             .unwrap();
//         assert_eq!(
//             event.get_entry().get_id(),
//             &base_log_id,
//             "Received event {} has wrong LogId.",
//             i
//         );
//     }
// }
