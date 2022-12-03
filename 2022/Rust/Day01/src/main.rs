mod args;
mod part_one;
mod part_two;

use clap::Parser;

fn main() {
    let a = args::Arguments::parse();
    match a.part {
        1 => part_one::run(a),
        2 => part_two::run(a),
        _ => eprintln!("Not implmented"),
    }
}
