//! Contains the [`Filter`] trait.

use std::marker::PhantomData;

use crate::publisher::CaptureControl;

use super::{entry::EventEntry, Id, Msg};

/// The [`Filter`] trait allows to add a custom filter to an [`EvidentPublisher`].
pub trait Filter<K, M>
where
    K: Id + CaptureControl,
    M: Msg,
{
    /// Return `true` if the entry is allowed to be captured.
    fn allow_entry(&self, entry: &impl EventEntry<K, M>) -> bool;
}

/// Struct used to provide a dummy filter in case no custom filter is set.
#[derive(Default, Debug)]
pub struct DummyFilter<K, M>
where
    K: Id + CaptureControl,
    M: Msg,
{
    v1: PhantomData<K>,
    v2: PhantomData<M>,
}

impl<K, M> Filter<K, M> for DummyFilter<K, M>
where
    K: Id + CaptureControl,
    M: Msg,
{
    fn allow_entry(&self, _entry: &impl EventEntry<K, M>) -> bool {
        true
    }
}
