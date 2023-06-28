use evident::event::filter::Filter;

use super::{entry::MinEventEntry, id::MinId};

#[derive(Default)]
pub struct MinFilter {}

impl Filter<MinId, MinEventEntry> for MinFilter {
    fn allow_event(&self, event: &evident::event::Event<MinId, MinEventEntry>) -> bool {
        if event.get_event_id().id % 2 == 0 {
            return true;
        }
        false
    }
}
