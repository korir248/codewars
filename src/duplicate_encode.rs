use std::collections::{HashMap, HashSet};

pub fn duplicate_encode(word: &str) -> String {
    let word = word.to_lowercase();
    let mut res = String::new();

    let words = word.chars().collect::<Vec<_>>();
    let word_map = word
        .chars()
        .collect::<HashSet<_>>()
        .iter()
        .map(|chr| (chr, 0))
        .collect::<HashMap<_, _>>()
        .iter_mut()
        .map(|(k, v)| {
            apply_weight(k, v, &words);
            (**k, *v)
        })
        .collect::<HashMap<_, _>>();

    word.chars().for_each(|chr| {
        if word_map.get_key_value(&chr).unwrap().1 == &1 {
            res.push('(');
        } else {
            res.push(')');
        }
    });

    res
}

fn apply_weight(key: &char, val: &mut i32, arr: &[char]) {
    arr.iter().for_each(|item| {
        if item == key {
            *val += 1;
        }
    });
}
