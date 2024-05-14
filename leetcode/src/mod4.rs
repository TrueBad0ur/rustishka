pub fn abbrev_name(name: &str) -> String {
    name[..1].to_owned().to_string().to_uppercase() + "." + &name.split(" ").nth(1).unwrap()[..1].to_uppercase()
}