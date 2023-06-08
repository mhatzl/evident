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

const STOP_CAPTURING: MinId = MinId { id: 0 };

impl evident::publisher::StopCapturing for MinId {
    fn stop_capturing(id: &Self) -> bool {
        if id == &STOP_CAPTURING {
            return true;
        }

        false
    }
}
