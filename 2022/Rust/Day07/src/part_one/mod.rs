use crate::args;
use crate::tree;

pub fn run(a: args::Arguments) {
    println!(
        "D07P1 {}",
        run_string(std::fs::read_to_string(a.get_input()).unwrap())
    );
}

fn run_string(s: String) -> usize {
    tree::NodePointer::default()
        .parse(s)
        .dfs()
        .map(|d| d.borrow().size)
        .filter(|x| *x < 100_000)
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    use ntest_timeout::timeout;

    #[test]
    #[timeout(1000)]
    fn it_works() {
        let input = "$ cd /
                     $ ls
                     dir a
                     14848514 b.txt
                     8504156 c.dat
                     dir d
                     $ cd a
                     $ ls
                     dir e
                     29116 f
                     2557 g
                     62596 h.lst
                     $ cd e
                     $ ls
                     584 i
                     $ cd ..
                     $ cd ..
                     $ cd d
                     $ ls
                     4060174 j
                     8033020 d.log
                     5626152 d.ext
                     7214296 k";

        println!("Testing input {}", input);
        assert_eq!(run_string(input.to_string()), 95437);
    }
}
