use crate::args;
use crate::tree;

pub fn run(a: args::Arguments) {
    println!(
        "Part 1: {}",
        run_string(std::fs::read_to_string(a.get_input()).unwrap())
    );
}

fn run_string(s: String) -> usize {
    let root = tree::NodePointer::default();
    let cur = root.clone();

    for line in s.lines() {
        let mut tokens = line.split_whitespace();
        let first = tokens.nth(0);

        if first.is_none() {
            break;
        }

        let second = tokens.nth(0);

        if second.is_none() {
            break;
        }

        match first.unwrap() {
            "$" => match second.unwrap() {
                "cd" => {
                    let third = tokens.nth(0);
                    if third.is_none() {
                        continue;
                    }
                    let d = cur.node.clone();
                    let d = d.as_ref().borrow_mut();
                    let d = &d.dirs;
                    let dir = d.get(third.unwrap());
                    if dir.is_none() {
                        continue;
                    }
                    cur.cd(tree::NodePointer::new(third.unwrap()));
                }
                _ => {}
            },
            "dir" => {
                let third = tokens.nth(0);
                if third.is_none() {
                    continue;
                }
                let mut t = cur.node.as_ref().borrow_mut();
                t.add_dir(third.unwrap());
            }
            num @ _ => {
                let num = num.parse::<usize>();
                if num.is_err() {
                    continue;
                }
                let num = num.unwrap();
                let mut tree = cur.node.as_ref().borrow_mut();
                tree.add_file(second.unwrap(), num)
            }
        }
    }

    let mut leaf_vec: Vec<tree::NodePointer> = Vec::new();
    root.leaf_dirs(&mut leaf_vec);

    for d in leaf_vec.iter_mut() {
        d.build_size();
    }

    // TODO: Find node < 10000 or whatever
    return root.node.borrow().size;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
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
