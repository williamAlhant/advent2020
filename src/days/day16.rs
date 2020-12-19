use advent2020::util::input;
use advent2020::util;
use anyhow::{Result, bail, Context, anyhow};
use std::collections::HashSet;

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

fn do_the_thing(mut lines: impl Iterator<Item = util::Result<String>>) -> Result<u64> {

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
    let my_ticket = ticket_vec_from_str(&my_ticket_str)?;
    assert!(lines.next().unwrap()?.is_empty());

    assert!(lines.next().unwrap()? == "nearby tickets:");

    let mut tickets: Vec<Vec<u32>> = Vec::new();
    tickets.push(my_ticket.clone());

    while let Some(line) = lines.next() {
        let line = line?;
        if valid_ticket(&line, &fields).unwrap() {
            tickets.push(ticket_vec_from_str(&line)?);
        }
    }

    let fields_order = identify_fields_order(&tickets, &fields)?;
    
    let answer = my_ticket.iter().enumerate().fold(1 as u64, |mut acc, (index, &field_value)| {
        let field = &fields[fields_order[index]];
        if field.name.starts_with("departure") {
            acc *= field_value as u64;
        }
        acc
    });

    Ok(answer)
}

fn valid_ticket(s: &str, fields: &[Field]) -> Result<bool> {

    for number in s.split(",") {
        let number: u32 = number.parse()?;
        let mut valid = false;
        for field in fields {
            if field.is_number_allowed(number) {
                valid = true;
                break;
            }
        }
        if !valid {
            return Ok(false);
        }
    }

    return Ok(true);
}

fn ticket_vec_from_str(s: &str) -> Result<Vec<u32>> {

    let mut vec = Vec::new();

    for number in s.split(",") {
        let number: u32 = number.parse()?;
        vec.push(number);
    }

    Ok(vec)
}

fn identify_fields_order(tickets: &Vec<Vec<u32>>, fields: &Vec<Field>) -> Result<Vec<usize>> {
    
    let mut remaining_fields: HashSet<usize> = HashSet::new();
    for i in 0..fields.len() {
        remaining_fields.insert(i);
    }

    let mut remaining_ticket_columns: HashSet<usize> = HashSet::new();
    for i in 0..fields.len() {
        remaining_ticket_columns.insert(i);
    }

    let mut fields_order = vec![9999 as usize; fields.len()];

    while !remaining_fields.is_empty() {

        for &i in &remaining_ticket_columns {

            let mut possible_fields: Vec<usize> = Vec::new();

            for &field_index in &remaining_fields {

                let field = &fields[field_index];
                let mut is_match = true;

                for ticket in tickets {
                    if !field.is_number_allowed(ticket[i]) {
                        is_match = false;
                        break;
                    }
                }

                if is_match {
                    possible_fields.push(field_index);
                }
            }

            assert!(!possible_fields.is_empty());

            if possible_fields.len() == 1 {
                let &only_possible_field = possible_fields.last().unwrap();
                fields_order[i] = only_possible_field;
                remaining_fields.remove(&only_possible_field);
                remaining_ticket_columns.remove(&i);
                break;
            }
        }

    }

    assert!(fields_order.iter().all(|&i| i != 9999));
    assert!(remaining_fields.is_empty());

    Ok(fields_order)
}