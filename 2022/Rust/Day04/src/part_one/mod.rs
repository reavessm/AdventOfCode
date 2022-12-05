use crate::args;

use sscanf::sscanf;

pub fn run(a: args::Arguments) {
    println!(
        "D04P1 {}",
        run_string(std::fs::read_to_string(a.get_input()).unwrap())
    );
}

fn run_string(s: String) -> usize {
    s.lines()
        .map(|x| x.trim())
        .map(|code| {
            sscanf!(code, "{usize}-{usize},{usize}-{usize}").unwrap()
        })
    .map(|(fl, fr, sl, sr)| {
        match (fl <= sl && fr >= sr)
           || (fl >= sl && fr <= sr) {
            true => 1,
            false => 0,
        }
    })
    .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "2-4,6-8
                     2-3,4-5
                     5-7,7-9
                     2-8,3-7
                     6-6,4-6
                     2-6,4-8";

        println!("Testing input {}", input);
        assert_eq!(run_string(input.to_string()), 2);
    }
}
