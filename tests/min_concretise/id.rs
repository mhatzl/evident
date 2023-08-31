//! This module contains the minimal required implementation for the [`Id`](evident::event::Id) trait.
//!
//! [req:qa.ux.usage](https://github.com/mhatzl/evident/wiki/5-REQ-qa.ux.usage#qauxusage-provide-usage-examples)

/// Struct used for a minimal [`Id`](evident::event::Id) trait implementation.
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Copy)]
pub struct MinId {
    pub id: isize,
}

impl evident::event::Id for MinId {}

/// Event-ID to notify the publisher and all listeners that capturing should be started.
///
/// [req:event.id.ctrl](https://github.com/mhatzl/evident/wiki/5-REQ-event.id.ctrl#eventidctrl-event-ids-for-capture-control), [req:cap.ctrl.start](https://github.com/mhatzl/evident/wiki/5-REQ-cap.ctrl.start#capctrlstart-start-capturing)
const START_CAPTURING: MinId = MinId { id: -1 };

/// Event-ID to notify the publisher and all listeners that capturing should be stopped.
///
/// [req:event.id.ctrl](https://github.com/mhatzl/evident/wiki/5-REQ-event.id.ctrl#eventidctrl-event-ids-for-capture-control), [req:cap.ctrl.stop](https://github.com/mhatzl/evident/wiki/5-REQ-cap.ctrl.stop#capctrlstop-stop-capturing)
const STOP_CAPTURING: MinId = MinId { id: -2 };

impl evident::publisher::CaptureControl for MinId {
    fn start(id: &Self) -> bool {
        id == &START_CAPTURING
    }

    fn start_id() -> Self {
        START_CAPTURING
    }

    fn stop(id: &Self) -> bool {
        id == &STOP_CAPTURING
    }

    fn stop_id() -> Self {
        STOP_CAPTURING
    }
}
