use regex::Regex;

pub fn validate_pin(pin: &str) -> bool {
    let re = Regex::new(r"(^\d\d\d\d$)|(^\d\d\d\d\d\d$)").unwrap();
    re.is_match(pin)
}

fn better(pin: &str) -> bool {
    pin.chars().all(|c| c.is_digit(10)) && (pin.len() == 4 || pin.len() == 6)
}