pub fn alphabet_position(text: &str) -> String {
    let mut res = String::new();

    text.to_lowercase().chars().for_each(|ch| {
        ('a'..='z').enumerate().for_each(|(i, chr)| {
            if chr == ch {
                res.push_str(&format!("{}", i + 1));

                res.push_str(" ")
            }
        })
    });

    res.trim().to_owned()
}
