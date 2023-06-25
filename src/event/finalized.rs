use crate::publisher::Id;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct FinalizedEvent<K: Id> {
    pub event_id: K,
    pub entry_id: crate::uuid::Uuid,
}

impl<K: Id> FinalizedEvent<K> {
    pub fn new(event_id: K, entry_id: crate::uuid::Uuid) -> Self {
        FinalizedEvent { event_id, entry_id }
    }

    pub fn into_event_id(self) -> K {
        self.event_id
    }

    pub fn get_entry_id(&self) -> &crate::uuid::Uuid {
        &self.entry_id
    }
}

impl<K: Id + std::fmt::Display> std::fmt::Display for FinalizedEvent<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id='{}', entry='{}'", self.event_id, self.entry_id)
    }
}
