use crate::args;

use itertools::Itertools;

pub fn run(a: args::Arguments) {
    println!(
        "D06P1 {}",
        run_string(std::fs::read_to_string(a.get_input()).unwrap())
    );
}

fn run_string(s: String) -> usize {
    let backup = s.clone();
    for (i, c) in s.chars().enumerate() {
        if i < 4 {
            continue;
        }
        let b = &backup[i - 3..i];
        if !b.contains(c) && b.to_string().chars().unique().collect::<String>().len() == 3 {
            return i + 1;
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = [
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];

        println!("Testing input {:?}", input);

        for i in input.iter() {
            assert_eq!(run_string(i.0.to_string()), i.1)
        }
    }
}
