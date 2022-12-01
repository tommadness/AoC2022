use std::env;
use std::fs;
use std::str::Split;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("Reading file: {file_path}");

    let contents = fs::read_to_string(file_path).expect("Failed to read!");
    let elves: Split<&str> = contents.split("\n\n");

    let mut elf_food_list = vec![];

    for elf in elves {
        let elf_pack: Split<&str> = elf.split("\n");
        let mut elf_total = 0;
        for food in elf_pack {
            println!("Food: {food}");
            if !food.is_empty(){elf_total += food.parse::<i32>().unwrap()};
        }
        elf_food_list.push(elf_total);
    }

    elf_food_list.sort();
    elf_food_list.reverse();

    let elf_food_total = elf_food_list[0]+elf_food_list[1]+elf_food_list[2];

    println!("Elf Total:{elf_food_total}");
}
