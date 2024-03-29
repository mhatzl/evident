//! This module contains the minimal required implementation for the [`Filter`] trait.
//!
//! [req:qa.ux.usage], [req:cap.filter]

use evident::event::filter::Filter;

use super::id::MinId;

/// Struct used for a minimal [`Filter`] trait implementation.
#[derive(Default)]
pub struct MinFilter {}

impl Filter<MinId, String> for MinFilter {
    fn allow_entry(&self, entry: &impl evident::event::entry::EventEntry<MinId, String>) -> bool {
        if entry.get_event_id().id % 2 == 0 {
            return true;
        }
        false
    }
}
