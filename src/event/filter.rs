use std::marker::PhantomData;

use crate::publisher::{Id, StopCapturing};

use super::{entry::EventEntry, intermediary::IntermediaryEvent};

pub trait Filter<K, T>
where
    K: Id + StopCapturing,
    T: EventEntry<K>,
{
    /// Return `true` if the event should be captured.
    fn allow_event(&self, event: &mut impl IntermediaryEvent<K, T>) -> bool;
}

#[derive(Default, Debug)]
pub struct DummyFilter<K, T>
where
    K: Id + StopCapturing,
    T: EventEntry<K>,
{
    v1: PhantomData<K>,
    v2: PhantomData<T>,
}

impl<K, T> Filter<K, T> for DummyFilter<K, T>
where
    K: Id + StopCapturing,
    T: EventEntry<K>,
{
    fn allow_event(&self, _event: &mut impl IntermediaryEvent<K, T>) -> bool {
        true
    }
}
