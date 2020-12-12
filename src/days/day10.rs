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
    nums.push(0); // makes it easier for part 2

    for line in lines {
        let line = line?;
        let num: u64 = line.parse()?;
        nums.push(num);
    }

    nums.sort();

    let mut optional_els: Vec<usize> = Vec::new();

    for i in 1..nums.len() - 1 {
        if nums[i + 1] - nums[i - 1] <= 3 {
            optional_els.push(i);
        }
    }

    let num_optionals = optional_els.len() as u32;
    println!(
        "{} optional elements, giving an upper bound of {}",
        num_optionals,
        (2 as u64).pow(num_optionals)
    );

    // number of arrangements up to num n
    let mut s_n: u64 = match nums[2] - nums[0] {
        2 => 2,
        _ => 1
    }; 
    let mut s_nm1: u64 = 1; // number of arrangement up to num n - 1
    let mut s_nm2: u64 = 1; 
    for j in 2..nums.len() - 1 {
        // number of arrangement up to num n + 1
        let s_np1 = match nums[j + 1] - nums[j - 1] {
            2 => match nums[j + 1] - nums[j - 2] {
                3 => s_n + s_nm1 + s_nm2,
                _ => s_n + s_nm1
            },
            3 => s_n + s_nm1,
            _ => s_n
        };
        s_nm2 = s_nm1;
        s_nm1 = s_n;
        s_n = s_np1;
    }

    Ok(s_n)
}

fn get_part_1_answer(sorted_nums: &Vec<u64>) -> u64 {
    let nums = sorted_nums;
    let mut diff_distrib: Vec<u32> = vec![1, 0, 1];

    for i in 0..(nums.len() - 1) {
        let diff = nums[i + 1] - nums[i];
        assert!(diff > 0 && diff <= 3);
        diff_distrib[(diff - 1) as usize] += 1;
    }

    let res = diff_distrib[0] * diff_distrib[2];
    res as u64
}