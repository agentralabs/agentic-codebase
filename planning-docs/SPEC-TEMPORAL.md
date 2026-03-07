# SPEC-TEMPORAL.md

> Code has memory. Git stores it. We extract patterns, predict failures, detect coupling.

---

## Overview

The Temporal Engine provides:
1. **History extraction**: Parse git commits to understand code evolution
2. **Stability scoring**: Quantify how volatile each code unit is
3. **Coupling detection**: Find units that change together (hidden dependencies)
4. **Prophecy**: Predict what will break based on patterns

---

## Architecture

```
┌─────────────────┐
│   Git Repository │
│   (.git/)        │
└────────┬─────────┘
         │
         ▼
┌─────────────────┐
│  History        │
│  Extractor      │
│  (gix)          │
└────────┬────────┘
         │
    ┌────┴────┐
    │         │
    ▼         ▼
┌────────┐ ┌────────┐
│Change  │ │Commit  │
│Timeline│ │Graph   │
└───┬────┘ └───┬────┘
    │          │
    └────┬─────┘
         │
         ▼
┌─────────────────┐
│  Temporal       │
│  Analyzer       │
└────────┬────────┘
         │
    ┌────┼────┬────────┐
    │    │    │        │
    ▼    ▼    ▼        ▼
┌─────┐┌─────┐┌─────┐┌─────┐
│Stab-││Coup-││Proph││Hot- │
│ility││ling ││ecy  ││spot │
└─────┘└─────┘└─────┘└─────┘
```

---

## History Extractor

```rust
// src/temporal/history.rs

use gix::Repository;

/// Extracts change history from git
pub struct HistoryExtractor {
    repo: Repository,
}

impl HistoryExtractor {
    pub fn open(repo_path: &Path) -> AcbResult<Self> {
        let repo = gix::open(repo_path)
            .map_err(|e| AcbError::GitError(e.to_string()))?;
        Ok(Self { repo })
    }
    
    /// Extract full history for all tracked files
    pub fn extract_history(&self, options: &HistoryOptions) -> AcbResult<ChangeHistory> {
        let mut history = ChangeHistory::new();
        
        // Walk commits from HEAD backwards
        let head = self.repo.head_commit()
            .map_err(|e| AcbError::GitError(e.to_string()))?;
        
        let mut revwalk = self.repo.rev_walk([head.id]);
        revwalk.sorting(gix::traverse::commit::Sorting::ByCommitTimeNewestFirst);
        
        let mut commit_count = 0;
        
        for commit_info in revwalk.all()? {
            let commit_info = commit_info?;
            let commit = commit_info.object()?;
            
            // Skip if beyond time limit
            if let Some(since) = options.since {
                if commit.time().seconds < since as i64 {
                    break;
                }
            }
            
            // Extract changes from this commit
            let changes = self.extract_commit_changes(&commit)?;
            
            for change in changes {
                history.add_change(change);
            }
            
            commit_count += 1;
            if commit_count >= options.max_commits {
                break;
            }
        }
        
        Ok(history)
    }
    
    fn extract_commit_changes(&self, commit: &gix::Commit) -> AcbResult<Vec<FileChange>> {
        let mut changes = Vec::new();
        
        // Get parent (if any)
        let parent_tree = commit.parent_ids()
            .next()
            .and_then(|id| id.object().ok())
            .and_then(|obj| obj.peel_to_tree().ok());
        
        let current_tree = commit.tree()?;
        
        // Diff trees
        let diff = if let Some(parent) = parent_tree {
            parent.changes()?.for_each_to_obtain_tree(&current_tree, |change| {
                changes.push(self.convert_change(change, commit));
                Ok::<_, std::convert::Infallible>(gix::diff::tree::visit::Action::Continue)
            })?;
        } else {
            // Initial commit - all files are added
            current_tree.traverse().breadthfirst(|entry| {
                if entry.mode().is_blob() {
                    changes.push(FileChange {
                        path: entry.filepath().to_string(),
                        change_type: ChangeType::Add,
                        timestamp: commit.time().seconds as u64,
                        commit_hash: commit.id().to_string()[..20].to_string(),
                        author: commit.author().name.to_string(),
                        message: commit.message_raw_sloppy().lines().next()
                            .unwrap_or("").to_string(),
                        is_bugfix: false,
                    });
                }
                gix::traverse::tree::visit::Action::Continue
            })?;
        };
        
        // Detect if this is a bugfix commit
        let is_bugfix = self.is_bugfix_commit(commit);
        for change in &mut changes {
            change.is_bugfix = is_bugfix;
        }
        
        Ok(changes)
    }
    
    fn convert_change(
        &self,
        change: gix::diff::tree::Change,
        commit: &gix::Commit,
    ) -> FileChange {
        let (path, change_type) = match change {
            gix::diff::tree::Change::Addition { entry_mode, oid, path } => {
                (path.to_string(), ChangeType::Add)
            }
            gix::diff::tree::Change::Deletion { entry_mode, oid, path } => {
                (path.to_string(), ChangeType::Delete)
            }
            gix::diff::tree::Change::Modification { previous_entry_mode, previous_oid, entry_mode, oid, path } => {
                (path.to_string(), ChangeType::Modify)
            }
            gix::diff::tree::Change::Rewrite { source_path, source_entry_mode, source_oid, entry_mode, oid, path, .. } => {
                (path.to_string(), ChangeType::Rename)
            }
        };
        
        FileChange {
            path,
            change_type,
            timestamp: commit.time().seconds as u64,
            commit_hash: commit.id().to_string()[..20].to_string(),
            author: commit.author().name.to_string(),
            message: commit.message_raw_sloppy().lines().next()
                .unwrap_or("").to_string(),
            is_bugfix: false,
        }
    }
    
    fn is_bugfix_commit(&self, commit: &gix::Commit) -> bool {
        let message = commit.message_raw_sloppy().to_lowercase();
        
        // Common bugfix indicators
        message.contains("fix") ||
        message.contains("bug") ||
        message.contains("patch") ||
        message.contains("hotfix") ||
        message.contains("issue") ||
        message.contains("resolve") ||
        message.contains("repair") ||
        message.starts_with("revert")
    }
}

#[derive(Debug, Clone)]
pub struct HistoryOptions {
    /// Only include commits since this timestamp
    pub since: Option<u64>,
    /// Maximum number of commits to process
    pub max_commits: usize,
    /// Include merge commits
    pub include_merges: bool,
}

impl Default for HistoryOptions {
    fn default() -> Self {
        Self {
            since: None,
            max_commits: 10000,
            include_merges: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FileChange {
    pub path: String,
    pub change_type: ChangeType,
    pub timestamp: u64,
    pub commit_hash: String,
    pub author: String,
    pub message: String,
    pub is_bugfix: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChangeType {
    Add,
    Modify,
    Delete,
    Rename,
}
```

---

## Change History

```rust
// src/temporal/history.rs

/// Accumulated change history for a codebase
pub struct ChangeHistory {
    /// Changes grouped by file path
    by_path: HashMap<String, Vec<FileChange>>,
    /// All changes in chronological order
    chronological: Vec<FileChange>,
    /// Commit to files mapping
    commits: HashMap<String, Vec<String>>,
}

impl ChangeHistory {
    pub fn new() -> Self {
        Self {
            by_path: HashMap::new(),
            chronological: Vec::new(),
            commits: HashMap::new(),
        }
    }
    
    pub fn add_change(&mut self, change: FileChange) {
        // Add to path index
        self.by_path
            .entry(change.path.clone())
            .or_default()
            .push(change.clone());
        
        // Add to commit index
        self.commits
            .entry(change.commit_hash.clone())
            .or_default()
            .push(change.path.clone());
        
        // Add to chronological list
        self.chronological.push(change);
    }
    
    /// Get all changes for a file
    pub fn changes_for_path(&self, path: &str) -> &[FileChange] {
        self.by_path.get(path).map(|v| v.as_slice()).unwrap_or(&[])
    }
    
    /// Get all files changed in a commit
    pub fn files_in_commit(&self, commit: &str) -> &[String] {
        self.commits.get(commit).map(|v| v.as_slice()).unwrap_or(&[])
    }
    
    /// Get change count for a file
    pub fn change_count(&self, path: &str) -> usize {
        self.by_path.get(path).map(|v| v.len()).unwrap_or(0)
    }
    
    /// Get bugfix count for a file
    pub fn bugfix_count(&self, path: &str) -> usize {
        self.by_path.get(path)
            .map(|changes| changes.iter().filter(|c| c.is_bugfix).count())
            .unwrap_or(0)
    }
    
    /// Get all commits
    pub fn all_commits(&self) -> impl Iterator<Item = &str> {
        self.commits.keys().map(|s| s.as_str())
    }
}
```

---

## Stability Analyzer

```rust
// src/temporal/stability.rs

/// Calculates stability scores for code units
pub struct StabilityAnalyzer;

impl StabilityAnalyzer {
    /// Calculate stability score for a code unit
    /// 
    /// Score ranges from 0.0 (very unstable) to 1.0 (very stable)
    pub fn calculate_stability(
        unit: &CodeUnit,
        history: &ChangeHistory,
        options: &StabilityOptions,
    ) -> StabilityResult {
        let path = unit.file_path.to_string_lossy();
        let changes = history.changes_for_path(&path);
        
        if changes.is_empty() {
            return StabilityResult {
                score: 1.0,
                factors: vec![StabilityFactor {
                    name: "No history".to_string(),
                    impact: 0.0,
                    detail: "File has no git history".to_string(),
                }],
                recommendation: StabilityRecommendation::SafeToModify,
            };
        }
        
        // Calculate individual factors
        let change_frequency = Self::change_frequency_factor(changes, options);
        let bugfix_ratio = Self::bugfix_ratio_factor(changes);
        let recent_activity = Self::recent_activity_factor(changes, options);
        let author_concentration = Self::author_concentration_factor(changes);
        let churn = Self::churn_factor(changes, options);
        
        // Weighted combination
        let score = (
            change_frequency.impact * 0.25 +
            bugfix_ratio.impact * 0.30 +
            recent_activity.impact * 0.20 +
            author_concentration.impact * 0.10 +
            churn.impact * 0.15
        ).clamp(0.0, 1.0);
        
        let factors = vec![
            change_frequency,
            bugfix_ratio,
            recent_activity,
            author_concentration,
            churn,
        ];
        
        let recommendation = Self::recommend(score, &factors);
        
        StabilityResult {
            score,
            factors,
            recommendation,
        }
    }
    
    fn change_frequency_factor(
        changes: &[FileChange],
        options: &StabilityOptions,
    ) -> StabilityFactor {
        let days = options.analysis_window_days as f32;
        let changes_per_month = (changes.len() as f32 / days) * 30.0;
        
        // Score: <1 change/month = 1.0, >10 changes/month = 0.0
        let impact = 1.0 - (changes_per_month / 10.0).min(1.0);
        
        StabilityFactor {
            name: "Change frequency".to_string(),
            impact,
            detail: format!("{:.1} changes/month", changes_per_month),
        }
    }
    
    fn bugfix_ratio_factor(changes: &[FileChange]) -> StabilityFactor {
        if changes.is_empty() {
            return StabilityFactor {
                name: "Bugfix ratio".to_string(),
                impact: 1.0,
                detail: "No changes".to_string(),
            };
        }
        
        let bugfixes = changes.iter().filter(|c| c.is_bugfix).count();
        let ratio = bugfixes as f32 / changes.len() as f32;
        
        // Score: 0% bugfixes = 1.0, 50%+ bugfixes = 0.0
        let impact = 1.0 - (ratio * 2.0).min(1.0);
        
        StabilityFactor {
            name: "Bugfix ratio".to_string(),
            impact,
            detail: format!("{}/{} changes were bugfixes ({:.0}%)", 
                bugfixes, changes.len(), ratio * 100.0),
        }
    }
    
    fn recent_activity_factor(
        changes: &[FileChange],
        options: &StabilityOptions,
    ) -> StabilityFactor {
        let now = crate::types::now_micros() / 1_000_000; // to seconds
        let recent_threshold = now - (7 * 24 * 3600); // Last 7 days
        
        let recent_changes = changes.iter()
            .filter(|c| c.timestamp > recent_threshold)
            .count();
        
        // Score: 0 recent changes = 1.0, 5+ recent changes = 0.0
        let impact = 1.0 - (recent_changes as f32 / 5.0).min(1.0);
        
        StabilityFactor {
            name: "Recent activity".to_string(),
            impact,
            detail: format!("{} changes in last 7 days", recent_changes),
        }
    }
    
    fn author_concentration_factor(changes: &[FileChange]) -> StabilityFactor {
        if changes.is_empty() {
            return StabilityFactor {
                name: "Author concentration".to_string(),
                impact: 1.0,
                detail: "No changes".to_string(),
            };
        }
        
        // Count unique authors
        let authors: HashSet<_> = changes.iter().map(|c| &c.author).collect();
        let author_count = authors.len();
        
        // Single author = good (knowledge concentrated)
        // Many authors = potentially risky (less ownership)
        // Sweet spot is 2-3 authors
        let impact = match author_count {
            1 => 0.8,
            2..=3 => 1.0,
            4..=5 => 0.7,
            _ => 0.5,
        };
        
        StabilityFactor {
            name: "Author concentration".to_string(),
            impact,
            detail: format!("{} different authors", author_count),
        }
    }
    
    fn churn_factor(
        changes: &[FileChange],
        options: &StabilityOptions,
    ) -> StabilityFactor {
        // Churn = changes followed quickly by more changes
        // High churn suggests instability
        
        let mut churn_events = 0;
        let churn_window = 24 * 3600; // 24 hours
        
        for i in 1..changes.len() {
            let time_diff = changes[i-1].timestamp.abs_diff(changes[i].timestamp);
            if time_diff < churn_window {
                churn_events += 1;
            }
        }
        
        let churn_ratio = if changes.len() > 1 {
            churn_events as f32 / (changes.len() - 1) as f32
        } else {
            0.0
        };
        
        // Score: 0% churn = 1.0, 50%+ churn = 0.0
        let impact = 1.0 - (churn_ratio * 2.0).min(1.0);
        
        StabilityFactor {
            name: "Code churn".to_string(),
            impact,
            detail: format!("{:.0}% of changes followed by quick changes", churn_ratio * 100.0),
        }
    }
    
    fn recommend(score: f32, factors: &[StabilityFactor]) -> StabilityRecommendation {
        if score >= 0.8 {
            StabilityRecommendation::SafeToModify
        } else if score >= 0.5 {
            let worst = factors.iter()
                .min_by(|a, b| a.impact.partial_cmp(&b.impact).unwrap())
                .map(|f| f.name.clone())
                .unwrap_or_default();
            StabilityRecommendation::ProceedWithCaution {
                reason: format!("Concern: {}", worst),
            }
        } else if score >= 0.3 {
            StabilityRecommendation::ConsiderRefactoring {
                suggestion: "Extract volatile logic into separate unit".to_string(),
            }
        } else {
            StabilityRecommendation::HighRisk {
                mitigation: "Add comprehensive tests before modifying".to_string(),
            }
        }
    }
}

#[derive(Debug)]
pub struct StabilityOptions {
    pub analysis_window_days: u32,
}

impl Default for StabilityOptions {
    fn default() -> Self {
        Self {
            analysis_window_days: 180, // 6 months
        }
    }
}

#[derive(Debug)]
pub struct StabilityResult {
    pub score: f32,
    pub factors: Vec<StabilityFactor>,
    pub recommendation: StabilityRecommendation,
}

#[derive(Debug)]
pub struct StabilityFactor {
    pub name: String,
    pub impact: f32,
    pub detail: String,
}

#[derive(Debug)]
pub enum StabilityRecommendation {
    SafeToModify,
    ProceedWithCaution { reason: String },
    ConsiderRefactoring { suggestion: String },
    HighRisk { mitigation: String },
}
```

---

## Coupling Detector

```rust
// src/temporal/coupling.rs

/// Detects hidden coupling between code units via co-change analysis
pub struct CouplingDetector;

impl CouplingDetector {
    /// Detect all couplings above threshold
    pub fn detect_all(
        history: &ChangeHistory,
        graph: &CodeGraph,
        options: &CouplingOptions,
    ) -> Vec<Coupling> {
        let mut couplings = Vec::new();
        
        // Build co-change matrix
        let cochange = Self::build_cochange_matrix(history);
        
        // For each pair with significant co-change, create coupling
        for ((path_a, path_b), count) in &cochange {
            let changes_a = history.change_count(path_a);
            let changes_b = history.change_count(path_b);
            
            let strength = *count as f32 / changes_a.min(changes_b) as f32;
            
            if strength >= options.min_coupling_strength {
                // Map paths to unit IDs
                let units_a = Self::find_units_for_path(graph, path_a);
                let units_b = Self::find_units_for_path(graph, path_b);
                
                // Check if there's an explicit dependency
                let is_explicit = Self::has_explicit_edge(graph, &units_a, &units_b);
                
                let coupling_type = if is_explicit {
                    CouplingType::Explicit
                } else if strength > 0.7 {
                    CouplingType::Hidden
                } else {
                    CouplingType::Temporal
                };
                
                for &unit_a in &units_a {
                    for &unit_b in &units_b {
                        if unit_a != unit_b {
                            couplings.push(Coupling {
                                unit_a,
                                unit_b,
                                coupling_type,
                                strength,
                                co_change_count: *count,
                                evidence: format!(
                                    "Changed together {} times out of {}/{} changes",
                                    count, changes_a, changes_b
                                ),
                            });
                        }
                    }
                }
            }
        }
        
        // Sort by strength descending
        couplings.sort_by(|a, b| b.strength.partial_cmp(&a.strength).unwrap());
        
        couplings
    }
    
    /// Detect couplings for a specific unit
    pub fn detect_for_unit(
        unit_id: u64,
        history: &ChangeHistory,
        graph: &CodeGraph,
        options: &CouplingOptions,
    ) -> Vec<Coupling> {
        let unit = graph.get_unit(unit_id).ok_or(AcbError::UnitNotFound(unit_id))?;
        let path = unit.file_path.to_string_lossy().to_string();
        
        let all = Self::detect_all(history, graph, options);
        all.into_iter()
            .filter(|c| c.unit_a == unit_id || c.unit_b == unit_id)
            .collect()
    }
    
    fn build_cochange_matrix(
        history: &ChangeHistory,
    ) -> HashMap<(String, String), usize> {
        let mut cochange: HashMap<(String, String), usize> = HashMap::new();
        
        // For each commit, record which files changed together
        for commit in history.all_commits() {
            let files = history.files_in_commit(commit);
            
            // All pairs of files in this commit
            for i in 0..files.len() {
                for j in (i+1)..files.len() {
                    let key = if files[i] < files[j] {
                        (files[i].clone(), files[j].clone())
                    } else {
                        (files[j].clone(), files[i].clone())
                    };
                    *cochange.entry(key).or_insert(0) += 1;
                }
            }
        }
        
        cochange
    }
    
    fn find_units_for_path(graph: &CodeGraph, path: &str) -> Vec<u64> {
        graph.all_units()
            .filter(|u| u.file_path.to_string_lossy() == path)
            .map(|u| u.id)
            .collect()
    }
    
    fn has_explicit_edge(
        graph: &CodeGraph,
        units_a: &[u64],
        units_b: &[u64],
    ) -> bool {
        for &a in units_a {
            for &b in units_b {
                if graph.has_edge(a, b) || graph.has_edge(b, a) {
                    return true;
                }
            }
        }
        false
    }
}

#[derive(Debug)]
pub struct CouplingOptions {
    pub min_coupling_strength: f32,
    pub min_cochange_count: usize,
}

impl Default for CouplingOptions {
    fn default() -> Self {
        Self {
            min_coupling_strength: 0.5,
            min_cochange_count: 3,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Coupling {
    pub unit_a: u64,
    pub unit_b: u64,
    pub coupling_type: CouplingType,
    pub strength: f32,
    pub co_change_count: usize,
    pub evidence: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CouplingType {
    /// Direct call/import relationship
    Explicit,
    /// Strong co-change but no explicit relationship (danger!)
    Hidden,
    /// Moderate co-change pattern
    Temporal,
}
```

---

## Prophecy Engine

```rust
// src/temporal/prophecy.rs

/// Predicts future code issues based on historical patterns
pub struct ProphecyEngine;

impl ProphecyEngine {
    /// Generate predictions for the codebase
    pub fn predict(
        graph: &CodeGraph,
        history: &ChangeHistory,
        options: &ProphecyOptions,
    ) -> ProphecyResult {
        let mut predictions = Vec::new();
        
        // Analyze each unit
        for unit in graph.all_units() {
            if let Some(prediction) = Self::predict_for_unit(unit, history, graph, options) {
                predictions.push(prediction);
            }
        }
        
        // Sort by confidence descending
        predictions.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
        
        // Get ecosystem alerts
        let ecosystem_alerts = Self::check_ecosystem(graph, options);
        
        ProphecyResult {
            predictions,
            ecosystem_alerts,
        }
    }
    
    fn predict_for_unit(
        unit: &CodeUnit,
        history: &ChangeHistory,
        graph: &CodeGraph,
        options: &ProphecyOptions,
    ) -> Option<Prediction> {
        let path = unit.file_path.to_string_lossy();
        let changes = history.changes_for_path(&path);
        
        if changes.len() < 3 {
            return None; // Not enough data
        }
        
        // Calculate prediction factors
        let velocity = Self::calculate_velocity(changes);
        let bugfix_trend = Self::calculate_bugfix_trend(changes);
        let complexity_growth = Self::estimate_complexity_growth(unit, history);
        let coupling_risk = Self::calculate_coupling_risk(unit, history, graph);
        
        // Determine prediction type and confidence
        let (prediction_type, confidence, reasoning) = 
            Self::determine_prediction(velocity, bugfix_trend, complexity_growth, coupling_risk);
        
        if confidence < options.min_confidence {
            return None;
        }
        
        let estimated_days = Self::estimate_time_to_incident(
            velocity, bugfix_trend, options.time_horizon_days
        );
        
        let recommendation = Self::generate_recommendation(&prediction_type, unit);
        
        Some(Prediction {
            unit_id: unit.id,
            prediction_type,
            confidence,
            estimated_days,
            reasoning,
            recommendation,
        })
    }
    
    fn calculate_velocity(changes: &[FileChange]) -> f32 {
        // Changes per week, weighted by recency
        if changes.len() < 2 {
            return 0.0;
        }
        
        let now = crate::types::now_micros() / 1_000_000;
        let week_seconds = 7 * 24 * 3600;
        
        let mut weighted_count = 0.0;
        for change in changes {
            let age_weeks = (now - change.timestamp) / week_seconds as u64;
            let weight = 1.0 / (1.0 + age_weeks as f32 * 0.1);
            weighted_count += weight;
        }
        
        weighted_count
    }
    
    fn calculate_bugfix_trend(changes: &[FileChange]) -> f32 {
        // Is the bugfix ratio increasing over time?
        if changes.len() < 6 {
            return 0.0;
        }
        
        let mid = changes.len() / 2;
        let older = &changes[..mid];
        let newer = &changes[mid..];
        
        let older_ratio = older.iter().filter(|c| c.is_bugfix).count() as f32 / older.len() as f32;
        let newer_ratio = newer.iter().filter(|c| c.is_bugfix).count() as f32 / newer.len() as f32;
        
        newer_ratio - older_ratio // Positive = getting worse
    }
    
    fn estimate_complexity_growth(
        unit: &CodeUnit,
        history: &ChangeHistory,
    ) -> f32 {
        // Estimate based on change frequency and current complexity
        let changes = history.change_count(&unit.file_path.to_string_lossy());
        let complexity = unit.complexity as f32;
        
        // High complexity + high changes = likely growing
        (complexity / 20.0).min(1.0) * (changes as f32 / 50.0).min(1.0)
    }
    
    fn calculate_coupling_risk(
        unit: &CodeUnit,
        history: &ChangeHistory,
        graph: &CodeGraph,
    ) -> f32 {
        let couplings = CouplingDetector::detect_for_unit(
            unit.id,
            history,
            graph,
            &CouplingOptions::default(),
        );
        
        // Hidden couplings are risky
        couplings.iter()
            .filter(|c| c.coupling_type == CouplingType::Hidden)
            .map(|c| c.strength)
            .sum::<f32>()
            .min(1.0)
    }
    
    fn determine_prediction(
        velocity: f32,
        bugfix_trend: f32,
        complexity_growth: f32,
        coupling_risk: f32,
    ) -> (PredictionType, f32, String) {
        let mut reasons = Vec::new();
        let mut confidence = 0.0;
        
        // High velocity + high bugfix trend = likely to break
        if velocity > 0.5 && bugfix_trend > 0.1 {
            confidence += 0.4;
            reasons.push(format!(
                "High change velocity ({:.1}/week) with increasing bugfix ratio (+{:.0}%)",
                velocity, bugfix_trend * 100.0
            ));
        }
        
        // High complexity growth
        if complexity_growth > 0.5 {
            confidence += 0.3;
            reasons.push("Complexity increasing over time".to_string());
        }
        
        // Hidden coupling risk
        if coupling_risk > 0.3 {
            confidence += 0.3;
            reasons.push(format!("Hidden couplings detected (risk: {:.0}%)", coupling_risk * 100.0));
        }
        
        let prediction_type = if confidence > 0.7 {
            PredictionType::LikelyToBreak
        } else if complexity_growth > 0.6 {
            PredictionType::TechDebtAccumulating
        } else if velocity > 0.8 {
            PredictionType::NeedsRefactoring
        } else {
            PredictionType::TestCoverageDecaying
        };
        
        (prediction_type, confidence, reasons.join("; "))
    }
    
    fn estimate_time_to_incident(
        velocity: f32,
        bugfix_trend: f32,
        horizon_days: u32,
    ) -> u32 {
        // Higher velocity and bugfix trend = sooner incident
        let risk_factor = velocity * (1.0 + bugfix_trend);
        let estimated = (horizon_days as f32 / risk_factor.max(0.1)) as u32;
        estimated.min(horizon_days)
    }
    
    fn generate_recommendation(
        prediction_type: &PredictionType,
        unit: &CodeUnit,
    ) -> String {
        match prediction_type {
            PredictionType::LikelyToBreak => {
                "Stabilize with tests and reduce change frequency".to_string()
            }
            PredictionType::NeedsRefactoring => {
                format!("Extract {} into smaller, focused units", unit.name)
            }
            PredictionType::TechDebtAccumulating => {
                "Schedule dedicated refactoring time".to_string()
            }
            PredictionType::TestCoverageDecaying => {
                "Add tests for recent changes".to_string()
            }
        }
    }
    
    fn check_ecosystem(
        graph: &CodeGraph,
        options: &ProphecyOptions,
    ) -> Vec<EcosystemAlert> {
        // Check dependencies against collective knowledge
        // This would query the collective registry
        Vec::new() // Placeholder - actual impl uses collective
    }
}

#[derive(Debug)]
pub struct ProphecyOptions {
    pub time_horizon_days: u32,
    pub min_confidence: f32,
    pub include_ecosystem: bool,
}

impl Default for ProphecyOptions {
    fn default() -> Self {
        Self {
            time_horizon_days: 30,
            min_confidence: 0.5,
            include_ecosystem: true,
        }
    }
}

#[derive(Debug)]
pub struct ProphecyResult {
    pub predictions: Vec<Prediction>,
    pub ecosystem_alerts: Vec<EcosystemAlert>,
}

#[derive(Debug)]
pub struct Prediction {
    pub unit_id: u64,
    pub prediction_type: PredictionType,
    pub confidence: f32,
    pub estimated_days: u32,
    pub reasoning: String,
    pub recommendation: String,
}

#[derive(Debug, Clone, Copy)]
pub enum PredictionType {
    LikelyToBreak,
    NeedsRefactoring,
    TechDebtAccumulating,
    TestCoverageDecaying,
}

#[derive(Debug)]
pub struct EcosystemAlert {
    pub library: String,
    pub current_version: String,
    pub alert_type: AlertType,
    pub affected_percentage: f32,
    pub recommendation: String,
}

#[derive(Debug)]
pub enum AlertType {
    BreakingChange,
    SecurityVulnerability,
    Deprecation,
    PerformanceRegression,
}
```

---

## Temporal Data Serialization

```rust
// src/temporal/mod.rs

/// Serialize temporal data for .acb file
pub fn serialize_temporal(
    history: &ChangeHistory,
    couplings: &[Coupling],
) -> Vec<u8> {
    let mut bytes = Vec::new();
    
    // History section
    let history_json = serde_json::to_vec(&history.chronological).unwrap();
    let history_compressed = lz4_flex::compress_prepend_size(&history_json);
    bytes.extend(&(history_compressed.len() as u64).to_le_bytes());
    bytes.extend(&history_compressed);
    
    // Coupling section
    bytes.extend(&(couplings.len() as u64).to_le_bytes());
    for coupling in couplings {
        bytes.extend(&coupling.unit_a.to_le_bytes());
        bytes.extend(&coupling.unit_b.to_le_bytes());
        bytes.extend(&(coupling.coupling_type as u8).to_le_bytes());
        bytes.extend(&coupling.strength.to_le_bytes());
        bytes.extend(&(coupling.co_change_count as u32).to_le_bytes());
    }
    
    bytes
}

/// Deserialize temporal data from .acb file
pub fn deserialize_temporal(bytes: &[u8]) -> AcbResult<(ChangeHistory, Vec<Coupling>)> {
    let mut offset = 0;
    
    // History section
    let history_len = u64::from_le_bytes(
        bytes[offset..offset+8].try_into().unwrap()
    ) as usize;
    offset += 8;
    
    let history_compressed = &bytes[offset..offset+history_len];
    offset += history_len;
    
    let history_json = lz4_flex::decompress_size_prepended(history_compressed)
        .map_err(|e| AcbError::Compression(e.to_string()))?;
    let changes: Vec<FileChange> = serde_json::from_slice(&history_json)?;
    
    let mut history = ChangeHistory::new();
    for change in changes {
        history.add_change(change);
    }
    
    // Coupling section
    let coupling_count = u64::from_le_bytes(
        bytes[offset..offset+8].try_into().unwrap()
    ) as usize;
    offset += 8;
    
    let mut couplings = Vec::with_capacity(coupling_count);
    for _ in 0..coupling_count {
        let unit_a = u64::from_le_bytes(bytes[offset..offset+8].try_into().unwrap());
        offset += 8;
        let unit_b = u64::from_le_bytes(bytes[offset..offset+8].try_into().unwrap());
        offset += 8;
        let coupling_type = match bytes[offset] {
            0 => CouplingType::Explicit,
            1 => CouplingType::Hidden,
            _ => CouplingType::Temporal,
        };
        offset += 1;
        let strength = f32::from_le_bytes(bytes[offset..offset+4].try_into().unwrap());
        offset += 4;
        let co_change_count = u32::from_le_bytes(bytes[offset..offset+4].try_into().unwrap()) as usize;
        offset += 4;
        
        couplings.push(Coupling {
            unit_a,
            unit_b,
            coupling_type,
            strength,
            co_change_count,
            evidence: String::new(),
        });
    }
    
    Ok((history, couplings))
}
```

---

## Performance Targets

| Operation | Target |
|-----------|--------|
| History extraction (10K commits) | <10s |
| Stability calculation (per unit) | <1ms |
| Coupling detection (100K units) | <5s |
| Prophecy generation | <2s |
| Temporal serialization | <500ms |
