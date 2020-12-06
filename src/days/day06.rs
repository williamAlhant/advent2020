use advent2020::util::input;
use advent2020::util;
use anyhow::{Result, bail};
use std::collections::HashMap;

fn main() -> Result<()> {
    
    let lines = input::lines_from_file_passed_as_argument()?;

    let ans = do_the_thing(lines)?;
    println!("Answer: {}", ans);

    Ok(())
}

fn do_the_thing(lines: impl Iterator<Item = util::Result<String>>) -> Result<u32> {

    let mut paragraph = String::new();
    let mut sum_of_counts = 0;

    for line in lines {
        let line = line?;
        if line.is_empty() {
            sum_of_counts += count_group(&paragraph)?;
            paragraph.clear();
        }
        else {
            paragraph.push_str(&line);
            paragraph.push('\n');
        }
    }
    if !paragraph.is_empty() {
        sum_of_counts += count_group(&paragraph)?;
    }

    Ok(sum_of_counts)
}

fn count_group(group: &String) -> Result<u32> {

    let mut map: HashMap<char, u32> = HashMap::new();
    let mut chars = group.chars();
    let mut num_lines = 0;

    while let Some(c) = chars.next() {
        if c == '\n' {
            num_lines += 1;
            continue;
        }
        match map.get_mut(&c) {
            None => {
                map.insert(c, 1);
            },
            Some(n) => *n += 1,
        }
    }

    let mut count = 0;

    for (_, n) in map {
        if n == num_lines {
            count += 1;
        }
    }

    Ok(count)
}