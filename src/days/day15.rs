use advent2020::util::input;
use advent2020::util;
use anyhow::{Result, bail, Context, anyhow};
use std::collections::HashMap;

fn main() -> Result<()> {
    
    let starting_numbers = vec![1,20,8,12,0,14];
    let until = 30000000;
    let ans = do_the_thing(&starting_numbers, until)?;
    println!("Answer: {}", ans);

    Ok(())
}

fn do_the_thing(starting_numbers: &[u64], until: u64) -> Result<u64> {

    let mut progress_indicator = ProgressIndicator::new(until);

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

        progress_indicator.update(i);
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

struct ProgressIndicator {
    total: u64,
    value_last_display: u64,
    threshold: u64,
}

impl ProgressIndicator {
    fn new(total: u64) -> Self {
        Self {
            total,
            value_last_display: 0,
            threshold: total / 10
        }
    }

    fn update(&mut self, new_value: u64) {
        assert!(new_value >= self.value_last_display);
        assert!(new_value <= self.total);
        if new_value - self.value_last_display > self.threshold {
            let percent = (new_value * 100) / self.total;
            println!("Progress: {}%", percent);
            self.value_last_display = new_value;
        }
    }
}