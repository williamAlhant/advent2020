use advent2020::util::input;
use advent2020::util::error_handling::exit_with_error_message;

fn main() {
    
    let lines = input::lines_from_file_passed_as_argument();

    let nums: Vec<i32> = lines.map(|l| l.parse::<i32>().unwrap_or_else(exit_with_error_message)).collect();

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
