use std::fs;
use std::str::Split;

fn main() {
    let contents = fs::read_to_string("aoc-01-calories/input.txt").expect("Failed to read!");
    let elves: Split<&str> = contents.trim().split("\n\n");

    let mut elf_food_list = vec![];

    for elf in elves {
        let elf_pack: Split<&str> = elf.split("\n");
        let elf_total = elf_pack
            .map(|food| food.parse::<i32>().unwrap())
            .fold(0, |sum, food| sum + food);

        elf_food_list.push(elf_total);
    }

    elf_food_list.sort();
    elf_food_list.reverse();

    let elf_food_total: i32 = elf_food_list.iter().take(3).sum();

    println!("Elf Total:{elf_food_total}");
}
