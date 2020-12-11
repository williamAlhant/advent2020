use advent2020::util::input;
use advent2020::util;
use anyhow::{Result, bail, Context};

fn main() -> Result<()> {
    
    let lines = input::lines_from_file_passed_as_argument()?;

    let ans = do_the_thing(lines)?;
    println!("Answer: {}", ans);

    Ok(())
}

fn do_the_thing(lines: impl Iterator<Item = util::Result<String>>) -> Result<u64> {
    let mut nums: Vec<u64> = Vec::new();

    for line in lines {
        let line = line?;
        let num: u64 = line.parse()?;
        nums.push(num);
    }

    nums.sort();

    let mut diff_distrib: Vec<u32> = vec![1, 0, 1];

    for i in 0..(nums.len() - 1) {
        let diff = nums[i + 1] - nums[i];
        assert!(diff > 0 && diff <= 3);
        diff_distrib[(diff - 1) as usize] += 1;
    }

    let res = diff_distrib[0] * diff_distrib[2];
    Ok(res as u64)
}