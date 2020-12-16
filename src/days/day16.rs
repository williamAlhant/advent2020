use advent2020::util::input;
use advent2020::util;
use anyhow::{Result, bail, Context, anyhow};
use std::collections::HashMap;

mod day16_struct;
mod day16_parser;

use day16_struct::Field;
use day16_parser::parse_field;

fn main() -> Result<()> {
    
    let lines = input::lines_from_file_passed_as_argument()?;

    let ans = do_the_thing(lines)?;
    println!("Answer: {}", ans);

    Ok(())
}

fn do_the_thing(mut lines: impl Iterator<Item = util::Result<String>>) -> Result<u32> {

    let mut fields: Vec<Field> = Vec::new();

    while let Some(line) = lines.next() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        let field = match parse_field(&line) {
            Ok((_, field)) => field,
            Err(e) => {
                bail!("Failed to parse field\nLine: {}\n{}", line, e);
            }
        };

        fields.push(field);
    }

    assert!(lines.next().unwrap()? == "your ticket:");
    let my_ticket_str = lines.next().unwrap()?;
    assert!(lines.next().unwrap()?.is_empty());

    assert!(lines.next().unwrap()? == "nearby tickets:");

    let mut invalid_numbers: Vec<u32> = Vec::new();

    while let Some(line) = lines.next() {
        let line = line?;
        for number in line.split(",") {
            let number: u32 = number.parse()?;
            let mut valid = false;
            for field in &fields {
                if field.range1.contains(&number) || field.range2.contains(&number) {
                    valid = true;
                    break;
                }
            }
            if !valid {
                invalid_numbers.push(number);
            }
        }
    }

    Ok(invalid_numbers.iter().sum())
}