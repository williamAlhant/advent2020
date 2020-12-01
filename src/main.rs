use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::env;
use std::process;

fn main() {
    let mut args = env::args();
    assert!(args.len() >= 2);
    let input_path = args.nth(1).unwrap();
    let file = File::open(input_path).unwrap_or_else(exit_with_error_message);
    let lines = BufReader::new(file).lines()
        .map(|l| l.unwrap_or_else(exit_with_error_message))
        .map(|l| l.parse::<i32>().unwrap_or_else(exit_with_error_message));
    let nums: Vec<i32> = lines.collect();

    for i in 0..nums.len() {
        for j in i..nums.len() {
            for k in j..nums.len() {
                if nums[i] + nums[j] + nums[k] == 2020 {
                    println!("Answer: {}", nums[i] * nums[j] * nums[k]);
                    return;
                }
            }
        }
    }
}

fn exit_with_error_message<ROk, RErr>(e: RErr) -> ROk
    where RErr: std::fmt::Display {
    println!("{}", e);
    process::exit(-1);
}