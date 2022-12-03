use crate::args;

pub fn run(a: args::Arguments) {
    println!(
        "Part 1: {}",
        run_string(std::fs::read_to_string(a.get_input()).unwrap())
    );
}

fn run_string(s: String) -> usize {
    s.lines()
        // Remove whitespace
        .map(|x| x.trim())
        // Get count and word
        .map(|x| (x.chars().count(), x))
        // Break word into equal halves
        .map(|(count, word)| {
            (
                word.chars().take(count / 2).collect::<String>(),
                word.chars().skip(count / 2).collect::<String>(),
            )
        })
        // Get first character that's in both halves
        .map(|(left, right)| left.chars().filter(|c| right.contains(*c)).nth(0).unwrap())
        // Assign value based on AoC
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
        assert_eq!(run_string(input.to_string()), 157);
    }
}
