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
        for y in 0..map.height {
            let square = map.square(x, y).unwrap();
            if square == Square::Occupied {
                num_occupied += 1;
            }
        }
    }

    Ok(num_occupied)
}

fn fill_seats(m: &mut Map) {
    let max_adjacent_seats = 5;

    let mut changed = true;
    let mut num_iter = 0;

    while changed {
        changed = false;
        let mut new_map = m.clone();

        for x in 0..m.width {
            for y in 0..m.height {
                let square = m.square(x, y).unwrap();
                let n = num_visible_occupied(x, y, &m);

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
        for yp in yp_start..=(y+1).min(m.height - 1) {
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

fn num_visible_occupied(x: usize, y: usize, m: &Map) -> usize {
    let mut num = 0;
    num += num_visible_occupied_in_direction((1, 0), x, y, m);
    num += num_visible_occupied_in_direction((0, 1), x, y, m);
    num += num_visible_occupied_in_direction((1, -1), x, y, m);
    num += num_visible_occupied_in_direction((1, 1), x, y, m);
    num
}

fn num_visible_occupied_in_direction(
    direction: Direction, x: usize, y: usize, m: &Map
) -> usize {

    let mut num = 0;

    for (xp, yp) in 
        SquareIterator::new(x, y, direction, m.width, m.height
    ) {
        let s = m.square(xp, yp).unwrap();
        if s == Square::Occupied {
            num += 1;
            break;
        }
        else if s == Square::Empty {
            break;
        }
    }

    let direction = (-direction.0, -direction.1);

    for (xp, yp) in 
        SquareIterator::new(x, y, direction, m.width, m.height
    ) {
        let s = m.square(xp, yp).unwrap();
        if s == Square::Occupied {
            num += 1;
            break;
        }
        else if s == Square::Empty {
            break;
        }
    }

    num
}

type SquareCoord = (usize, usize);
type Direction = (i32, i32);

struct SquareIterator {
    start_x: usize,
    start_y: usize,
    direction: Direction,
    x: usize,
    y: usize,
    map_width: usize,
    map_height: usize
}

impl SquareIterator {
    fn new(x: usize, y: usize, direction: Direction, map_width: usize, map_height: usize) -> Self {
        Self {
            start_x: x,
            start_y: y,
            direction,
            x,
            y,
            map_width,
            map_height
        }
    }
}

impl Iterator for SquareIterator {
    type Item = SquareCoord;

    fn next(&mut self) -> Option<SquareCoord> {

        let x = self.x as i32 + self.direction.0;
        if x < 0 || x >= self.map_width as i32 {
            return None;
        }

        let y = self.y as i32 + self.direction.1;
        if y < 0 || y >= self.map_height as i32 {
            return None;
        }

        self.x = x as usize;
        self.y = y as usize;

        Some((self.x, self.y))
    }
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
    height: usize
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

        map.height = line_count;

        Ok(map)
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
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