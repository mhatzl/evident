// //! Tests capturing functionalities

// use logid::{
//     event::{origin::Origin, EventFns},
//     log_id::{get_log_id, LogLevel, ANONYMOUS_LOG_ID},
//     publisher, set_event, subscribe,
// };

// #[test]
// fn capture_single_logid() {
//     let log_id = get_log_id(0, 0, LogLevel::Error, 2);
//     let msg = "Set first log message";

//     let recv = publisher::subscribe(log_id, env!("CARGO_PKG_NAME")).unwrap();

//     set_event!(log_id, msg).finalize();

//     let event = recv
//         .recv_timeout(std::time::Duration::from_millis(10))
//         .unwrap();

//     let entry = event.get_entry();
//     assert_eq!(
//         *entry.get_id(),
//         log_id,
//         "Set and stored log-ids are not equal"
//     );
//     assert_eq!(
//         *entry.get_level(),
//         LogLevel::Error,
//         "Set and stored event levels are not equal"
//     );
//     assert_eq!(
//         *entry.get_msg(),
//         msg,
//         "Set and stored messages are not equal"
//     );
//     assert_eq!(
//         *entry.get_origin(),
//         Origin::new(file!(), line!() - 24, module_path!()), //Note: Event is set 24 lines above
//         "Set and stored origins are not equal"
//     );
// }

// #[cfg(feature = "causes")]
// #[test]
// fn capture_single_logid_with_cause() {
//     let cause_log_id = get_log_id(0, 0, LogLevel::Warn, 1);
//     let cause_msg = "Cause log message";
//     let log_id = get_log_id(0, 0, LogLevel::Error, 2);
//     let msg = "My log message";

//     let recv = publisher::subscribe_to_logs(
//         vec![cause_log_id, log_id].iter().copied(),
//         env!("CARGO_PKG_NAME"),
//     )
//     .unwrap();

//     set_event!(cause_log_id, cause_msg).finalize();

//     let cause_event = recv
//         .recv_timeout(std::time::Duration::from_millis(10))
//         .unwrap();

//     set_event!(log_id, msg).add_cause(cause_event).finalize();

//     let event = recv
//         .recv_timeout(std::time::Duration::from_millis(10))
//         .unwrap();

//     assert_eq!(
//         event.get_entry().get_id(),
//         &log_id,
//         "Set and received log-ids are not equal"
//     );
//     assert_eq!(
//         event
//             .get_entry()
//             .get_causes()
//             .get(0)
//             .unwrap()
//             .get_entry()
//             .get_id(),
//         &cause_log_id,
//         "Set and received causing log-ids are not equal"
//     );
//     assert_eq!(
//         event
//             .get_entry()
//             .get_causes()
//             .get(0)
//             .unwrap()
//             .get_entry()
//             .get_msg(),
//         &cause_msg,
//         "Set and received causing msgs are not equal"
//     );
// }

// #[test]
// fn capture_single_logid_with_info() {
//     let log_id = get_log_id(0, 1, LogLevel::Info, 1);
//     let msg = "Set first log message";
//     let info = "Additional info for this log-id";

//     let recv = publisher::subscribe(log_id, env!("CARGO_PKG_NAME")).unwrap();

//     set_event!(log_id, msg).add_info(info).finalize();

//     let event = recv
//         .recv_timeout(std::time::Duration::from_millis(10))
//         .unwrap();

//     let entry = event.get_entry();
//     assert_eq!(
//         *entry.get_id(),
//         log_id,
//         "Set and stored log-ids are not equal"
//     );
//     assert_eq!(
//         *entry.get_level(),
//         LogLevel::Info,
//         "Set and stored event levels are not equal"
//     );

//     assert_eq!(
//         entry.get_infos().len(),
//         1,
//         "More than one or no info was set"
//     );
//     let act_info = entry.get_infos().last().unwrap();
//     assert_eq!(act_info, info, "Set and stored messages are not equal");
// }

// #[test]
// fn capture_single_logid_with_debug() {
//     let log_id = get_log_id(1, 1, LogLevel::Debug, 0);
//     let msg = "Set first log message";
//     let debug = "Additional debug info for this log-id";

//     let recv = publisher::subscribe(log_id, env!("CARGO_PKG_NAME")).unwrap();

//     set_event!(log_id, msg).add_debug(debug).finalize();

//     let event = recv
//         .recv_timeout(std::time::Duration::from_millis(10))
//         .unwrap();

//     let entry = event.get_entry();
//     assert_eq!(
//         *entry.get_id(),
//         log_id,
//         "Set and stored log-ids are not equal"
//     );
//     assert_eq!(
//         *entry.get_level(),
//         LogLevel::Debug,
//         "Set and stored event levels are not equal"
//     );

//     assert_eq!(
//         entry.get_debugs().len(),
//         1,
//         "More than one or no debug info was set"
//     );
//     let act_debug = entry.get_debugs().last().unwrap();
//     assert_eq!(act_debug, debug, "Set and stored messages are not equal");
// }

// #[test]
// fn capture_single_logid_with_trace() {
//     let log_id = get_log_id(1, 1, LogLevel::Debug, 0);
//     let msg = "Set first log message";
//     let trace = "Additional debug info for this log-id";

//     let recv = publisher::subscribe(log_id, env!("CARGO_PKG_NAME")).unwrap();

//     set_event!(log_id, msg).add_trace(trace).finalize();

//     let event = recv
//         .recv_timeout(std::time::Duration::from_millis(10))
//         .unwrap();

//     let entry = event.get_entry();
//     assert_eq!(
//         *entry.get_id(),
//         log_id,
//         "Set and stored log-ids are not equal"
//     );
//     assert_eq!(
//         *entry.get_level(),
//         LogLevel::Debug,
//         "Set and stored event levels are not equal"
//     );

//     assert_eq!(
//         entry.get_traces().len(),
//         1,
//         "More than one or no trace was set"
//     );
//     let act_trace = entry.get_traces().last().unwrap();
//     assert_eq!(act_trace, trace, "Set and stored messages are not equal");
// }

// #[cfg(feature = "spans")]
// #[test]
// fn logid_with_span() {
//     // Note: Tracing subscribers are responsible for span handling => simple subscriber needed for span testing
//     tracing_subscriber::fmt::init();
//     const SPAN_NAME: &str = "mySpan";

//     let log_id = get_log_id(0, 0, LogLevel::Info, 2);
//     let msg = "Set first log message";

//     let recv = publisher::subscribe(log_id, env!("CARGO_PKG_NAME")).unwrap();

//     let span = tracing::span!(tracing::Level::ERROR, SPAN_NAME);
//     span.in_scope(|| {
//         set_event!(log_id, msg);
//     });

//     let event = recv
//         .recv_timeout(std::time::Duration::from_millis(10))
//         .unwrap();

//     let entry = event.get_entry();
//     assert_eq!(
//         entry.get_span().as_ref().unwrap().metadata().unwrap(),
//         span.metadata().unwrap(),
//         "Span names are not equal"
//     );
// }

// #[test]
// fn capture_same_logid_twice_with_different_origin() {
//     let log_id = get_log_id(0, 0, LogLevel::Error, 2);
//     let msg = "Set first log message";

//     let recv = publisher::subscribe(log_id, env!("CARGO_PKG_NAME")).unwrap();

//     let line_1 = line!() + 1;
//     set_event!(log_id, msg).finalize();

//     let line_2 = line!() + 1;
//     set_event!(log_id, msg).finalize();

//     let event_1 = recv
//         .recv_timeout(std::time::Duration::from_millis(10))
//         .unwrap();
//     let event_2 = recv
//         .recv_timeout(std::time::Duration::from_millis(10))
//         .unwrap();

//     let entry_1 = event_1.get_entry();
//     assert_eq!(
//         *entry_1.get_id(),
//         log_id,
//         "Set and stored log-ids are not equal"
//     );
//     assert_eq!(
//         entry_1.get_origin().line_nr,
//         line_1,
//         "Set and stored line numbers are not equal"
//     );

//     let entry_2 = event_2.get_entry();
//     assert_eq!(
//         *entry_2.get_id(),
//         log_id,
//         "Set and stored log-ids are not equal"
//     );
//     assert_eq!(
//         entry_2.get_origin().line_nr,
//         line_2,
//         "Set and stored line numbers are not equal"
//     );
// }

// #[test]
// fn capture_same_logid_twice_with_same_origin() {
//     let log_id = get_log_id(0, 0, LogLevel::Error, 2);
//     let msg = "Set first log message";
//     let file = file!();
//     let line = line!();

//     let recv = publisher::subscribe(log_id, env!("CARGO_PKG_NAME")).unwrap();

//     log_id
//         .set_event(env!("CARGO_PKG_NAME"), msg, file, line, module_path!())
//         .finalize();
//     log_id
//         .set_event(env!("CARGO_PKG_NAME"), msg, file, line, module_path!())
//         .finalize();

//     let event_1 = recv
//         .recv_timeout(std::time::Duration::from_millis(10))
//         .unwrap();
//     let event_2 = recv
//         .recv_timeout(std::time::Duration::from_millis(10))
//         .unwrap();

//     let entry_1 = event_1.get_entry();
//     assert_eq!(
//         *entry_1.get_id(),
//         log_id,
//         "Set and stored log-ids are not equal"
//     );
//     assert_eq!(
//         entry_1.get_origin().line_nr,
//         line,
//         "Set and stored line numbers are not equal"
//     );

//     let entry_2 = event_2.get_entry();
//     assert_eq!(
//         *entry_2.get_id(),
//         log_id,
//         "Set and stored log-ids are not equal"
//     );
//     assert_eq!(
//         entry_2.get_origin().line_nr,
//         line,
//         "Set and stored line numbers are not equal"
//     );

//     assert_ne!(entry_1, entry_2, "Received events have the same entry");
// }

// #[cfg(feature = "diagnostics")]
// #[test]
// fn capture_single_logid_with_diagnostics() {
//     use lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range};

//     let log_id = get_log_id(1, 1, LogLevel::Debug, 0);
//     let msg = "Set first log message";
//     let diagnostics = Diagnostic {
//         range: Range {
//             start: Position {
//                 line: 0,
//                 character: 4,
//             },
//             end: Position {
//                 line: 0,
//                 character: 10,
//             },
//         },
//         severity: Some(DiagnosticSeverity::INFORMATION),
//         message: "Some diagnostic information useful for lsp implementations.".to_owned(),
//         ..Default::default()
//     };

//     let recv = publisher::subscribe(log_id, env!("CARGO_PKG_NAME")).unwrap();

//     set_event!(log_id, msg)
//         .add_diagnostic(diagnostics.clone())
//         .finalize();

//     let event = recv
//         .recv_timeout(std::time::Duration::from_millis(10))
//         .unwrap();

//     let entry = event.get_entry();
//     let act_diagnostics = entry.get_diagnostics().last().unwrap();
//     assert_eq!(
//         act_diagnostics, &diagnostics,
//         "Set and stored diagnostics are not equal"
//     );
// }

// #[test]
// fn capture_anonymous_logid() {
//     let msg = "Set anonymous log message";

//     let recv = subscribe!(ANONYMOUS_LOG_ID).unwrap();

//     set_event!(ANONYMOUS_LOG_ID, msg).finalize();

//     let event = recv
//         .recv_timeout(std::time::Duration::from_millis(10))
//         .unwrap();

//     let entry = event.get_entry();
//     assert_eq!(
//         *entry.get_id(),
//         ANONYMOUS_LOG_ID,
//         "Set and stored log-ids are not equal"
//     );
//     assert_eq!(
//         *entry.get_level(),
//         LogLevel::Debug,
//         "Set and stored event levels are not equal"
//     );
//     assert_eq!(
//         *entry.get_msg(),
//         msg,
//         "Set and stored messages are not equal"
//     );
// }

// #[test]
// fn capture_single_logid_with_paylod() {
//     let log_id = get_log_id(1, 1, LogLevel::Debug, 0);
//     let msg = "Set first log message";
//     let payload = serde_json::json!({
//         "code": 200,
//         "success": true,
//         "payload": {
//             "features": [
//                 "serde",
//                 "json"
//             ]
//         }
//     });

//     let recv = publisher::subscribe(log_id, env!("CARGO_PKG_NAME")).unwrap();

//     set_event!(log_id, msg)
//         .add_payload(payload.clone())
//         .finalize();

//     let event = recv
//         .recv_timeout(std::time::Duration::from_millis(10))
//         .unwrap();

//     let entry = event.get_entry();
//     assert_eq!(
//         *entry.get_id(),
//         log_id,
//         "Set and stored log-ids are not equal"
//     );
//     assert_eq!(
//         *entry.get_level(),
//         LogLevel::Debug,
//         "Set and stored event levels are not equal"
//     );

//     assert_eq!(
//         entry.get_payloads().len(),
//         1,
//         "More than one or no trace was set"
//     );
//     let act_payload = entry.get_payloads().last().unwrap();
//     assert_eq!(
//         act_payload, &payload,
//         "Set and stored payloads are not equal"
//     );
// }
