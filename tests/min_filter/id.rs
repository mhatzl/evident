#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Copy)]
pub struct MinId {
    pub id: isize,
}

impl evident::event::Id for MinId {}

// Note: `id: 1` is important, since filter would not allow an event with this id.
// Test in `mod` ensures that stop capturing event is still captured.
pub(super) const STOP_CAPTURING: MinId = MinId { id: 1 };

const START_CAPTURING: MinId = MinId { id: -1 };

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
