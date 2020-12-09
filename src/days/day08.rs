use advent2020::util::input;
use advent2020::util;
use anyhow::{Result, bail, Context};

fn main() -> Result<()> {
    
    let lines = input::lines_from_file_passed_as_argument()?;

    let ans = do_the_thing(lines)?;
    println!("Answer: {}", ans);

    Ok(())
}

fn do_the_thing(lines: impl Iterator<Item = util::Result<String>>) -> Result<i64> {
    let instructions = Instructions::from_lines(lines)?;
    let mut context = InstrContext {
        acc: 0,
        pc: 0,
        ..Default::default()
    };
    instructions.run(&mut context)
}

struct Instructions {
    instr: Vec<Instruction>
}

enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop
}

impl Instruction {
    fn from_line(line: &str) -> Result<Self> {
        let param: i32 = line[4..].parse()?;
        let instr = match &line[0..3] {
            "acc" => Instruction::Acc(param),
            "jmp" => Instruction::Jmp(param),
            "nop" => Instruction::Nop,
            _ => bail!("Unrecognized op"),
        };
        Ok(instr)
    }
}

impl Instructions {
    fn from_lines(lines: impl Iterator<Item = util::Result<String>>) -> Result<Self> {
        let mut instr: Vec<Instruction> = Vec::new();
        for line in lines {
            let line = line?;
            let instruction = Instruction::from_line(&line)
                .context("Failed to parse instruction")?;
            instr.push(instruction);
        }
        Ok(Self {
            instr
        })
    }

    fn run(&self, c: &mut InstrContext) -> Result<i64> {

        let instr = &self.instr;
        c.visited.resize(instr.len(), false);

        loop {
            if !(0..instr.len()).contains(&c.pc) {
                bail!("pc out of instr range");
            }
            else if c.visited[c.pc] {
                return Ok(c.acc);
            }
            c.visited[c.pc] = true;
            match instr[c.pc] {
                Instruction::Acc(val) => {
                    c.acc += val as i64;
                    c.pc += 1;
                },
                Instruction::Jmp(val) => {
                    let t = c.pc as i64 + val as i64;
                    if t < 0 {
                        bail!("pc + jmp val is negative");
                    }
                    c.pc = t as usize;
                },
                Instruction::Nop => {
                    c.pc += 1;
                }
            }
        }
    }
}

#[derive(Default)]
struct InstrContext {
    acc: i64,
    pc: usize,
    visited: Vec<bool>
}