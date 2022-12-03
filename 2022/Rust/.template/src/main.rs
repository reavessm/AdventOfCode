mod args;
mod part_one;
mod part_two;

fn main() {
    match args::parse() {
        (1, a) => part_one::run(a),
        (2, a) => part_two::run(a),
        _ => eprintln!("Not implmented"),
    }
}
