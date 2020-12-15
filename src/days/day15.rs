use advent2020::util::input;
use advent2020::util;
use anyhow::{Result, bail, Context, anyhow};
use std::collections::HashMap;

fn main() -> Result<()> {
    
    let starting_numbers = vec![1,20,8,12,0,14];
    let until = 2020;
    let ans = do_the_thing(&starting_numbers, until)?;
    println!("Answer: {}", ans);

    Ok(())
}

fn do_the_thing(starting_numbers: &[u64], until: u64) -> Result<u64> {

    let mut occurences: HashMap<u64, Occur> = HashMap::new();
    let mut last_number = 0;

    for (i, &number) in starting_numbers.iter().enumerate() {
        occurences.insert(number, Occur::new(i as u64));
        last_number = number;
    }

    for i in starting_numbers.len() as u64..until {
        let occur = occurences.get_mut(&last_number).unwrap();
        let next = match occur.nm2 {
            None => 0,
            Some(nm2) => occur.nm1 - nm2,
        };

        let occur = occurences.get_mut(&next);
        match occur {
            None => {
                occurences.insert(next, Occur::new(i));
            },
            Some(occur) => occur.update(i)
        }

        last_number = next;
    }

    Ok(last_number)
}

struct Occur {
    nm1: u64,
    nm2: Option<u64>
}

impl Occur {
    fn new(nm1: u64) -> Self {
        Self {
            nm1,
            nm2: None
        }
    }

    fn update(&mut self, new: u64) {
        assert!(new > self.nm1);
        self.nm2 = Some(self.nm1);
        self.nm1 = new;
    }
}