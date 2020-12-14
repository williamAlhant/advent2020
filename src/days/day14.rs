use advent2020::util::input;
use advent2020::util;
use anyhow::{Result, bail, Context, anyhow};
use std::collections::HashMap;

fn main() -> Result<()> {
    
    let lines = input::lines_from_file_passed_as_argument()?;

    let ans = do_the_thing(lines)?;
    println!("Answer: {}", ans);

    Ok(())
}

fn do_the_thing(lines: impl Iterator<Item = util::Result<String>>) -> Result<u64> {

    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut mask = MaskParam::default();

    for line in lines {
        let line = line?;

        let instruction = Instruction::deserialize(&line)
            .with_context(|| format!("trying to parse this line: {}", line))?;
        
        match instruction {
            Instruction::Mask(new_mask) => {
                mask = new_mask;
            },
            Instruction::Mem(instr) => {
                update_mem(&instr, &mask, &mut mem);
            }
        }
    }

    let ret = mem.iter().map(|(k, v)| v).sum();

    Ok(ret)
}

fn update_mem(instr: &MemParam, mask: &MaskParam, mem: &mut HashMap<u64, u64>) {
    let mut base = instr.addr;
    // set floating bits to 0
    for &i in &mask.floating {
        base &= !(1 << i);
    }
    // set bits to 1
    base |= mask.or;

    // update every possible address
    for floating_bits in 0..(2 as u32).pow(mask.floating.len() as u32) {
        let mut floating_mask: u64 = 0;
        for i in 0..mask.floating.len() {
            floating_mask |= ((floating_bits as u64 >> i) & 1) << mask.floating[i];
        }

        let addr = base | floating_mask;
        mem.insert(addr, instr.value);
    }
}

#[derive(Default)]
struct MaskParam {
    or: u64,
    floating: Vec<u8>
}

struct MemParam {
    addr: u64,
    value: u64
}

enum Instruction {
    Mask(MaskParam),
    Mem(MemParam)
}

impl Instruction {
    fn deserialize(s: &str) -> Result<Self> {
        if s.starts_with("mask") {
            let mask_string = &s["mask = ".len()..];
            let mut or = 0;
            let mut floating = Vec::new();

            for (i, c) in mask_string.chars().rev().enumerate() {
                match c {
                    '0' => {},
                    '1' => {
                        or |= 1 << i;
                    },
                    _ => {
                        floating.push(i as u8);
                    }
                }
            }

            Ok(Instruction::Mask(MaskParam {
                or,
                floating
            }))
        }
        else if s.starts_with("mem") {
            let arg1_pos = s.find("[").context("expected mem to be followed by [")? + 1;
            let arg1_end = arg1_pos + s[arg1_pos..].find("]").context("expected closing brace")?;
            let arg2_pos = arg1_end + "] = ".len();
            if arg2_pos >= s.len() {
                bail!("arg2_pos >= s.len")
            }

            Ok(Instruction::Mem(MemParam {
                addr: s[arg1_pos..arg1_end].parse().context("trying to parse mem instruction")?,
                value: s[arg2_pos..].parse().context("trying to parse mem instruction")?
            }))
        }
        else {
            bail!("did not start with known instruction type")
        }
    }
}