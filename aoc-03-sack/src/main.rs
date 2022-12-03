use array_tool::vec::Intersect;
use std::fs;

fn main() {
    let file_path = format!("{}/input.txt", env!("CARGO_PKG_NAME"));
    let input = fs::read_to_string(file_path).expect("Error!");

    let sacks: Vec<&str> = input.trim().split("\n").collect();

    //one(&sacks);
    two(sacks);
}

fn one(sacks: Vec<&str>) {
    let mut intersect = Vec::new();
    for sack in sacks {
        let split_sack = sack.split_at(sack.len() / 2);
        let sack_left: Vec<char> = split_sack.0.chars().collect();
        let sack_right: Vec<char> = split_sack.1.chars().collect();

        let sack_intersect = sack_left.intersect(sack_right);

        intersect.push(sack_intersect[0]);

        //print!("{:?}", sack_intersect);
    }
    let total = priority(intersect);
    println! {"Solution 1: {total}"};
}

fn two(sacks: Vec<&str>) {
    let mut intersect = Vec::new();
    let elf_one: Vec<&&str> = sacks.iter().step_by(3).collect();
    let elf_two: Vec<&&str> = sacks.iter().skip(1).step_by(3).collect();
    let elf_three: Vec<&&str> = sacks.iter().skip(2).step_by(3).collect();

    for i in 0..elf_one.len() {
        intersect.push(
            elf_one[i].chars().collect::<Vec<char>>().intersect(
                elf_two[i]
                    .chars()
                    .collect::<Vec<char>>()
                    .intersect(elf_three[i].chars().collect()),
            )[0],
        )
    }
    println!("{:?}", intersect);
    let total = priority(intersect);
    println!("{}", total);
}

fn priority(intersect: Vec<char>) -> i32 {
    let mut total: i32 = 0;
    let mut dst = [0; 2];
    for letter in intersect {
        if letter.is_lowercase() {
            total += i32::from_le_bytes([
                letter.encode_utf8(&mut dst).as_bytes()[0] - 64 - 32,
                0,
                0,
                0,
            ]);
        } else {
            total += i32::from_le_bytes([
                letter.encode_utf8(&mut dst).as_bytes()[0] - 64 + 26,
                0,
                0,
                0,
            ]);
        }
    }
    return total;
}
