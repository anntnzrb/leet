use std::collections::HashMap;

pub fn is_anagram_v1(s: String, t: String) -> bool {
    // if lenght of both strings differ, then false
    if s.len() != t.len() {
        return false;
    }

    // transform into vector of chars & sort
    let mut s_vec: Vec<char> = s.chars().collect();
    let mut t_vec: Vec<char> = t.chars().collect();

    s_vec.sort();
    t_vec.sort();

    s_vec == t_vec
}

pub fn is_anagram_v2(s: String, t: String) -> bool {
    // if lenght of both strings differ, then false
    if s.len() != t.len() {
        return false;
    }

    // create a map for each string
    // the maps contain the number of char occurrences
    let mut map1: HashMap<char, i32> = HashMap::new();
    let mut map2: HashMap<char, i32> = HashMap::new();

    s.chars().for_each(|c| *map1.entry(c).or_insert(0) += 1);
    t.chars().for_each(|c| *map2.entry(c).or_insert(0) += 1);

    map1 == map2
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fn_sol<F: Fn(String, String) -> bool>(sol: F) {
        assert!(sol("anagram".into(), "nagaram".into()));
        assert!(sol("bake".into(), "beak".into()));
        assert!(sol("meats".into(), "steam".into()));
        assert!(sol("ring".into(), "grin".into()));

        assert!(!sol("rat".into(), "car".into()));
        assert!(!sol("number".into(), "rbmiun".into()));
    }

    #[test]
    fn v1() {
        fn_sol(is_anagram_v1);
    }

    #[test]
    fn v2() {
        fn_sol(is_anagram_v2);
    }
}
