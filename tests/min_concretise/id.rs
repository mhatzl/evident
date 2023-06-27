#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Copy)]
pub struct MinId {
    pub id: isize,
}

impl evident::publisher::Id for MinId {}

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
