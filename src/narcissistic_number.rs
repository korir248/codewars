pub fn narcissistic(num: u64) -> bool {
    let numbers = num
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u64)
        .collect::<Vec<_>>();

    println!("{:?}", numbers);

    let base = numbers.len() as u32;

    numbers
        .iter()
        .map(|num| num.pow(base) as usize)
        .sum::<usize>()
        == num as usize
}
