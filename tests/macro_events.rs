// use logid::{
//     drain_map,
//     id_entry::Origin,
//     log_id::{get_log_id, EventLevel},
//     set_event,
// };

// mod helper;
// use crate::helper::delayed_map_drain;

// #[test]
// fn set_event_with_macro() {
//     drain_map!();

//     let log_id = get_log_id(0, 0, EventLevel::Error, 2);
//     let msg = "Set first log message";

//     let log_id = set_event!(log_id, msg).finalize();

//     let map = delayed_map_drain();

//     assert_eq!(map.len(), 1, "More than one or no event captured!");
//     assert!(map.contains_key(&log_id), "Log-id not inside captured map!");

//     let entries = map.get(&log_id).unwrap();
//     assert_eq!(
//         entries.len(),
//         1,
//         "More than one or no entry for the same log-id"
//     );

//     let entry = entries.iter().last().unwrap();
//     assert_eq!(
//         *entry.get_id(),
//         log_id,
//         "Set and stored log-ids are not equal"
//     );
//     assert_eq!(
//         *entry.get_origin(),
//         Origin {
//             filename: file!().to_string(),
//             line_nr: line!() - 24,
//         },
//         "Origin of log_id not set correctly"
//     );
//     assert_eq!(
//         *entry.get_level(),
//         EventLevel::Error,
//         "Set and stored event levels are not equal"
//     );

//     assert_eq!(entry.get_infos().len(), 0, "Info was set");
// }

// #[test]
// fn set_event_with_literal_msg() {
//     drain_map!();

//     let log_id = get_log_id(0, 0, EventLevel::Error, 2);

//     let log_id = set_event!(log_id, "Set first log message").finalize();

//     let map = delayed_map_drain();
//     let entries = map.get(&log_id).unwrap();
//     let entry = entries.iter().last().unwrap();
//     assert_eq!(
//         *entry.get_origin(),
//         Origin {
//             filename: file!().to_string(),
//             line_nr: line!() - 9
//         },
//         "Origin of log_id not set correctly"
//     );
// }

// #[test]
// fn set_event_macro() {
//     drain_map!();

//     let log_id = get_log_id(0, 0, EventLevel::Error, 2);

//     let log_id = set_event!(log_id, "Set first log message").finalize();

//     let map = delayed_map_drain();
//     let entries = map.get(&log_id).unwrap();
//     let entry = entries.iter().last().unwrap();
//     assert_eq!(
//         *entry.get_origin(),
//         Origin {
//             filename: file!().to_string(),
//             line_nr: line!() - 9,
//         },
//         "Origin of log_id not set correctly"
//     );
// }

// #[test]
// fn set_event_macro_using_expression() {
//     drain_map!();

//     let log_id = get_log_id(0, 0, EventLevel::Error, 2);

//     let log_id =
//         set_event!(log_id, &format!("Set first log message with id={}", log_id)).finalize();

//     let map = delayed_map_drain();
//     let entries = map.get(&log_id).unwrap();
//     let entry = entries.iter().last().unwrap();
//     assert_eq!(*entry.get_id(), log_id, "ID of log_id not set correctly");
// }

// enum TestLogId {
//     Id = get_log_id(0, 0, EventLevel::Error, 2),
// }

// #[test]
// fn set_event_with_enum() {
//     drain_map!();

//     let msg = "Set first log message";
//     let log_id = set_event!(TestLogId::Id, msg).finalize();

//     let map = delayed_map_drain();

//     assert_eq!(map.len(), 1, "More than one or no event captured!");
//     assert!(map.contains_key(&log_id), "Log-id not inside captured map!");

//     let entries = map.get(&log_id).unwrap();
//     assert_eq!(
//         entries.len(),
//         1,
//         "More than one or no entry for the same log-id"
//     );

//     let entry = entries.iter().last().unwrap();
//     assert_eq!(
//         *entry.get_id(),
//         log_id,
//         "Set and stored log-ids are not equal"
//     );
//     assert_eq!(
//         *entry.get_level(),
//         EventLevel::Error,
//         "Set and stored event levels are not equal"
//     );

//     assert_eq!(entry.get_infos().len(), 0, "Info was set");
// }
