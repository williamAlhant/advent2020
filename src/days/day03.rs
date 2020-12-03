use advent2020::util::input;
use anyhow::{Context, Result, anyhow, bail};
use std::fmt::Write;

fn main() -> Result<()> {
    
    let lines = input::lines_from_file_passed_as_argument();

    let ans = do_the_thing(lines)?;
    println!("Answer: {}", ans);

    Ok(())
}

fn do_the_thing<L>(lines: L) -> Result<u64>
    where L: IntoIterator<Item = String> {

    let map = Map::from_lines(lines).context("Map construction")?;
    
    let a0 = find_num_trees(&map, 1, 1).unwrap() as u64;
    let a1 = find_num_trees(&map, 3, 1).unwrap() as u64;
    let a2 = find_num_trees(&map, 5, 1).unwrap() as u64;
    let a3 = find_num_trees(&map, 7, 1).unwrap() as u64;
    let a4 = find_num_trees(&map, 1, 2).unwrap() as u64;

    Ok(a0 * a1 * a2 * a3 * a4)
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Square {
    Open,
    Tree
}

struct Map {
    data: Vec<Square>,
    width: usize,
    heigth: usize
}

impl Map {
    fn new() -> Self {
        Self {
            data: Vec::new(),
            width: 0,
            heigth: 0
        }
    }

    fn extend_data(&mut self, squares: &[Square]) {
        self.data.extend_from_slice(squares);
    }

    fn square(&self, x: usize, y: usize) -> Option<Square> {
        if y * self.width + x > self.data.len() {
            return None;
        }

        Some(self.data[y * self.width + x])
    }

    fn from_lines<L>(lines: L) -> Result<Self>
        where L: IntoIterator<Item = String> {
    
        let mut map = Map::new();
        let mut lines = lines.into_iter();
        
        let first_line = lines.next().ok_or_else(|| anyhow!("No lines"))?;
        let mut line_count = 1;

        map.width = first_line.len();

        let mut squares: Vec<Square> = Vec::new();
        squares.resize(map.width, Square::Open);
        
        line_to_squares(&first_line, &mut squares)?;
        map.extend_data(&squares);

        for line in lines {
            line_to_squares(&line, &mut squares)?;
            map.extend_data(&squares);
            line_count += 1;
        }

        map.heigth = line_count;

        Ok(map)
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.heigth {
            for x in 0..self.width {
                let square = self.square(x, y).unwrap();
                f.write_char(match square {
                    Square::Open => '.',
                    Square::Tree => '#'
                })?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

fn line_to_squares(line: &String, squares: &mut Vec<Square>) -> Result<()> {
    if line.len() != squares.len() {
        bail!("Unexpected line size");
    }

    for (i, c) in line.chars().enumerate() {
        squares[i] = match c {
            '.' => Square::Open,
            '#' => Square::Tree,
            _ => bail!("Unexpected square char")
        };
    }

    Ok(())
}

fn find_num_trees(map: &Map, slope_right: usize, slope_down: usize) -> Option<u32> {
    let mut num_trees = 0;
    let mut x = 0;
    for y in (0..map.heigth).step_by(slope_down) {
        if map.square(x, y).unwrap() == Square::Tree {
            num_trees += 1;
        }
        x = (x + slope_right) % map.width;
    }
    Some(num_trees)
}