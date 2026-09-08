use std::cmp::Reverse;

/// 274. H-Index
///
/// You are given an array `citations` where `citations[i]` is the number of
/// citations the researcher's `i`th paper received. Return the researcher's
/// h-index: the largest `h` such that at least `h` of their papers have `h` or
/// more citations each. The array is in no particular order.
#[allow(dead_code)]
pub fn h_index(citations: Vec<i32>) -> i32 {
    let mut citations = citations;
    citations.sort_unstable_by_key(|&x| Reverse(x));
    for (i, &value) in citations.iter().enumerate() {
        if value < (i + 1) as i32 {
            return i as i32
        }
    }
    citations.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(h_index(vec![3, 0, 6, 1, 5]), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(h_index(vec![1, 3, 1]), 1);
    }

    #[test]
    fn single_uncited_paper() {
        assert_eq!(h_index(vec![0]), 0);
    }

    #[test]
    fn single_heavily_cited_paper() {
        // One paper can never push the h-index above 1.
        assert_eq!(h_index(vec![100]), 1);
    }

    #[test]
    fn nothing_cited() {
        assert_eq!(h_index(vec![0, 0, 0]), 0);
    }

    #[test]
    fn capped_by_paper_count() {
        assert_eq!(h_index(vec![10, 10, 10]), 3);
    }

    #[test]
    fn all_equal_to_the_count() {
        assert_eq!(h_index(vec![2, 2]), 2);
    }

    #[test]
    fn one_short_of_the_next_h() {
        // Three papers with >= 2 citations, but only two with >= 3.
        assert_eq!(h_index(vec![5, 3, 2, 1]), 2);
    }
}
