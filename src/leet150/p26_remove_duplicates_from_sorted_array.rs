/// 26. Remove Duplicates from Sorted Array
///
/// Remove duplicate values from `nums` in place, keeping one of each.
///
/// `nums` is sorted in non-decreasing order. The unique values must be at the
/// start of `nums`, in their original order. Return how many unique values
/// remain.
#[allow(dead_code)]
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut idx = 0;
    for i in 1..nums.len() {
        if nums[i] != nums[idx] {
            idx += 1;
            nums[idx] = nums[i];
        }
    }
    (idx + 1) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut nums = vec![1, 1, 2];
        assert_eq!(remove_duplicates(&mut nums), 2);
        assert_eq!(nums[..2], [1, 2]);
    }

    #[test]
    fn example_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(remove_duplicates(&mut nums), 5);
        assert_eq!(nums[..5], [0, 1, 2, 3, 4]);
    }
}
