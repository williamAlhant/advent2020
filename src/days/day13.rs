use advent2020::util::input;
use advent2020::util;
use anyhow::{Result, bail, Context, anyhow};

fn main() -> Result<()> {
    
    let lines = input::lines_from_file_passed_as_argument()?;

    let ans = do_the_thing(lines)?;
    println!("Answer: {}", ans);

    Ok(())
}

fn do_the_thing(mut lines: impl Iterator<Item = util::Result<String>>) -> Result<u64> {

    let first_line = unwrap_opt_result_str(lines.next())?;
    let second_line = unwrap_opt_result_str(lines.next())?;

    let min_timestamp: u64 = first_line.parse()?;

    let mut bus_ids: Vec<u64> = Vec::new();
    for bus_id in second_line.split(",") {
        if bus_id == "x" {
            continue;
        }

        let bus_id: u64 = bus_id.parse()?;
        bus_ids.push(bus_id);
    }

    // for each bus_id, first departure at/after min_timestamp
    let mut departures: Vec<u64> = Vec::new();
    for &bus_id in &bus_ids {
        let q = min_timestamp / bus_id;
        let r = min_timestamp % bus_id;

        let departure = match r {
            0 => min_timestamp,
            _ => (q + 1) * bus_id
        };

        departures.push(departure);
    }

    let min_departure_index = min_index(&departures);
    let min_departure = departures[min_departure_index];
    let min_bus_id = bus_ids[min_departure_index];

    Ok((min_departure - min_timestamp) * min_bus_id)
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