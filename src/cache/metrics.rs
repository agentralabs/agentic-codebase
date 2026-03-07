use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

pub struct CacheMetrics {
    hits: AtomicU64,
    misses: AtomicU64,
    evictions: AtomicU64,
    current_size: AtomicUsize,
}

impl CacheMetrics {
    pub fn new() -> Self {
        Self {
            hits: AtomicU64::new(0),
            misses: AtomicU64::new(0),
            evictions: AtomicU64::new(0),
            current_size: AtomicUsize::new(0),
        }
    }

    pub fn record_hit(&self) {
        self.hits.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_miss(&self) {
        self.misses.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_eviction(&self) {
        self.evictions.fetch_add(1, Ordering::Relaxed);
    }

    pub fn set_size(&self, size: usize) {
        self.current_size.store(size, Ordering::Relaxed);
    }

    pub fn hits(&self) -> u64 {
        self.hits.load(Ordering::Relaxed)
    }

    pub fn misses(&self) -> u64 {
        self.misses.load(Ordering::Relaxed)
    }

    pub fn evictions(&self) -> u64 {
        self.evictions.load(Ordering::Relaxed)
    }

    pub fn current_size(&self) -> usize {
        self.current_size.load(Ordering::Relaxed)
    }

    pub fn hit_rate(&self) -> f64 {
        let t = self.hits() + self.misses();
        if t == 0 {
            0.0
        } else {
            self.hits() as f64 / t as f64
        }
    }
}

impl Default for CacheMetrics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_metrics() {
        let m = CacheMetrics::new();
        assert_eq!(m.hits(), 0);
        assert_eq!(m.misses(), 0);
        assert_eq!(m.evictions(), 0);
        assert_eq!(m.current_size(), 0);
    }

    #[test]
    fn test_hit_rate() {
        let m = CacheMetrics::new();
        m.record_hit();
        m.record_hit();
        m.record_miss();
        assert!((m.hit_rate() - 2.0 / 3.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_hit_rate_no_data() {
        let m = CacheMetrics::new();
        assert_eq!(m.hit_rate(), 0.0);
    }
}
