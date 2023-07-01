#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MinMsg {
    pub nr: usize,
}

impl evident::event::Msg for MinMsg {}
