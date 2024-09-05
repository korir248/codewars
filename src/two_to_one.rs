use std::collections::HashSet;

pub fn longest(a1: &str, a2: &str) -> String {
    let mut a1 = a1.to_string();
    a1.push_str(a2);
    println!("Comp: {}", &a1);
    let mut str = a1
        .chars()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    str.sort();

    str.iter().collect()
}
