/// 27. Remove Element
///
/// Remove every value equal to `val` from `nums` in place.
///
/// The remaining values must be at the start of `nums`. Their order does not
/// matter. Return how many values remain.
#[allow(dead_code)]
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut idx: i32 = 0;
    for i in 0..nums.len() {
        if nums[i] != val {
            nums[idx as usize] = nums[i];
            idx += 1;
        }
    }
    return idx;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut nums = vec![3, 2, 2, 3];
        assert_eq!(remove_element(&mut nums, 3), 2);
        let mut got = nums[..2].to_vec();
        got.sort();
        assert_eq!(got, vec![2, 2]);
    }

    #[test]
    fn example_2() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        assert_eq!(remove_element(&mut nums, 2), 5);
        let mut got = nums[..5].to_vec();
        got.sort();
        assert_eq!(got, vec![0, 0, 1, 3, 4]);
    }
}
