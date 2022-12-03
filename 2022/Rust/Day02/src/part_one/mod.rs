use crate::args;

use std::fs;

pub fn run(a: args::Arguments) {
    println!("Part 1");
    let result = run_string(fs::read_to_string(a.get_input()).unwrap());
    println!("Result is {}", result);
}

fn run_string(s: String) -> u32 {
    s.lines()
        .map(|game| match game.trim() {
            // There might be a better way to do this...
            "A X" => 1 + 3,
            "A Y" => 2 + 6,
            "A Z" => 3 + 0,
            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 1 + 6,
            "C Y" => 2 + 0,
            "C Z" => 3 + 3,
            _ => 0,
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "A Y
                     B X
                     C Z";

        println!("Testing input {}", input);
        assert_eq!(run_string(input.to_string()), 15);
    }
}
