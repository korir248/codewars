#![allow(unused)]
use duplicate_encode::duplicate_encode;
use find_odd_int::find_odd;
use narcissistic_number::narcissistic;
use replace_with_alphabet_position::alphabet_position;
use two_to_one::longest;

mod duplicate_encode;
mod find_odd_int;
mod narcissistic_number;
mod replace_with_alphabet_position;
mod two_to_one;

fn main() {
    let res = alphabet_position("hello`");
    println!("Res: {}", res)
}
