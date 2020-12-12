use advent2020::util::input;
use advent2020::util;
use anyhow::{Result, bail, Context, anyhow};
use std::fmt::Write;

fn main() -> Result<()> {
    
    let lines = input::lines_from_file_passed_as_argument()?;

    let ans = do_the_thing(lines)?;
    println!("Answer: {}", ans);

    Ok(())
}

fn do_the_thing(lines: impl Iterator<Item = util::Result<String>>) -> Result<u64> {

    let mut map = Map::from_lines(lines)?;

    fill_seats(&mut map);

    let mut num_occupied = 0;

    for x in 0..map.width {
        for y in 0..map.heigth {
            let square = map.square(x, y).unwrap();
            if square == Square::Occupied {
                num_occupied += 1;
            }
        }
    }

    Ok(num_occupied)
}

fn fill_seats(m: &mut Map) {
    let max_adjacent_seats = 4;

    let mut changed = true;
    let mut num_iter = 0;

    while changed {
        changed = false;
        let mut new_map = m.clone();

        for x in 0..m.width {
            for y in 0..m.heigth {
                let square = m.square(x, y).unwrap();
                let n = num_adjacent_occupied(x, y, &m);

                match square {
                    Square::Empty => {
                        if n == 0 {
                            new_map.set_square(x, y, Square::Occupied);
                            changed = true;
                        }
                    },
                    Square::Occupied => {
                        if n >= max_adjacent_seats {
                            new_map.set_square(x, y, Square::Empty);
                            changed = true;
                        }
                    },
                    Square::Floor => {},
                }
            }
        }

        *m = new_map;
        num_iter += 1;
    }
}

fn num_adjacent_occupied(x: usize, y: usize, m: &Map) -> usize {
    let mut n = 0;
    let xp_start = (x as i64 -1).max(0) as usize;
    let yp_start = (y as i64 -1).max(0) as usize;
    for xp in xp_start..=(x+1).min(m.width - 1) {
        for yp in yp_start..=(y+1).min(m.heigth - 1) {
            if m.square(xp, yp).unwrap() == Square::Occupied {
                n += 1;
            }
        }
    }

    if m.square(x, y).unwrap() == Square::Occupied {
        n -= 1;
    }

    n
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Square {
    Empty,
    Occupied,
    Floor
}

#[derive(Default, Clone)]
struct Map {
    data: Vec<Square>,
    width: usize,
    heigth: usize
}

impl Map {

    fn extend_data(&mut self, squares: &[Square]) {
        self.data.extend_from_slice(squares);
    }

    fn square(&self, x: usize, y: usize) -> Option<Square> {
        if y * self.width + x > self.data.len() {
            return None;
        }

        Some(self.data[y * self.width + x])
    }

    fn set_square(&mut self, x: usize, y: usize, v: Square) {
        self.data[y * self.width + x] = v;
    }

    fn from_lines(lines: impl Iterator<Item = util::Result<String>>) -> Result<Self> {
    
        let mut map = Map::default();
        let mut lines = lines.into_iter();
        
        let first_line = lines.next().ok_or_else(|| anyhow!("No lines"))??;
        let mut line_count = 1;

        map.width = first_line.len();

        let mut squares: Vec<Square> = Vec::new();
        squares.resize(map.width, Square::Floor);
        
        line_to_squares(&first_line, &mut squares)?;
        map.extend_data(&squares);

        for line in lines {
            let line = line?;
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
                    Square::Floor => '.',
                    Square::Empty => 'L',
                    Square::Occupied => '#',
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
            '.' => Square::Floor,
            'L' => Square::Empty,
            '#' => Square::Occupied,
            _ => bail!("Unexpected square char")
        };
    }

    Ok(())
}