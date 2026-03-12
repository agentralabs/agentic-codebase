//! Design pattern detection.
//!
//! Detects common design patterns in code: Singleton, Factory, Repository,
//! Decorator, Observer, Strategy patterns.

use std::collections::{HashMap, HashSet};

use crate::types::{AcbResult, CodeUnitType, Visibility};

use super::resolver::ResolvedUnit;

/// Detects common design patterns in code.
pub struct PatternDetector {
    /// Pattern matchers to run.
    matchers: Vec<Box<dyn PatternMatcher>>,
}

/// A detected pattern instance.
#[derive(Debug, Clone)]
pub struct PatternInstance {
    /// The pattern name.
    pub pattern_name: String,
    /// The primary unit involved.
    pub primary_unit: u64,
    /// All units participating in the pattern.
    pub participating_units: Vec<u64>,
    /// Confidence score (0.0 to 1.0).
    pub confidence: f32,
}

/// Trait for pattern matchers.
trait PatternMatcher: Send + Sync {
    /// Detect instances of this pattern.
    fn detect(&self, units: &[ResolvedUnit]) -> Vec<PatternInstance>;
}

impl PatternDetector {
    /// Create a new pattern detector with all built-in matchers.
    pub fn new() -> Self {
        let matchers: Vec<Box<dyn PatternMatcher>> = vec![
            Box::new(SingletonMatcher),
            Box::new(FactoryMatcher),
            Box::new(RepositoryMatcher),
            Box::new(DecoratorMatcher),
        ];
        Self { matchers }
    }

    /// Detect all patterns in the resolved units.
    pub fn detect(&self, units: &[ResolvedUnit]) -> AcbResult<Vec<PatternInstance>> {
        let mut instances = Vec::new();

        for matcher in &self.matchers {
            instances.extend(matcher.detect(units));
        }

        Ok(instances)
    }
}

impl Default for PatternDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Detects Singleton pattern: classes with private constructors and
/// static instance access methods.
struct SingletonMatcher;

impl PatternMatcher for SingletonMatcher {
    fn detect(&self, units: &[ResolvedUnit]) -> Vec<PatternInstance> {
        let mut instances = Vec::new();
        let (functions_by_owner, functions_by_file) = build_function_indexes(units);

        for unit in units {
            if unit.unit.unit_type != CodeUnitType::Type {
                continue;
            }

            let type_name_lower = unit.unit.name.to_lowercase();
            let candidate_methods =
                gather_candidate_methods(unit, &functions_by_owner, &functions_by_file);

            let mut has_instance_method = false;
            let mut has_private_constructor = false;
            let mut participants = vec![unit.unit.temp_id];
            let mut seen = HashSet::from([unit.unit.temp_id]);

            for other in candidate_methods {
                if !method_belongs_to_type(other, &type_name_lower) {
                    continue;
                }

                let other_name_lower = other.unit.name.to_lowercase();

                // Check for get_instance, instance, or shared patterns
                if other_name_lower.contains("instance")
                    || other_name_lower.contains("shared")
                    || other_name_lower == "default"
                {
                    has_instance_method = true;
                    if seen.insert(other.unit.temp_id) {
                        participants.push(other.unit.temp_id);
                    }
                }

                // Check for private constructors
                if (other_name_lower == "__init__"
                    || other_name_lower == "new"
                    || other_name_lower == "constructor")
                    && other.unit.visibility == Visibility::Private
                {
                    has_private_constructor = true;
                    if seen.insert(other.unit.temp_id) {
                        participants.push(other.unit.temp_id);
                    }
                }
            }

            let score = (has_instance_method as u8 + has_private_constructor as u8) as f32 / 2.0;
            if score > 0.0 {
                instances.push(PatternInstance {
                    pattern_name: "Singleton".to_string(),
                    primary_unit: unit.unit.temp_id,
                    participating_units: participants,
                    confidence: score,
                });
            }
        }

        instances
    }
}

/// Detects Factory pattern: functions or classes that create and return
/// other object instances.
struct FactoryMatcher;

impl PatternMatcher for FactoryMatcher {
    fn detect(&self, units: &[ResolvedUnit]) -> Vec<PatternInstance> {
        let mut instances = Vec::new();

        for unit in units {
            if unit.unit.unit_type != CodeUnitType::Function
                && unit.unit.unit_type != CodeUnitType::Type
            {
                continue;
            }

            let name_lower = unit.unit.name.to_lowercase();

            // Name-based detection
            if name_lower.contains("factory")
                || name_lower.starts_with("create_")
                || name_lower.starts_with("make_")
                || name_lower.starts_with("build_")
                || name_lower == "new"
            {
                instances.push(PatternInstance {
                    pattern_name: "Factory".to_string(),
                    primary_unit: unit.unit.temp_id,
                    participating_units: vec![unit.unit.temp_id],
                    confidence: if name_lower.contains("factory") {
                        0.9
                    } else {
                        0.5
                    },
                });
            }
        }

        instances
    }
}

/// Detects Repository pattern: data access layer classes.
struct RepositoryMatcher;

impl PatternMatcher for RepositoryMatcher {
    fn detect(&self, units: &[ResolvedUnit]) -> Vec<PatternInstance> {
        let mut instances = Vec::new();
        let (functions_by_owner, functions_by_file) = build_function_indexes(units);

        for unit in units {
            if unit.unit.unit_type != CodeUnitType::Type {
                continue;
            }

            let name_lower = unit.unit.name.to_lowercase();

            if name_lower.contains("repository")
                || name_lower.contains("repo")
                || name_lower.contains("dao")
                || name_lower.contains("store")
            {
                // Look for CRUD methods among likely methods for this type.
                let methods =
                    gather_candidate_methods(unit, &functions_by_owner, &functions_by_file);
                let mut crud_count = 0;
                for other in methods {
                    if !method_belongs_to_type(other, &name_lower) {
                        continue;
                    }

                    let method_lower = other.unit.name.to_lowercase();
                    if method_lower.starts_with("get")
                        || method_lower.starts_with("find")
                        || method_lower.starts_with("create")
                        || method_lower.starts_with("update")
                        || method_lower.starts_with("delete")
                        || method_lower.starts_with("save")
                        || method_lower.starts_with("list")
                    {
                        crud_count += 1;
                    }
                }

                let confidence = if crud_count >= 3 {
                    0.9
                } else if crud_count >= 1 {
                    0.6
                } else {
                    0.4
                };

                instances.push(PatternInstance {
                    pattern_name: "Repository".to_string(),
                    primary_unit: unit.unit.temp_id,
                    participating_units: vec![unit.unit.temp_id],
                    confidence,
                });
            }
        }

        instances
    }
}

/// Detects Decorator pattern by name convention.
struct DecoratorMatcher;

impl PatternMatcher for DecoratorMatcher {
    fn detect(&self, units: &[ResolvedUnit]) -> Vec<PatternInstance> {
        let mut instances = Vec::new();

        for unit in units {
            let name_lower = unit.unit.name.to_lowercase();

            if name_lower.contains("decorator")
                || name_lower.contains("wrapper")
                || name_lower.contains("middleware")
            {
                instances.push(PatternInstance {
                    pattern_name: "Decorator".to_string(),
                    primary_unit: unit.unit.temp_id,
                    participating_units: vec![unit.unit.temp_id],
                    confidence: if name_lower.contains("decorator") {
                        0.8
                    } else {
                        0.5
                    },
                });
            }
        }

        instances
    }
}

fn build_function_indexes<'a>(
    units: &'a [ResolvedUnit],
) -> (
    HashMap<String, Vec<&'a ResolvedUnit>>,
    HashMap<String, Vec<&'a ResolvedUnit>>,
) {
    let mut by_owner: HashMap<String, Vec<&ResolvedUnit>> = HashMap::new();
    let mut by_file: HashMap<String, Vec<&ResolvedUnit>> = HashMap::new();

    for unit in units {
        if unit.unit.unit_type != CodeUnitType::Function {
            continue;
        }

        let file_key = unit.unit.file_path.to_string_lossy().to_string();
        by_file.entry(file_key).or_default().push(unit);

        if let Some(owner) = infer_owner_type_name(&unit.unit.qualified_name, &unit.unit.name) {
            by_owner.entry(owner).or_default().push(unit);
        }
    }

    (by_owner, by_file)
}

fn gather_candidate_methods<'a>(
    type_unit: &ResolvedUnit,
    functions_by_owner: &HashMap<String, Vec<&'a ResolvedUnit>>,
    functions_by_file: &HashMap<String, Vec<&'a ResolvedUnit>>,
) -> Vec<&'a ResolvedUnit> {
    let type_name_lower = type_unit.unit.name.to_lowercase();
    if let Some(methods) = functions_by_owner.get(&type_name_lower) {
        return methods.clone();
    }

    let file_key = type_unit.unit.file_path.to_string_lossy().to_string();
    functions_by_file
        .get(&file_key)
        .cloned()
        .unwrap_or_default()
}

fn method_belongs_to_type(method: &ResolvedUnit, type_name_lower: &str) -> bool {
    if let Some(owner) = infer_owner_type_name(&method.unit.qualified_name, &method.unit.name) {
        if owner == type_name_lower {
            return true;
        }
    }

    method
        .unit
        .qualified_name
        .to_lowercase()
        .contains(type_name_lower)
}

fn infer_owner_type_name(qname: &str, function_name: &str) -> Option<String> {
    let segments: Vec<&str> = qname
        .split(|c| c == '.' || c == ':' || c == '/' || c == '$')
        .filter(|segment| !segment.is_empty())
        .collect();

    if segments.len() < 2 {
        return None;
    }

    let fn_segment_idx = segments.iter().rposition(|segment| {
        let clean = segment.split('(').next().unwrap_or(segment);
        clean == function_name || clean.starts_with(function_name)
    })?;

    if fn_segment_idx == 0 {
        return None;
    }

    Some(segments[fn_segment_idx - 1].to_lowercase())
}
