use clap::Parser;
use std::path::PathBuf;

pub static DEFAULT_INPUT: &str = "input.txt";

#[derive(Debug, Parser)]
#[clap(
    author = "Stephen M. Reaves",
    version = "2022.03",
    about = "Solves Advent of Code 2022 Day 3 - https://adventofcode.com/2022/day/3"
)]
pub struct Arguments {
    /// Which part are we doing? <1|2>
    #[clap(short, long, value_parser = clap::value_parser!(u8).range(1..3))]
    pub part: u8,

    /// Input file to use
    #[clap(short, long)]
    pub input: Option<PathBuf>,
}

impl Arguments {
    pub fn get_input(self) -> PathBuf {
        self.input.unwrap_or(PathBuf::from(DEFAULT_INPUT))
    }
}

pub fn parse() -> (u8, Arguments) {
    let a = Arguments::parse();
    (a.part, a)
}
