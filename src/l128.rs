use std::cmp;
use std::collections::HashSet;

pub fn longest_consecutive_v1(nums: Vec<i32>) -> i32 {
    let set: HashSet<_> = nums.iter().collect();
    let mut longest = 0;

    for n in nums.iter() {
        if !set.contains(&(n - 1)) {
            let mut lenght = 0;

            while set.contains(&(n + lenght)) {
                lenght += 1;
            }

            longest = cmp::max(longest, lenght);
        }
    }

    longest
}

pub fn longest_consecutive_v2(nums: Vec<i32>) -> i32 {
    let set: HashSet<_> = nums.iter().collect();

    nums.iter()
        .filter(|&&n| !set.contains(&(n - 1)))
        .map(|&n| (n..).take_while(|n| set.contains(n)).count() as i32)
        .max()
        .unwrap_or(0) as _
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v1() {
        assert_eq!(longest_consecutive_v1(vec![100, 4, 200, 1, 3, 2]), 4);
        assert_eq!(
            longest_consecutive_v1(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]),
            9
        );
    }

    #[test]
    fn v2() {
        assert_eq!(longest_consecutive_v2(vec![100, 4, 200, 1, 3, 2]), 4);
        assert_eq!(
            longest_consecutive_v2(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]),
            9
        );
    }
}
