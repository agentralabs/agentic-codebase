//! High-level concept extraction.
//!
//! Extracts concepts like "authentication", "payments", "user management"
//! by analyzing symbol names, docstrings, and usage patterns.

use crate::types::{AcbResult, CodeUnitType};

use super::resolver::ResolvedUnit;

/// Extracts high-level concepts from code.
pub struct ConceptExtractor {
    /// Concept definitions with keywords.
    concepts: Vec<ConceptDefinition>,
}

/// Definition of a concept to detect.
#[derive(Debug, Clone)]
struct ConceptDefinition {
    /// Concept name.
    name: String,
    /// Keywords that indicate this concept.
    keywords: Vec<String>,
    /// Typical code unit types for this concept.
    typical_types: Vec<CodeUnitType>,
}

/// An extracted concept grouping related code units.
#[derive(Debug, Clone)]
pub struct ExtractedConcept {
    /// Concept name.
    pub name: String,
    /// Code units belonging to this concept.
    pub units: Vec<ConceptUnit>,
    /// Overall confidence.
    pub confidence: f32,
}

/// A code unit's membership in a concept.
#[derive(Debug, Clone)]
pub struct ConceptUnit {
    /// Unit temp_id.
    pub unit_id: u64,
    /// Role in the concept.
    pub role: ConceptRole,
    /// Score (0.0 to 1.0).
    pub score: f32,
}

/// The role a code unit plays in a concept.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConceptRole {
    /// Defines the concept (interface, base class).
    Definition,
    /// Implements the concept.
    Implementation,
    /// Uses the concept.
    Usage,
    /// Tests the concept.
    Test,
}

impl ConceptExtractor {
    /// Create a new concept extractor with built-in concept definitions.
    pub fn new() -> Self {
        let concepts = vec![
            ConceptDefinition {
                name: "Authentication".to_string(),
                keywords: vec![
                    "auth",
                    "login",
                    "logout",
                    "session",
                    "token",
                    "jwt",
                    "oauth",
                    "password",
                    "credential",
                    "authenticate",
                ]
                .into_iter()
                .map(String::from)
                .collect(),
                typical_types: vec![CodeUnitType::Function, CodeUnitType::Type],
            },
            ConceptDefinition {
                name: "Payment".to_string(),
                keywords: vec![
                    "payment",
                    "charge",
                    "refund",
                    "transaction",
                    "stripe",
                    "paypal",
                    "billing",
                    "invoice",
                    "checkout",
                ]
                .into_iter()
                .map(String::from)
                .collect(),
                typical_types: vec![CodeUnitType::Function, CodeUnitType::Type],
            },
            ConceptDefinition {
                name: "UserManagement".to_string(),
                keywords: vec![
                    "user",
                    "account",
                    "profile",
                    "registration",
                    "signup",
                    "settings",
                    "preferences",
                ]
                .into_iter()
                .map(String::from)
                .collect(),
                typical_types: vec![CodeUnitType::Type, CodeUnitType::Function],
            },
            ConceptDefinition {
                name: "Database".to_string(),
                keywords: vec![
                    "database",
                    "db",
                    "query",
                    "sql",
                    "migration",
                    "schema",
                    "repository",
                    "model",
                    "entity",
                    "table",
                    "record",
                ]
                .into_iter()
                .map(String::from)
                .collect(),
                typical_types: vec![CodeUnitType::Type, CodeUnitType::Function],
            },
            ConceptDefinition {
                name: "API".to_string(),
                keywords: vec![
                    "api",
                    "endpoint",
                    "route",
                    "handler",
                    "controller",
                    "request",
                    "response",
                    "middleware",
                    "rest",
                    "graphql",
                ]
                .into_iter()
                .map(String::from)
                .collect(),
                typical_types: vec![CodeUnitType::Function, CodeUnitType::Type],
            },
            ConceptDefinition {
                name: "Logging".to_string(),
                keywords: vec![
                    "log",
                    "logger",
                    "logging",
                    "trace",
                    "debug",
                    "info",
                    "warn",
                    "error",
                    "metric",
                    "telemetry",
                ]
                .into_iter()
                .map(String::from)
                .collect(),
                typical_types: vec![CodeUnitType::Function, CodeUnitType::Type],
            },
            ConceptDefinition {
                name: "Configuration".to_string(),
                keywords: vec![
                    "config",
                    "configuration",
                    "setting",
                    "env",
                    "environment",
                    "option",
                    "preference",
                    "feature_flag",
                ]
                .into_iter()
                .map(String::from)
                .collect(),
                typical_types: vec![CodeUnitType::Type, CodeUnitType::Function],
            },
            ConceptDefinition {
                name: "Testing".to_string(),
                keywords: vec![
                    "test",
                    "mock",
                    "stub",
                    "fixture",
                    "assert",
                    "expect",
                    "spec",
                    "bench",
                    "benchmark",
                ]
                .into_iter()
                .map(String::from)
                .collect(),
                typical_types: vec![CodeUnitType::Test, CodeUnitType::Function],
            },
            ConceptDefinition {
                name: "ErrorHandling".to_string(),
                keywords: vec![
                    "error",
                    "exception",
                    "fault",
                    "retry",
                    "fallback",
                    "recovery",
                    "panic",
                    "catch",
                    "throw",
                ]
                .into_iter()
                .map(String::from)
                .collect(),
                typical_types: vec![CodeUnitType::Type, CodeUnitType::Function],
            },
            ConceptDefinition {
                name: "Caching".to_string(),
                keywords: vec![
                    "cache",
                    "memoize",
                    "lru",
                    "ttl",
                    "invalidate",
                    "redis",
                    "memcached",
                ]
                .into_iter()
                .map(String::from)
                .collect(),
                typical_types: vec![CodeUnitType::Function, CodeUnitType::Type],
            },
        ];

        Self { concepts }
    }

    /// Extract concepts from the resolved units.
    pub fn extract(&self, units: &[ResolvedUnit]) -> AcbResult<Vec<ExtractedConcept>> {
        let mut concept_units_by_idx: Vec<Vec<ConceptUnit>> =
            (0..self.concepts.len()).map(|_| Vec::new()).collect();

        // Precompute normalized text once per unit to avoid repeated lowercasing
        // for each concept definition.
        for unit in units {
            let name_lower = unit.unit.name.to_lowercase();
            let qname_lower = unit.unit.qualified_name.to_lowercase();
            let doc_lower = unit.unit.doc.as_ref().map(|d| d.to_lowercase());
            let role = self.determine_role(unit);

            for (idx, concept_def) in self.concepts.iter().enumerate() {
                let score = self.score_unit_normalized(
                    &name_lower,
                    &qname_lower,
                    doc_lower.as_deref(),
                    unit.unit.unit_type,
                    concept_def,
                );

                if score > 0.3 {
                    concept_units_by_idx[idx].push(ConceptUnit {
                        unit_id: unit.unit.temp_id,
                        role,
                        score,
                    });
                }
            }
        }

        let mut extracted = Vec::new();
        for (idx, concept_def) in self.concepts.iter().enumerate() {
            let concept_units = std::mem::take(&mut concept_units_by_idx[idx]);
            if concept_units.is_empty() {
                continue;
            }

            let avg_score =
                concept_units.iter().map(|u| u.score).sum::<f32>() / concept_units.len() as f32;

            extracted.push(ExtractedConcept {
                name: concept_def.name.clone(),
                units: concept_units,
                confidence: avg_score,
            });
        }

        Ok(extracted)
    }

    fn score_unit_normalized(
        &self,
        name_lower: &str,
        qname_lower: &str,
        doc_lower: Option<&str>,
        unit_type: CodeUnitType,
        concept: &ConceptDefinition,
    ) -> f32 {
        let mut score = 0.0f32;

        // Keyword matching in name
        for keyword in &concept.keywords {
            if name_lower.contains(keyword.as_str()) {
                score += 0.4;
            } else if qname_lower.contains(keyword.as_str()) {
                score += 0.2;
            }
        }

        // Doc matching
        if let Some(doc_lower) = doc_lower {
            for keyword in &concept.keywords {
                if doc_lower.contains(keyword.as_str()) {
                    score += 0.15;
                }
            }
        }

        // Type bonus
        if concept.typical_types.contains(&unit_type) {
            score += 0.1;
        }

        score.min(1.0)
    }

    fn determine_role(&self, unit: &ResolvedUnit) -> ConceptRole {
        match unit.unit.unit_type {
            CodeUnitType::Type | CodeUnitType::Trait => ConceptRole::Definition,
            CodeUnitType::Test => ConceptRole::Test,
            CodeUnitType::Function | CodeUnitType::Impl => ConceptRole::Implementation,
            _ => ConceptRole::Usage,
        }
    }
}

impl Default for ConceptExtractor {
    fn default() -> Self {
        Self::new()
    }
}
