/// 55. Jump Game
///
/// You are given an array `nums` of non-negative integers. You start at index 0,
/// and `nums[i]` is the maximum number of steps you may jump forward from index
/// `i`. Return `true` if you can reach the last index, `false` otherwise.
#[allow(dead_code)]
pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut furthest: usize = 0;
    for (i, &value) in nums.iter().enumerate() {
        if i > furthest {
            return false
        }
        furthest = furthest.max(i + value as usize)
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert!(can_jump(vec![2, 3, 1, 1, 4]));
    }

    #[test]
    fn example_2() {
        assert!(!can_jump(vec![3, 2, 1, 0, 4]));
    }

    #[test]
    fn single_element() {
        assert!(can_jump(vec![0]));
    }

    #[test]
    fn stuck_at_start() {
        assert!(!can_jump(vec![0, 1]));
    }

    #[test]
    fn zero_on_last_index_is_fine() {
        assert!(can_jump(vec![1, 0]));
    }

    #[test]
    fn first_jump_clears_everything() {
        assert!(can_jump(vec![5, 0, 0, 0, 0, 0]));
    }

    #[test]
    fn just_barely_reaches() {
        // Every hop is forced: 0 -> 2 -> 4, with dead zeros in between.
        assert!(can_jump(vec![2, 0, 2, 0, 1]));
    }

    #[test]
    fn falls_one_index_short() {
        assert!(!can_jump(vec![2, 0, 1, 0, 1]));
    }
}
