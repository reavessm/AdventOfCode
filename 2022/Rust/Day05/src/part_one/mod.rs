use crate::args;
use crate::stack;

use sscanf::sscanf;

pub fn run(a: args::Arguments) {
    println!(
        "D05P1 {}",
        run_string(std::fs::read_to_string(a.get_input()).unwrap())
    );
}

fn run_string(s: String) -> String {
    let mut parts = s.split("\n\n");
    let first = parts.next().unwrap().to_string();
    let second = parts.next().unwrap().to_string();

    let mut first = first.lines().rev();

    // Trust me, this works
    let stack_count = first
        .next()
        .unwrap()
        .to_string()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let mut stack_map = stack::init_hash_map::<String>(stack_count);

    // Get value inside box
    for l in first {
        let mut index = 0;
        for (i, c) in l.chars().enumerate() {
            if c == '[' {
                // Each box is three characters plus a space, and stacks are
                // indexed by 1, for some reason.
                index = (i / 4) + 1;
            }

            if c.is_alphabetic() {
                let s = stack_map.get(&index);
                if s.is_some() {
                    let mut s = s.unwrap().as_ref().clone();
                    s.push(c.to_string());

                    stack_map.insert(index, Box::from(s));
                }
            }
        }
    }

    for l in second.lines() {
        let l = l.trim();

        let (count, from, to) = sscanf!(l, "move {usize} from {usize} to {usize}").unwrap();
        //let mut f = stack_map.get(&from).unwrap();
        let f = stack_map.get(&from);
        let t = stack_map.get(&to);
        if f.is_some() && t.is_some() {
            let mut f = f.unwrap().as_ref().clone();
            let mut t = t.unwrap().as_ref().clone();

            f.mv(&mut t, count);

            stack_map.insert(from, Box::from(f));
            stack_map.insert(to, Box::from(t));
        }
    }

    let mut s = String::new();

    if stack_map.len() > 0 {
        for index in 1..stack_map.len() + 1 {
            s += stack_map
                .get(&index)
                .unwrap()
                .as_ref()
                .peek()
                .unwrap()
                .as_str();
        }
    }

    return s;

    //for value in stack_map
    //.iter()
    //.map(|(_, s)| s.as_ref().peek().unwrap().to_string())
    //.collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        println!("Testing input {}", input);
        assert_eq!(run_string(input.to_string()), "CMZ");
    }
}
