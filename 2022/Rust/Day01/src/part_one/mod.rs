use crate::args;

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy)]
pub struct Elf {
    pub total_calories: u32,
}

pub fn run(a: args::Arguments) {
    let mut elves: Vec<Elf> = Vec::new();
    let mut e = Elf { total_calories: 0 };
    let mut max_elf = 0;

    // @REF - Read a file line by line - https://riptutorial.com/rust/example/4275/read-a-file-line-by-line
    //let file = match File::open(a.input.unwrap_or(PathBuf::from(args::DEFAULT_INPUT))) {
    let file = match File::open(a.get_input()) {
        Err(e) => panic!("Could not open file: {}", e),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors
        if line.is_empty() {
            elves.push(e);

            if e.total_calories > elves.get(max_elf).unwrap().total_calories {
                max_elf = elves.len() - 1;
            }

            e = Elf { total_calories: 0 }
        } else {
            let c = line.parse::<u32>().unwrap();
            e.total_calories += c;
        }
    }

    println!("D01P1 {}", elves.get(max_elf).unwrap().total_calories);
}
