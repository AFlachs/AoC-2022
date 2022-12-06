use std::fs;

pub(crate) fn day2(file_path: &str) -> i32 {
    let content = fs::read_to_string(file_path)
        .expect("Should have been able to read file");

    let mut total_score = 0;
    for line in content.lines() {
        let values = line.trim().split(" ").collect::<Vec<&str>>();
        total_score += compute_outcome(values[0], values[1]);
    }
    total_score
}

fn compute_outcome(opponent: &str, result: &str) -> i32 {
    let (opponent, result) = encode(opponent, result);

    let player_move = (opponent + result).rem_euclid(3);
    player_move +1 + (result+1)*3
}

fn encode(opponent: &str, player: &str) -> (i32, i32) {
    let opponent = match opponent {
        "A" => 0,
        "B" => 1,
        "C" => 2,
        _ => panic!("Not recognized")
    };

    let player = match player {
        "X" => -1, // Loose required
        "Y" => 0, // Draw required
        "Z" => 1, // Win required
        _ => panic!("Not recognized")
    };
    (opponent, player)
}