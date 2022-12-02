use std::collections::HashMap;
use std::fs;

fn main() {
    let file_path = format!("{}/input.txt", env!("CARGO_PKG_NAME"));
    let contents = fs::read_to_string(file_path).expect("Failed to read!");

    let rounds: Vec<Vec<&str>> = contents
        .trim()
        .split("\n")
        .map(|x| x.trim().split(" ").collect::<Vec<&str>>())
        .collect();

    let mut total_points = 0;
    for round in rounds {
        total_points += evaluate_round(round[0], round[1]);
    }

    println!("{total_points}");
}

fn evaluate_round(elf: &str, player: &str) -> i32 {
    let points: HashMap<&str, HashMap<&str, i32>> = HashMap::from([
        (
            "Z",
            HashMap::from([("A", 2 + 6), ("B", 3 + 6), ("C", 1 + 6)]),
        ),
        (
            "Y",
            HashMap::from([("A", 1 + 3), ("B", 2 + 3), ("C", 3 + 3)]),
        ),
        (
            "X",
            HashMap::from([("A", 3 + 0), ("B", 1 + 0), ("C", 2 + 0)]),
        ),
    ]);

    return points[player][elf];
    /*
    let wins_points = HashMap::from([("A", 2), ("B", 3), ("C", 1)]);

    let draws_points = HashMap::from([("A", 1), ("B", 2), ("C", 3)]);

    let loss_points = HashMap::from([("A", 3), ("B", 1), ("C", 2)]);

    if player == "Z" {
        return wins_points[elf] + 6;
    } else if player == "Y" {
        return draws_points[elf] + 3;
    } else {
        return loss_points[elf];
    } */
}
