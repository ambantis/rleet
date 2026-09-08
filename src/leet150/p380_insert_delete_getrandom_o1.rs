use std::{cell::RefCell, collections::HashMap};
use rand::{RngExt, rngs::ThreadRng};
use rand::rngs::SmallRng;
use rand::{RngExt,SeedableRng};

/// 380. Insert Delete GetRandom O(1)
///
/// Implement a set of `i32` values where every operation runs in average O(1)
/// time:
///
/// - `insert(val)` adds `val` and returns `true`, or returns `false` if `val`
///   was already present.
/// - `remove(val)` removes `val` and returns `true`, or returns `false` if
///   `val` was not present.
/// - `get_random()` returns a value chosen uniformly at random from the current
///   contents. It is only called when the set is non-empty.
///
/// Note that `get_random` takes `&self`, so any RNG state has to live behind a
/// `Cell`/`RefCell`. LeetCode's judge has the `rand` crate; this repo has no
/// dependencies, so the source of entropy has to come out of `std`.
#[allow(dead_code)]
struct RandomizedSet {
    values: Vec<i32>,
    lookup: HashMap<i32, usize>,
    rng: RefCell<SmallRng>
}

#[allow(dead_code)]
impl RandomizedSet {
    fn new() -> Self {
        RandomizedSet {
            values: Vec::new(),
            lookup: HashMap::new(),
            rng: RefCell::new(SmallRng::from_rng(&mut rand::rng())),

        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.lookup.contains_key(&val) {
            false
        } else {
            let idx = self.values.len();
            self.values.push(val);
            self.lookup.insert(val, idx);
            true
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        match self.lookup.remove(&val) {
            None => false,
            Some(idx) => {
                self.values.swap_remove(idx);
                if idx < self.values.len() {
                    self.lookup.insert(self.values[idx], idx);
                }
                true
            }
        }
    }

    fn get_random(&self) -> i32 {
        let idx = self.rng.borrow_mut().random_range(0..self.values.len());
        self.values[idx]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn example_1() {
        let mut set = RandomizedSet::new();
        assert!(set.insert(1));
        assert!(!set.remove(2));
        assert!(set.insert(2));
        assert!([1, 2].contains(&set.get_random()));
        assert!(set.remove(1));
        assert!(!set.insert(2));
        assert_eq!(set.get_random(), 2);
    }

    #[test]
    fn insert_is_idempotent() {
        let mut set = RandomizedSet::new();
        assert!(set.insert(7));
        assert!(!set.insert(7));
        assert_eq!(set.get_random(), 7);
    }

    #[test]
    fn remove_then_reinsert() {
        let mut set = RandomizedSet::new();
        assert!(set.insert(3));
        assert!(set.remove(3));
        assert!(!set.remove(3));
        assert!(set.insert(3));
        assert_eq!(set.get_random(), 3);
    }

    #[test]
    fn handles_negative_and_extreme_values() {
        let mut set = RandomizedSet::new();
        assert!(set.insert(i32::MIN));
        assert!(set.insert(i32::MAX));
        assert!(set.insert(0));
        assert!(!set.insert(i32::MIN));
        assert!(set.remove(0));
        assert!([i32::MIN, i32::MAX].contains(&set.get_random()));
    }

    #[test]
    fn removing_the_middle_keeps_the_rest_reachable() {
        // A swap-remove implementation moves the last element into the hole; if
        // its index is not updated, that element goes missing.
        let mut set = RandomizedSet::new();
        for value in 0..5 {
            assert!(set.insert(value));
        }
        assert!(set.remove(1));
        let mut seen = HashSet::new();
        for _ in 0..1_000 {
            seen.insert(set.get_random());
        }
        assert_eq!(seen, HashSet::from([0, 2, 3, 4]));
    }

    #[test]
    fn get_random_is_roughly_uniform() {
        let mut set = RandomizedSet::new();
        for value in 0..10 {
            assert!(set.insert(value));
        }
        let mut counts = [0_u32; 10];
        for _ in 0..100_000 {
            counts[set.get_random() as usize] += 1;
        }
        // Expected 10,000 each; a generous band still catches a stuck or
        // badly skewed generator.
        assert!(
            counts.iter().all(|&c| (7_000..13_000).contains(&c)),
            "not uniform: {counts:?}"
        );
    }
}
