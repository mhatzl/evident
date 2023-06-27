#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Copy)]
pub struct MinId {
    pub id: isize,
}

impl evident::publisher::Id for MinId {}

impl std::fmt::Display for MinId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.id.to_string())
    }
}

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
