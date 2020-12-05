use advent2020::util::input;
use advent2020::util;
use anyhow::{Result, bail};
use std::{iter::FromIterator, collections::HashSet};

fn main() -> Result<()> {
    
    let lines = input::lines_from_file_passed_as_argument()?;

    let ans = do_the_thing(lines)?;
    println!("Answer: {}", ans);

    Ok(())
}

fn do_the_thing(lines: impl Iterator<Item = util::Result<String>>) -> Result<u16> {
    let mut ids: HashSet<u16> = HashSet::new();
    for i in 0..1024 {
        ids.insert(i);
    }

    for line in lines.into_iter() {
        let line = line?;
        ids.remove(&id_from_str(&line)?);
    }

    let mut ids = Vec::from_iter(ids);
    ids.sort();

    for i in 1..(ids.len() - 1) {
        if ids[i - 1] != ids[i] - 1 && ids[i + 1] != ids[i] + 1 {
            return Ok(ids[i]);
        }
    }

    bail!("No suitable id found");
}

fn id_from_str(s: &str) -> Result<u16> {
    let mut id: u16 = 0;
    for (i, c) in s.chars().enumerate() {
        match c {
            'F'|'L' => (),
            'B'|'R' => id |= 1 << (9 - i),
            _ => bail!("Unexpected char")
        };
    }
    Ok(id)
}

#[cfg(test)]
mod test {
    use crate::id_from_str;

    #[test]
    fn test_id_from_str() {
        assert!(id_from_str("BFFFBBFRRR").unwrap() == 567);
    }
}