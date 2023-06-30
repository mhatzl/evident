use std::marker::PhantomData;

use crate::publisher::{CaptureControl, Id};

use super::entry::EventEntry;

pub trait Filter<K>
where
    K: Id + CaptureControl,
{
    /// Return `true` if the entry is allowed to be captured.
    fn allow_entry(&self, entry: &impl EventEntry<K>) -> bool;
}

#[derive(Default, Debug)]
pub struct DummyFilter<K>
where
    K: Id + CaptureControl,
{
    v: PhantomData<K>,
}

impl<K> Filter<K> for DummyFilter<K>
where
    K: Id + CaptureControl,
{
    fn allow_entry(&self, _entry: &impl EventEntry<K>) -> bool {
        true
    }
}
