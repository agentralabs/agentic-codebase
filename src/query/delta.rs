use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Tracks versioned state for delta-based token conservation.
///
/// Instead of returning full state on every query, only changes
/// since the last known version are returned.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionedState<T> {
    /// The current version counter (monotonically increasing).
    version: u64,
    /// Timestamp of the last modification.
    last_modified: DateTime<Utc>,
    /// The current state.
    state: T,
    /// History of changes for delta computation.
    changes: Vec<ChangeRecord<T>>,
    /// Maximum number of change records to retain.
    max_history: usize,
}

/// A single recorded change.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeRecord<T> {
    /// The version at which this change occurred.
    pub version: u64,
    /// Timestamp of the change.
    pub timestamp: DateTime<Utc>,
    /// The type of change.
    pub change_type: ChangeType,
    /// The value after the change.
    pub value: T,
}

/// The type of change that occurred.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChangeType {
    /// A new item was created.
    Created,
    /// An existing item was updated.
    Updated,
    /// An item was deleted.
    Deleted,
}

/// Result of a delta query -- either unchanged or a set of changes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeltaResult<T> {
    /// No changes since the requested version.
    Unchanged {
        /// The current version.
        version: u64,
    },
    /// Changes that occurred since the requested version.
    Changed {
        /// The current version.
        version: u64,
        /// The changes since the requested version.
        changes: Vec<ChangeRecord<T>>,
    },
}

impl<T> DeltaResult<T> {
    /// Whether the state is unchanged.
    pub fn is_unchanged(&self) -> bool {
        matches!(self, DeltaResult::Unchanged { .. })
    }

    /// The number of changes (0 if unchanged).
    pub fn change_count(&self) -> usize {
        match self {
            DeltaResult::Unchanged { .. } => 0,
            DeltaResult::Changed { changes, .. } => changes.len(),
        }
    }

    /// The current version.
    pub fn version(&self) -> u64 {
        match self {
            DeltaResult::Unchanged { version } => *version,
            DeltaResult::Changed { version, .. } => *version,
        }
    }
}

impl<T: Clone> VersionedState<T> {
    /// Create a new versioned state with the initial value.
    pub fn new(initial: T) -> Self {
        Self {
            version: 0,
            last_modified: Utc::now(),
            state: initial,
            changes: Vec::new(),
            max_history: 100,
        }
    }

    /// Create a new versioned state with a custom history limit.
    pub fn with_max_history(initial: T, max_history: usize) -> Self {
        Self {
            version: 0,
            last_modified: Utc::now(),
            state: initial,
            changes: Vec::new(),
            max_history,
        }
    }

    /// Get the current version number.
    pub fn version(&self) -> u64 {
        self.version
    }

    /// Get the timestamp of the last modification.
    pub fn last_modified(&self) -> DateTime<Utc> {
        self.last_modified
    }

    /// Get a reference to the current state.
    pub fn state(&self) -> &T {
        &self.state
    }

    /// Record a change, incrementing the version and updating the timestamp.
    pub fn record_change(&mut self, change_type: ChangeType, new_state: T) {
        self.version += 1;
        self.last_modified = Utc::now();
        self.state = new_state.clone();

        self.changes.push(ChangeRecord {
            version: self.version,
            timestamp: self.last_modified,
            change_type,
            value: new_state,
        });

        // Trim history if exceeding limit
        if self.changes.len() > self.max_history {
            let drain_count = self.changes.len() - self.max_history;
            self.changes.drain(..drain_count);
        }
    }

    /// Get all changes since a given version.
    pub fn changes_since_version(&self, since_version: u64) -> DeltaResult<T> {
        if since_version >= self.version {
            return DeltaResult::Unchanged {
                version: self.version,
            };
        }

        let changes: Vec<ChangeRecord<T>> = self
            .changes
            .iter()
            .filter(|c| c.version > since_version)
            .cloned()
            .collect();

        if changes.is_empty() {
            DeltaResult::Unchanged {
                version: self.version,
            }
        } else {
            DeltaResult::Changed {
                version: self.version,
                changes,
            }
        }
    }

    /// Check whether the state has changed since a given version.
    pub fn is_unchanged_since(&self, version: u64) -> bool {
        version >= self.version
    }

    /// Get the total number of recorded changes in history.
    pub fn history_len(&self) -> usize {
        self.changes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_version() {
        let state = VersionedState::new(42);
        assert_eq!(state.version(), 0);
        assert_eq!(*state.state(), 42);
    }

    #[test]
    fn test_record_change_increments_version() {
        let mut state = VersionedState::new(0);
        state.record_change(ChangeType::Updated, 1);
        assert_eq!(state.version(), 1);
        assert_eq!(*state.state(), 1);
    }

    #[test]
    fn test_changes_since_version() {
        let mut state = VersionedState::new(0);
        state.record_change(ChangeType::Updated, 1);
        state.record_change(ChangeType::Updated, 2);
        state.record_change(ChangeType::Updated, 3);

        let delta = state.changes_since_version(1);
        assert!(!delta.is_unchanged());
        assert_eq!(delta.change_count(), 2); // versions 2 and 3
    }

    #[test]
    fn test_unchanged_since_current() {
        let state = VersionedState::new(42);
        let delta = state.changes_since_version(0);
        assert!(delta.is_unchanged());
    }

    #[test]
    fn test_is_unchanged_since() {
        let mut state = VersionedState::new(0);
        state.record_change(ChangeType::Created, 1);
        assert!(state.is_unchanged_since(1));
        assert!(!state.is_unchanged_since(0));
    }

    #[test]
    fn test_change_types() {
        let mut state = VersionedState::new("initial".to_string());
        state.record_change(ChangeType::Created, "created".to_string());
        state.record_change(ChangeType::Updated, "updated".to_string());
        state.record_change(ChangeType::Deleted, "deleted".to_string());
        assert_eq!(state.version(), 3);
        assert_eq!(state.history_len(), 3);
    }

    #[test]
    fn test_history_trimming() {
        let mut state = VersionedState::with_max_history(0, 3);
        for i in 1..=5 {
            state.record_change(ChangeType::Updated, i);
        }
        assert_eq!(state.history_len(), 3);
        assert_eq!(state.version(), 5);
    }

    #[test]
    fn test_delta_result_version() {
        let delta: DeltaResult<i32> = DeltaResult::Unchanged { version: 42 };
        assert_eq!(delta.version(), 42);
    }

    #[test]
    fn test_serialization() {
        let change_type = ChangeType::Updated;
        let json = serde_json::to_string(&change_type).unwrap();
        let back: ChangeType = serde_json::from_str(&json).unwrap();
        assert_eq!(change_type, back);
    }

    #[test]
    fn test_delta_proportional() {
        let mut state = VersionedState::new(0);
        // Add 10 changes
        for i in 1..=10 {
            state.record_change(ChangeType::Updated, i);
        }
        // Request last 3 changes
        let delta = state.changes_since_version(7);
        assert_eq!(delta.change_count(), 3);
        // Request last 1 change
        let delta = state.changes_since_version(9);
        assert_eq!(delta.change_count(), 1);
    }
}
