fn main() {
    println!("Advent of Code - Day 2");

    // Read in data from file
    let data: &str = include_str!("../data.txt");

    // Evaluate the games scores
    let (opponent_score, my_score) = evaluate_game(data, false);

    println!(
        "Total scores\n\topponent score: {}\n\tmy score: {}",
        opponent_score, my_score
    );

    // Evaluate the games scores
    let (opponent_score, my_score) = evaluate_game(data, true);

    println!(
        "Total scores alt\n\topponent score: {}\n\tmy score: {}",
        opponent_score, my_score
    );
}

fn evaluate_game(game: &str, alt: bool) -> (u32, u32) {
    // Total scores
    let mut total_0: u32 = 0;
    let mut total_1: u32 = 0;

    // Split on each line and iterate
    for line in game.split("\n") {
        if line.is_empty() {
            continue;
        }

        if alt {
            let (score_0, score_1) = evaluate_round_alt(line);
            total_0 += score_0;
            total_1 += score_1;
        } else {
            let (score_0, score_1) = evaluate_round(line);
            total_0 += score_0;
            total_1 += score_1;
        }
    }

    return (total_0, total_1);
}

fn evaluate_round(round: &str) -> (u32, u32) {
    match round {
        "A X" => return (4, 4),
        "A Y" => return (1, 8),
        "A Z" => return (7, 3),
        "B X" => return (8, 1),
        "B Y" => return (5, 5),
        "B Z" => return (2, 9),
        "C X" => return (3, 7),
        "C Y" => return (9, 2),
        "C Z" => return (6, 6),
        _ => panic!("unhandled round state '{}'", round),
    }
}

fn evaluate_round_alt(round: &str) -> (u32, u32) {
    match round {
        "A X" => return (7, 3),
        "A Y" => return (4, 4),
        "A Z" => return (1, 8),
        "B X" => return (8, 1),
        "B Y" => return (6, 5),
        "B Z" => return (2, 9),
        "C X" => return (9, 2),
        "C Y" => return (6, 6),
        "C Z" => return (3, 7),
        _ => panic!("unhandled round state '{}'", round),
    }
}
