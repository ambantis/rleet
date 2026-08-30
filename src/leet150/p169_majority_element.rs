/// 169. Majority Element
///
/// Given an array nums of size n, return the majority element.
///
/// The majority element is the element that appears more than n / 2 times.
/// You may assume that the majority element always exists in the array.
#[allow(dead_code)]
pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut nums: Vec<i32> = nums;
    nums.sort();
    nums[nums.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(majority_element(vec![3, 2, 3]), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }
}
