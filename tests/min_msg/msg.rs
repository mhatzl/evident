//! This module contains the minimal required implementation for the [`Msg`](evident::event::Msg) trait.
//!
//! [req:qa.ux.usage], [req:event.msg]

/// Struct used for a minimal [`Msg`](evident::event::Msg) trait implementation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MinMsg {
    pub nr: usize,
}

impl evident::event::Msg for MinMsg {}
