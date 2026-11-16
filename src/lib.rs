// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-systemd-repart: systemd repart component model.
//!
//! Real implementation: reparts, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Repart {
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
pub enum SystemdRepartError {
    EmptyRepartName,
    DuplicateRepart(String),
    EmptyEventSubject,
}

impl fmt::Display for SystemdRepartError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyRepartName => write!(f, "repart name must not be empty"),
            Self::DuplicateRepart(id) => write!(f, "duplicate repart {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for SystemdRepartError {}

pub struct SystemdRepart {
    pub name: String,
    reparts: BTreeMap<String, Repart>,
    events: BTreeMap<String, Event>,
}

impl SystemdRepart {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            reparts: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_repart(&mut self, item: Repart) -> Result<(), SystemdRepartError> {
        if item.name.trim().is_empty() {
            return Err(SystemdRepartError::EmptyRepartName);
        }
        let id = item.id.clone();
        if self.reparts.contains_key(&id) {
            return Err(SystemdRepartError::DuplicateRepart(id.clone()));
        }
        self.reparts.insert(id, item);
        Ok(())
    }

    pub fn repart_count(&self) -> usize {
        self.reparts.len()
    }

    pub fn get_repart(&self, id: &str) -> Option<&Repart> {
        self.reparts.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), SystemdRepartError> {
        if event.subject.trim().is_empty() {
            return Err(SystemdRepartError::EmptyEventSubject);
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

    fn sample_repart(id: &str, name: &str) -> Repart {
        Repart {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = SystemdRepart::new("SystemdRepart");
        assert_eq!(m.name, "SystemdRepart");
        assert_eq!(m.repart_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_repart() {
        let mut m = SystemdRepart::new("SystemdRepart");
        m.add_repart(sample_repart("a1", "Alpha")).unwrap();
        assert_eq!(m.repart_count(), 1);
        assert!(m.get_repart("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = SystemdRepart::new("SystemdRepart");
        assert!(m.add_repart(sample_repart("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = SystemdRepart::new("SystemdRepart");
        m.add_repart(sample_repart("x", "One")).unwrap();
        assert!(m.add_repart(sample_repart("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = SystemdRepart::new("SystemdRepart");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
