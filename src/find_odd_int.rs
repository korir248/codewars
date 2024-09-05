use std::collections::{HashMap, HashSet};

pub fn find_odd(arr: &[i32]) -> i32 {
    arr.iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .map(|item| (*item, 0))
        .collect::<HashMap<_, _>>()
        .iter_mut()
        .map(|(k, v)| {
            apply_weight(k, v, arr);
            (*k, *v)
        })
        .filter(|(_, v)| v % 2 != 0)
        .last()
        .unwrap()
        .0
}

fn apply_weight(key: &i32, val: &mut i32, arr: &[i32]) {
    arr.iter().for_each(|item| {
        if item == key {
            *val += 1;
        }
    });
}
