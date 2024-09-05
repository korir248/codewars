mod preloaded;
use preloaded::MORSE_CODE;
// MORSE_CODE is `HashMap<String, String>`. e.g. ".-" -> "A".

fn decode_morse(encoded: &str) -> String {
    let mut decode = String::new();

 encoded.trim().split("   ").map(|word|{
    word.split_whitespace().map(|letter| decode.push_str(&MORSE_CODE.get(letter).unwrap())).collect::<Vec<_>>();

        decode.push_str(" ");
}).collect::<Vec<_>>();


    decode.trim().to_owned()
}