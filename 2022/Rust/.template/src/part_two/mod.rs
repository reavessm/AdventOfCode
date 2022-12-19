use crate::args;

pub fn run(a: args::Arguments) {
    println!(
        "D$day_padded$P2 {}",
        run_string(std::fs::read_to_string(a.get_input()).unwrap())
    );
}

fn run_string(s: String) -> usize {
    println!("{}", s);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "";

        println!("Testing input {}", input);
        assert_eq!(run_string(input.to_string()), 1);
    }
}
