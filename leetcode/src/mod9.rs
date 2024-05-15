pub fn points(games: &[String]) -> u32 {
    let mut score: u32 = 0;
    for pair in games {
        let splitted: Vec<&str> = pair.split(":").collect();
        if splitted[0] > splitted[1] {
            score += 3
        } else if splitted[0] == splitted[1] {
            score += 1
        }
    }

    return score
}