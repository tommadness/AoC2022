use std::collections::HashMap;
use std::fs;

fn main() {
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

    let file_path = format!("{}/input.txt", env!("CARGO_PKG_NAME"));
    let total_points: i32 = fs::read_to_string(file_path)
        .expect("Failure!")
        .trim()
        .split("\n")
        .flat_map(|x| x.trim().split_once(' '))
        .map(|(e, p)| points[p][e])
        .sum();

    println!("{total_points}");
}
