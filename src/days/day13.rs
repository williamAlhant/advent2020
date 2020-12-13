use advent2020::util::input;
use advent2020::util;
use anyhow::{Result, bail, Context, anyhow};
use modinverse::modinverse;

fn main() -> Result<()> {
    
    let lines = input::lines_from_file_passed_as_argument()?;

    let ans = do_the_thing(lines)?;
    println!("Answer: {}", ans);

    Ok(())
}

fn do_the_thing(mut lines: impl Iterator<Item = util::Result<String>>) -> Result<u64> {

    let first_line = unwrap_opt_result_str(lines.next())?;
    let second_line = unwrap_opt_result_str(lines.next())?;

    let mut coprimes: Vec<u64> = Vec::new();
    let mut remainders: Vec<u64> = Vec::new();

    for (i, bus_id) in second_line.split(",").enumerate() {
        if bus_id == "x" {
            continue;
        }

        let bus_id: u64 = bus_id.parse()?;
        coprimes.push(bus_id);
        
        let remainder = (bus_id - ((i as u64) % bus_id)) % bus_id; // ugly way to convert -i to positive remainder
        remainders.push(remainder);
    }

    let res = chinese_remainders(&coprimes, &remainders);

    Ok(res)
}

fn chinese_remainders(coprimes: &Vec<u64>, remainders: &Vec<u64>) -> u64 {
    let mut x = 0;
    let product_of_m = coprimes.iter().fold(1, |acc, x| acc * x);
    for (&m, &a) in coprimes.iter().zip(remainders) {
        let b = product_of_m / m;
        // modinverse is broken with unsigned arguments
        let bp = modinverse(b as i64, m as i64).unwrap() as u64;
        x += a * b * bp;
        x = x % product_of_m;
    }
    x
}

fn min_index(v: &Vec<u64>) -> usize {
    let mut min = *v.first().unwrap();
    let mut min_index = 0;
    for (i, &x) in v.iter().enumerate() {
        if x < min {
            min = x;
            min_index = i;
        }
    }
    min_index
}

fn unwrap_opt_result_str(opt: Option<Result<String, util::Error>>) -> Result<String> {
    match opt {
        None => bail!("no str"),
        Some(v) => v.map_err(|e| anyhow!(e))
    }
}