use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub struct CacheInvalidator<K> {
    dependents: HashMap<K, HashSet<K>>,
}

impl<K: Eq + Hash + Clone> CacheInvalidator<K> {
    pub fn new() -> Self {
        Self {
            dependents: HashMap::new(),
        }
    }

    pub fn add_dependency(&mut self, dep: K, dependent: K) {
        self.dependents.entry(dep).or_default().insert(dependent);
    }

    pub fn cascade(&self, key: &K) -> Vec<K> {
        let mut result = Vec::new();
        let mut visited = HashSet::new();
        let mut stack = vec![key.clone()];
        while let Some(current) = stack.pop() {
            if !visited.insert(current.clone()) {
                continue;
            }
            result.push(current.clone());
            if let Some(deps) = self.dependents.get(&current) {
                for dep in deps {
                    if !visited.contains(dep) {
                        stack.push(dep.clone());
                    }
                }
            }
        }
        result
    }

    pub fn clear(&mut self) {
        self.dependents.clear();
    }
}

impl<K: Eq + Hash + Clone> Default for CacheInvalidator<K> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_deps() {
        let inv: CacheInvalidator<String> = CacheInvalidator::new();
        let result = inv.cascade(&"a".to_string());
        assert_eq!(result, vec!["a".to_string()]);
    }

    #[test]
    fn test_simple_cascade() {
        let mut inv = CacheInvalidator::new();
        inv.add_dependency("a", "b");
        inv.add_dependency("a", "c");
        let result = inv.cascade(&"a");
        assert!(result.contains(&"a"));
        assert!(result.contains(&"b"));
        assert!(result.contains(&"c"));
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_transitive_cascade() {
        let mut inv = CacheInvalidator::new();
        inv.add_dependency("a", "b");
        inv.add_dependency("b", "c");
        let result = inv.cascade(&"a");
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_cycle_handling() {
        let mut inv = CacheInvalidator::new();
        inv.add_dependency("a", "b");
        inv.add_dependency("b", "a");
        let result = inv.cascade(&"a");
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_clear() {
        let mut inv = CacheInvalidator::new();
        inv.add_dependency("a", "b");
        inv.clear();
        let result = inv.cascade(&"a");
        assert_eq!(result.len(), 1);
    }
}
