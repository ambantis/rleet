use std::collections::HashSet;

/// 189. Rotate Array
///
/// Rotate `nums` to the right by `k` steps, in place.
///
/// After the rotation, the last `k` values must be at the start of `nums`, in
/// their original order, followed by the rest.
#[allow(dead_code)]
pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let len = nums.len();
    let k: usize = k as usize % len;
    if k < 2 {
        return
    }
    let inc = | i: &usize| -> usize {
        let mut next = i + k; 
        if next >= len {
            next -= len;
        }
        next
    };
    let mut seen: HashSet<usize> = HashSet::new();
    let mut tmp: i32;
    for first in 0..nums.len() {
        if seen.contains(&first) {
            continue;
        }
        let mut prev: i32 = nums[first];
        let mut i = first;
        while seen.len() < len {
            i = inc(&i);
            tmp = nums[i];
            nums[i] = prev;
            prev = tmp;
            seen.insert(i);
            if i == first {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        rotate(&mut nums, 3);
        assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    fn example_2() {
        let mut nums = vec![-1, -100, 3, 99];
        rotate(&mut nums, 2);
        assert_eq!(nums, vec![3, 99, -1, -100]);
    }
}
