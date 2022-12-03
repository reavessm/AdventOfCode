use crate::args;

use std::fs;

pub fn run(a: args::Arguments) {
    let result = run_string(fs::read_to_string(a.get_input()).unwrap());
    println!("D02P2 {}", result);
}

fn run_string(s: String) -> u32 {
    s.lines()
        .map(|game| match game.trim() {
            // There might be a better way to do this...
            "A X" => 3 + 0,
            "A Y" => 1 + 3,
            "A Z" => 2 + 6,
            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 2 + 0,
            "C Y" => 3 + 3,
            "C Z" => 1 + 6,
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
        assert_eq!(run_string(input.to_string()), 12);
    }
}
