/// 88. Merge Sorted Array
///
/// Merge `nums2` into `nums1` as one sorted array.
///
/// `nums1` has space for `m + n` elements. The first `m` elements are sorted.
/// The last `n` elements are `0` and must be ignored. `nums2` has `n` sorted
/// elements. The result must be stored in `nums1`.
#[allow(dead_code)]
pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut m = m - 1;
    let mut n = n - 1;
    for i in (0..nums1.len()).rev() {
        if m >= 0 && n >= 0 {
            if nums1[m as usize] >= nums2[n as usize] {
                nums1[i] = nums1[m as usize];
                m -= 1;
            } else {
                nums1[i] = nums2[n as usize];
                n -= 1;
            }
        } else if m >= 0 {
            nums1[i] = nums1[m as usize];
            m -= 1;
        } else {
            nums1[i] = nums2[n as usize];
            n -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn example_2() {
        let mut nums1 = vec![1];
        let mut nums2 = vec![];
        merge(&mut nums1, 1, &mut nums2, 0);
        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn example_3() {
        let mut nums1 = vec![0];
        let mut nums2 = vec![1];
        merge(&mut nums1, 0, &mut nums2, 1);
        assert_eq!(nums1, vec![1]);
    }

    /// Merges `a` and `b` through `merge`, building the arguments the way the
    /// problem specifies, and compares against a sort of everything.
    fn check(a: &[i32], b: &[i32]) {
        let mut expected: Vec<i32> = a.iter().chain(b).copied().collect();
        expected.sort();

        let mut nums1 = a.to_vec();
        nums1.resize(a.len() + b.len(), 0);
        let mut nums2 = b.to_vec();
        merge(&mut nums1, a.len() as i32, &mut nums2, b.len() as i32);

        assert_eq!(nums1, expected, "merging {a:?} into {b:?}");
    }

    #[test]
    fn both_empty() {
        check(&[], &[]);
    }

    #[test]
    fn one_side_empty() {
        check(&[1, 2, 3], &[]);
        check(&[], &[1, 2, 3]);
    }

    #[test]
    fn disjoint_ranges() {
        check(&[1, 2, 3], &[4, 5, 6]);
        check(&[4, 5, 6], &[1, 2, 3]);
    }

    #[test]
    fn fully_interleaved() {
        check(&[1, 3, 5, 7], &[2, 4, 6, 8]);
    }

    #[test]
    fn duplicates_across_both() {
        check(&[2, 2, 2], &[2, 2, 2]);
        check(&[1, 1, 5], &[1, 5, 5]);
    }

    #[test]
    fn extreme_values() {
        check(&[i32::MIN, -1], &[0, i32::MAX]);
    }

    /// Deterministic linear congruential generator, so the randomized test
    /// stays dependency-free and reproducible.
    struct Lcg(u64);

    impl Lcg {
        fn below(&mut self, n: u64) -> u64 {
            self.0 = self
                .0
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (self.0 >> 33) % n
        }
    }

    /// Small values and lengths on purpose: they force ties and exhaustion of
    /// either side, which is where the back-to-front merge tends to go wrong.
    #[test]
    fn agrees_with_sort_on_random_inputs() {
        let mut rng = Lcg(0x5EED);
        for _ in 0..20_000 {
            let len_a = rng.below(8);
            let mut a: Vec<i32> = (0..len_a).map(|_| rng.below(6) as i32).collect();
            let len_b = rng.below(8);
            let mut b: Vec<i32> = (0..len_b).map(|_| rng.below(6) as i32).collect();
            a.sort();
            b.sort();
            check(&a, &b);
        }
    }
}
