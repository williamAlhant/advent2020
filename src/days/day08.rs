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
    let mut instructions = Instructions::from_lines(lines)?;
    let mut context = InstrContext {
        acc: 0,
        pc: 0,
        ..Default::default()
    };
    instructions.run(&mut context)?;

    let flow = context.flow.clone();
    let mut swapped_instr_i = 0;

    while context.pc != instructions.instr.len() {

        loop {
            let instr = &mut instructions.instr[flow[swapped_instr_i]];
            match instr {
                Instruction::Acc(_) => {},
                Instruction::Jmp(v) => {
                    *instr = Instruction::Nop(v.clone());
                    break;
                },
                Instruction::Nop(v) => {
                    *instr = Instruction::Jmp(v.clone());
                    break;
                },
            }
            swapped_instr_i += 1;
        }

        // reset context before running
        context.acc = 0;
        context.pc = 0;
        context.visited.clear();
        instructions.run(&mut context)?;
        println!("Tried to swap {} out of {}", swapped_instr_i, flow.len());

        // restore original flow
        let instr = &mut instructions.instr[flow[swapped_instr_i]];
        match instr {
            Instruction::Acc(_) => {},
            Instruction::Jmp(v) => {
                *instr = Instruction::Nop(v.clone());
            },
            Instruction::Nop(v) => {
                *instr = Instruction::Jmp(v.clone());
            },
        };

        // break before increment in prev loop so must increment here
        swapped_instr_i += 1;
    }
    
    Ok(context.acc)
}

struct Instructions {
    instr: Vec<Instruction>
}

enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32)
}

impl Instruction {
    fn from_line(line: &str) -> Result<Self> {
        let param: i32 = line[4..].parse()?;
        let instr = match &line[0..3] {
            "acc" => Instruction::Acc(param),
            "jmp" => Instruction::Jmp(param),
            "nop" => Instruction::Nop(param),
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
        c.flow.clear();

        loop {
            if c.pc > instr.len() {
                bail!("pc out of instr range");
            }
            else if c.pc == instr.len() {
                return Ok(c.acc);
            }
            else if c.visited[c.pc] {
                return Ok(c.acc);
            }
            c.visited[c.pc] = true;
            c.flow.push(c.pc);
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
                Instruction::Nop(_) => {
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
    visited: Vec<bool>,
    flow: Vec<usize>
}