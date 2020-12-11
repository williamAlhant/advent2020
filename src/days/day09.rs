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

    let part_1_answer = 393911906;

    for i in 0..nums.len() {
        let mut sum = nums[i];
        for j in (i + 1)..nums.len() {
            sum += nums[j];
            if sum == part_1_answer {
                return Ok(smallest_plus_largest(&nums[i..=j]));
            }
            else if sum > part_1_answer {
                break;
            }
        }
    }

    bail!("answer not found");
}

fn smallest_plus_largest(nums: &[u64]) -> u64 {
    let mut min = nums[0];
    let mut max = nums[0];

    for &x in nums {
        if x > max {
            max = x;
        }
        if x < min {
            min = x;
        }
    }

    min + max
}

fn print_result_part_1(nums: &Vec<u64>, valid: &Vec<bool>) {
    for (n, v) in nums.iter().zip(valid) {
        println!("{} : {}", n, v);
    }
}

fn get_part_1_answer(nums: &Vec<u64>) -> Result<u64> {
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