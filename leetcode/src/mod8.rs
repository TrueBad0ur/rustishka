pub fn are_you_playing_banjo(name: &str) -> String {
    return match name[..1].to_ascii_uppercase() == *"R" {
        true => name.to_string() + " plays banjo" ,
        false => name.to_string() + " does not play banjo",
    }
}

fn right(name: &str) -> String {
    match &name[0..1] {
        "R" | "r" => format!("{} plays banjo", name),
        _ => format!("{} does not play banjo", name)
    }
}