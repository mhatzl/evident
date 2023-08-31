//! This module contains the minimal required implementation for the [`Id`](evident::event::Id) trait.
//!
//! [req:qa.ux.usage](https://github.com/mhatzl/evident/wiki/5-REQ-qa.ux.usage#qauxusage-provide-usage-examples)

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Copy)]
pub struct MinId {
    pub id: isize,
}

impl evident::event::Id for MinId {}

const START_CAPTURING: MinId = MinId { id: -1 };
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
