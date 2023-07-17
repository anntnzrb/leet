use std::collections::HashMap;

pub fn top_k_frequent_v1(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    nums.iter().for_each(|n| {
        *map.entry(*n).or_insert(0) += 1;
    });

    let mut xs: Vec<(i32, i32)> = map.into_iter().collect();

    xs.sort_by(|a, b| b.1.cmp(&a.1));

    xs.into_iter()
        .map(|(key, _)| key)
        .take(k as usize)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v1() {
        let mut xs = top_k_frequent_v1(vec![1, 1, 1, 2, 2, 3], 2);
        let mut ys = top_k_frequent_v1(vec![1], 1);
        let mut zs = top_k_frequent_v1(vec![1, 2], 2);

        xs.sort();
        ys.sort();
        zs.sort();

        assert_eq!(xs, vec![1, 2]);
        assert_eq!(ys, vec![1]);
        assert_eq!(zs, vec![1, 2]);
    }
}
