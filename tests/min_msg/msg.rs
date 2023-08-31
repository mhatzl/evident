//! This module contains the minimal required implementation for the [`Msg`](evident::event::Msg) trait.
//!
//! [req:qa.ux.usage](https://github.com/mhatzl/evident/wiki/5-REQ-qa.ux.usage#qauxusage-provide-usage-examples), [req:event.msg](https://github.com/mhatzl/evident/wiki/5-REQ-event.msg#eventmsg-event-message)

/// Struct used for a minimal [`Msg`](evident::event::Msg) trait implementation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MinMsg {
    pub nr: usize,
}

impl evident::event::Msg for MinMsg {}
