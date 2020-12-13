use advent2020::util::input;
use advent2020::util;
use anyhow::{Result, bail, Context, anyhow};
use nalgebra::Vector2;

fn main() -> Result<()> {
    
    let lines = input::lines_from_file_passed_as_argument()?;

    let ans = do_the_thing(lines)?;
    println!("Answer: {}", ans);

    Ok(())
}

fn do_the_thing(lines: impl Iterator<Item = util::Result<String>>) -> Result<u64> {

    let mut ship_state = ShipState {
        direction: Vec2::from(Direction::EAST),
        position: Vec2::from([0, 0])
    };

    for line in lines {
        let line = line?;
        let c = line.as_bytes()[0] as char;
        let n: i32 = line[1..].parse()?;
        match c {
            'E' => ship_state.move_in_direction(Vec2::from(Direction::EAST), n),
            'W' => ship_state.move_in_direction(Vec2::from(Direction::WEST), n),
            'N' => ship_state.move_in_direction(Vec2::from(Direction::NORTH), n),
            'S' => ship_state.move_in_direction(Vec2::from(Direction::SOUTH), n),
            'F' => ship_state.move_forward(n),
            'L' | 'R' => {
                assert!(n % 90 == 0);
                let quarters_abs: usize = n.abs() as usize / 90;
                match c {
                    'L' => ship_state.turn_quarters_counterclockwise(quarters_abs),
                    'R' => ship_state.turn_quarters_clockwise(quarters_abs),
                    _ => ()
                }
            }
            _ => bail!("unexpected char")
        }
    }

    println!("final pos: {}", ship_state.position);

    let p = ship_state.position;
    let res = p.fold(0, |acc, x| acc + x.abs());
    Ok(res as u64)
}

type Vec2 = Vector2<i32>;

struct ShipState {
    direction: Vec2,
    position: Vec2
}

impl ShipState {
    fn move_forward(&mut self, length: i32) {
        self.position = self.position + self.direction * length;
    }

    fn move_in_direction(&mut self, direction: Vec2, length: i32) {
        self.position = self.position + direction * length;
    }

    fn turn_quarters_clockwise(&mut self, quarters: usize) {
        for _ in 0..quarters {
            self.direction = Vec2::new(self.direction[1], -self.direction[0]);
        }
    }

    fn turn_quarters_counterclockwise(&mut self, quarters: usize) {
        for _ in 0..quarters {
            self.direction = Vec2::new(-self.direction[1], self.direction[0]);
        }
    }
}

struct Direction;

impl Direction {
    const EAST: [i32; 2] = [1, 0];
    const WEST: [i32; 2] = [-1, 0];
    const NORTH: [i32; 2] = [0, 1];
    const SOUTH: [i32; 2] = [0, -1];
}