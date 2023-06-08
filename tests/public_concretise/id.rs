#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Copy)]
pub struct MinId {
    pub id: isize,
}

impl evident::publisher::Id for MinId {}

const STOP_CAPTURING: MinId = MinId { id: 0 };

impl evident::publisher::StopCapturing for MinId {
    fn stop_capturing(id: &Self) -> bool {
        if id == &STOP_CAPTURING {
            return true;
        }

        false
    }
}
