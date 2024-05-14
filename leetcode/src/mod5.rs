pub fn rps(p1: &str, p2: &str) -> &'static str  {
    return match (p1, p2) {
        ("scissors", "paper") => { "Player 1 won!" },
        ("scissors", "rock") => { "Player 2 won!" },
        ("scissors", "scissors") => { "Draw!" },

        ("paper", "scissors") => { "Player 2 won!" },
        ("paper", "rock") => { "Player 1 won!" },
        ("paper", "paper") => { "Draw!" },

        ("rock", "scissors") => { "Player 1 won!" },
        ("rock", "paper") => { "Player 2 won!" },
        ("rock", "rock") => { "Draw!" },

        _ => { "Error!" },
    }
}