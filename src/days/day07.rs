use advent2020::util::input;
use advent2020::util;
use anyhow::{Result, bail};
mod day07_struct;
mod day07_parser;

fn main() -> Result<()> {
    
    let lines = input::lines_from_file_passed_as_argument()?;

    let ans = do_the_thing(lines)?;
    println!("Answer: {}", ans);

    Ok(())
}

fn do_the_thing(lines: impl Iterator<Item = util::Result<String>>) -> Result<u32> {
    bail!("");
}