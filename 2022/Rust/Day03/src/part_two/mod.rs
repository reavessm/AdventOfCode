use crate::args;
use itertools::Itertools;

pub fn run(a: args::Arguments) {
    println!(
        "Part 2: {}",
        run_string(std::fs::read_to_string(a.get_input()).unwrap())
    );
}

fn run_string(s: String) -> usize {
    s.lines()
        // Remove whitespace
        .map(|x| x.trim())
        // Take three lines at a time
        .chunks(3)
        // Create iterator from iteratable
        .into_iter()
        // Wrap triplet into tuple
        .map(|mut triplet| {
            (
                triplet.nth(0).unwrap(),
                triplet.nth(0).unwrap(),
                triplet.nth(0).unwrap(),
            )
        })
        // Find common char
        .map(|(first, second, third)| {
            first
                .chars()
                .filter(|c| second.contains(*c) && third.contains(*c))
                .nth(0)
                .unwrap()
        })
        // Assign value
        .map(|c| match c.is_lowercase() {
            true => ((c as u32) - ('a' as u32) + 1) as usize,
            false => ((c as u32) - ('A' as u32) + 27) as usize,
        })
        // Sum
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
                     jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
                     PmmdzqPrVvPwwTWBwg
                     wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
                     ttgJtRGJQctTZtZT
                     CrZsJsPPZsGzwwsLwLmpwMDw";

        println!("Testing input {}", input);
        assert_eq!(run_string(input.to_string()), 70);
    }
}
