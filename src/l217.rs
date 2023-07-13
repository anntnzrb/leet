use std::collections::HashSet;
use std::iter::FromIterator;

/// Sets contain unique elements only.
/// (element cannot be present more than once)
///
/// Making a Set from a collection (which may have repeated elements)
/// leads to a new Set with the collection's unique elements.
///
/// If the sizes of both collection and set differ, then the collection had
/// repeated elements.

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    nums.len() != HashSet::<i32>::from_iter(nums).len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v1() {
        assert_eq!(contains_duplicate(vec![1, 2, 3, 1]), true);
        assert_eq!(contains_duplicate(vec![1, 2, 3, 4]), false);
        assert_eq!(contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]), true);
    }
}
