use std::{fs, ops::RangeInclusive};

fn main() {
    let file_path = format!("{}/input.txt", env!("CARGO_PKG_NAME"));
    let input = fs::read_to_string(file_path).expect("Error!");

    let elves = input
        .split("\n")
        .flat_map(|e| e.trim().split_once(","))
        .map(|(l, r)| {
            (
                l.split_once("-")
                    .map(|(min, max)| (min.parse::<i32>().unwrap()..=max.parse::<i32>().unwrap()))
                    .unwrap(),
                r.split_once("-")
                    .map(|(min, max)| (min.parse::<i32>().unwrap()..=max.parse::<i32>().unwrap()))
                    .unwrap(),
            )
        })
        .into_iter()
        .collect();

    one(&elves);
    two(&elves);

    //println!("{:?}", elves);
}

fn one(elves: &Vec<(RangeInclusive<i32>, RangeInclusive<i32>)>) {
    let total: Vec<(&RangeInclusive<i32>, &RangeInclusive<i32>)> = elves
        .into_iter()
        .filter_map(|(l, r)| {
            if (l.start() <= r.start() && r.end() <= l.end())
                || (r.start() <= l.start() && l.end() <= r.end())
            {
                Some((l, r))
            } else {
                None
            }
        })
        .collect();

    println!("{:?}", total.len());
}

fn two(elves: &Vec<(RangeInclusive<i32>, RangeInclusive<i32>)>) {
    let total: Vec<(&RangeInclusive<i32>, &RangeInclusive<i32>)> = elves
        .into_iter()
        .filter_map(|(l, r)| {
            if (l.start() <= r.end()) && (r.start() <= l.end()) {
                Some((l, r))
            } else {
                None
            }
        })
        .collect();

    println!("{:?}", total.len());
}
