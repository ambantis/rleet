/// 45. Jump Game II
///
/// You are given an array `nums` of non-negative integers. You start at index 0,
/// and `nums[i]` is the maximum number of steps you may jump forward from index
/// `i`. The input guarantees the last index is reachable. Return the minimum
/// number of jumps needed to get there.
#[allow(dead_code)]
pub fn jump(nums: Vec<i32>) -> i32 {
    let mut count: i32 = 0;
    let mut i = nums.len() - 1;
    while i > 0 {
        let mut next_i = i;
        for (j, &value) in nums.iter().enumerate().take(i) {
            if value as usize >= i - j {
                next_i = j;
                break;
            }
        }
        if next_i < i {
            i = next_i;
            count += 1;
        } else {
            return -1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(jump(vec![2, 3, 1, 1, 4]), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(jump(vec![2, 3, 0, 1, 4]), 2);
    }

    #[test]
    fn already_at_the_end() {
        assert_eq!(jump(vec![0]), 0);
    }

    #[test]
    fn one_jump_clears_everything() {
        assert_eq!(jump(vec![5, 0, 0, 0, 0, 0]), 1);
    }

    #[test]
    fn every_step_is_forced() {
        assert_eq!(jump(vec![1, 1, 1, 1]), 3);
    }

    #[test]
    fn greedy_must_look_past_the_biggest_first_hop() {
        // Jumping the full 3 from index 0 lands on a 0; the answer needs the
        // shorter hop to index 1, which opens up the rest.
        assert_eq!(jump(vec![3, 4, 0, 0, 4, 0, 0, 1]), 3);
    }
}
