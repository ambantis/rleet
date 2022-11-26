use std::collections::HashMap;

// Given an array of integers nums and an integer target, return indices of the
// two numbers such that they add up to target. You may assume that each input
// would have exactly one solution, and you may not use the same element twice.

// You can return the answer in any order.
#[allow(dead_code)]
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut mapping: HashMap<i32, usize> = HashMap::new();
    for (idx, ptr_value) in nums.iter().enumerate() {
        mapping.insert(ptr_value.clone(), idx.clone());
    }
    for (idx1, x) in nums.iter().enumerate() {
        let y: i32 = target - x;
        match mapping.get(&y) {
            Some(idx2) if idx1 != *idx2 => {
                let mut result = Vec::new();
                result.push(idx1.clone().try_into().expect(""));
                result.push(idx2.clone().try_into().expect(""));
                return result;
            }
            _ => {}
        }
    }
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let target = 9;
        let nums = vec![2, 7, 11, 15];
        let expected = vec![0, 1];
        let mut actual = two_sum(nums, target);
        actual.sort();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_two() {
        let target = 6;
        let nums = vec![3, 2, 4];
        let expected = vec![1, 2];
        let mut actual = two_sum(nums, target);
        actual.sort();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_three() {
        let target = 6;
        let nums = vec![3, 3];
        let expected = vec![0, 1];
        let mut actual = two_sum(nums, target);
        actual.sort();
        assert_eq!(expected, actual);
    }
}
