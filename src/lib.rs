// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-a2dp: a2dp component model.
//!
//! Real implementation: a2dp_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct A2dpItem {
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
pub enum A2dpError {
    EmptyA2dpItemName,
    DuplicateA2dpItem(String),
    EmptyEventSubject,
}

impl fmt::Display for A2dpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyA2dpItemName => write!(f, "a2dp_item name must not be empty"),
            Self::DuplicateA2dpItem(id) => write!(f, "duplicate a2dp_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for A2dpError {}

pub struct A2dp {
    pub name: String,
    a2dp_items: BTreeMap<String, A2dpItem>,
    events: BTreeMap<String, Event>,
}

impl A2dp {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            a2dp_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_a2dp_item(&mut self, item: A2dpItem) -> Result<(), A2dpError> {
        if item.name.trim().is_empty() {
            return Err(A2dpError::EmptyA2dpItemName);
        }
        let id = item.id.clone();
        if self.a2dp_items.contains_key(&id) {
            return Err(A2dpError::DuplicateA2dpItem(id.clone()));
        }
        self.a2dp_items.insert(id, item);
        Ok(())
    }

    pub fn a2dp_item_count(&self) -> usize {
        self.a2dp_items.len()
    }

    pub fn get_a2dp_item(&self, id: &str) -> Option<&A2dpItem> {
        self.a2dp_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), A2dpError> {
        if event.subject.trim().is_empty() {
            return Err(A2dpError::EmptyEventSubject);
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

    fn sample_a2dp_item(id: &str, name: &str) -> A2dpItem {
        A2dpItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = A2dp::new("A2dp");
        assert_eq!(m.name, "A2dp");
        assert_eq!(m.a2dp_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_a2dp_item() {
        let mut m = A2dp::new("A2dp");
        m.add_a2dp_item(sample_a2dp_item("a1", "Alpha")).unwrap();
        assert_eq!(m.a2dp_item_count(), 1);
        assert!(m.get_a2dp_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = A2dp::new("A2dp");
        assert!(m.add_a2dp_item(sample_a2dp_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = A2dp::new("A2dp");
        m.add_a2dp_item(sample_a2dp_item("x", "One")).unwrap();
        assert!(m.add_a2dp_item(sample_a2dp_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = A2dp::new("A2dp");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
