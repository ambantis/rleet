/// 80. Remove Duplicates from Sorted Array II
///
/// Remove duplicate values from `nums` in place, keeping at most two of each.
///
/// `nums` is sorted in non-decreasing order. The kept values must be at the
/// start of `nums`, in their original order. Return how many values remain.
#[allow(dead_code)]
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut count = 1;
    let mut idx: usize = 0;
    for i in 1..nums.len() {
        if nums[i] == nums[idx] {
            count += 1;
        }
        if nums[i] != nums[idx] {
            idx += 1;
            count = 1;
            nums[idx] = nums[i];
        } else if count <= 2 {
            idx += 1;
            count += 1;
            nums[idx] = nums[i];
        }
    }
    idx += 1;
    idx as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        assert_eq!(remove_duplicates(&mut nums), 5);
        assert_eq!(nums[..5], [1, 1, 2, 2, 3]);
    }

    #[test]
    fn example_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        assert_eq!(remove_duplicates(&mut nums), 7);
        assert_eq!(nums[..7], [0, 0, 1, 1, 2, 3, 3]);
    }
}
