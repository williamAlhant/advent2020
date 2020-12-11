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

    let preamble_len = 25;
    let mut is_valid: Vec<bool> = vec![false; nums.len()];

    for i in 0..nums.len() {
        let mut j = i + 1;
        while j < nums.len() - 1 && j <= i + preamble_len - 1 {
            let mut k = j + 1;
            while k < nums.len() && k <= i + preamble_len {
                if nums[i] + nums[j] == nums[k] {
                    is_valid[k] = true;
                }
                k += 1;
            }
            j += 1;
        }
    }

    for i in preamble_len..nums.len() {
        if !is_valid[i] {
            return Ok(nums[i]);
        }
    }

    bail!("answer not found");
}

fn print_result(nums: &Vec<u64>, valid: &Vec<bool>) {
    for (n, v) in nums.iter().zip(valid) {
        println!("{} : {}", n, v);
    }
}