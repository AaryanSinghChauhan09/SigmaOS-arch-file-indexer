// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-arch-file-indexer: arch file indexer component model.
//!
//! Real implementation: indexers, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;
pub mod sigmaindexmanager;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Indexer {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchFileIndexerError {
    EmptyIndexerName,
    DuplicateIndexer(String),
    EmptyEventSubject,
}

impl fmt::Display for ArchFileIndexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIndexerName => write!(f, "indexer name must not be empty"),
            Self::DuplicateIndexer(id) => write!(f, "duplicate indexer {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for ArchFileIndexerError {}

pub struct ArchFileIndexer {
    pub name: String,
    indexers: BTreeMap<String, Indexer>,
    events: BTreeMap<String, Event>,
}

impl ArchFileIndexer {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            indexers: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_indexer(&mut self, item: Indexer) -> Result<(), ArchFileIndexerError> {
        if item.name.trim().is_empty() {
            return Err(ArchFileIndexerError::EmptyIndexerName);
        }
        let id = item.id.clone();
        if self.indexers.contains_key(&id) {
            return Err(ArchFileIndexerError::DuplicateIndexer(id.clone()));
        }
        self.indexers.insert(id, item);
        Ok(())
    }

    pub fn indexer_count(&self) -> usize {
        self.indexers.len()
    }

    pub fn get_indexer(&self, id: &str) -> Option<&Indexer> {
        self.indexers.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), ArchFileIndexerError> {
        if event.subject.trim().is_empty() {
            return Err(ArchFileIndexerError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_indexer(id: &str, name: &str) -> Indexer {
        Indexer {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = ArchFileIndexer::new("ArchFileIndexer");
        assert_eq!(m.name, "ArchFileIndexer");
        assert_eq!(m.indexer_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_indexer() {
        let mut m = ArchFileIndexer::new("ArchFileIndexer");
        m.add_indexer(sample_indexer("a1", "Alpha")).unwrap();
        assert_eq!(m.indexer_count(), 1);
        assert!(m.get_indexer("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = ArchFileIndexer::new("ArchFileIndexer");
        assert!(m.add_indexer(sample_indexer("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = ArchFileIndexer::new("ArchFileIndexer");
        m.add_indexer(sample_indexer("x", "One")).unwrap();
        assert!(m.add_indexer(sample_indexer("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = ArchFileIndexer::new("ArchFileIndexer");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
