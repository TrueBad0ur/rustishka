pub fn square_digits(num: u64) -> u64 {
    let mut result: String = "".to_owned();

    let converted: Vec<char> = num.to_string().chars().collect();
    for digit in converted {
        let converted_digit = digit.to_digit(10).expect("error");
        result.push_str((converted_digit * converted_digit).to_string().as_str());
    }
    result.parse().unwrap()
}

fn right(num: u64) -> u64 {
    num
        .to_string()
        .chars()
        .map(|i| i.to_digit(10).expect("char isnt digit").pow(2).to_string())
        .collect::<String>()
        .parse()
        .expect("result isnt u64 parsable")
}