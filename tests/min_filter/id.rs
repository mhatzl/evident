#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Copy)]
pub struct MinId {
    pub id: isize,
}

impl evident::publisher::Id for MinId {}

// Note: `id: 1` is important, since filter would not allow an event with this id.
// Test in `mod` ensures that stop capturing event is still captured.
pub(super) const STOP_CAPTURING: MinId = MinId { id: 1 };

impl evident::publisher::StopCapturing for MinId {
    fn stop_capturing(id: &Self) -> bool {
        if id == &STOP_CAPTURING {
            return true;
        }

        false
    }
}
