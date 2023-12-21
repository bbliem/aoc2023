pub mod config;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt::Display;
use std::fs;

#[derive(Clone, Debug)]
struct Tile {
    dug: bool,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.dug {
            write!(f, "#")
        } else {
            write!(f, ".")
        }
    }
}

#[derive(Debug)]
enum Direction { L, R, U, D }


#[derive(Debug)]
struct Puzzle {
    w: usize,
    h: usize,
    outline_length: usize,
    rows: Vec<Vec<Tile>>,
    row_heights: Vec<usize>,
    column_widths: Vec<usize>,
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            for tile in row {
                write!(f, "{tile}")?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Puzzle {
    fn from_input_part1(input: &str) -> Result<Self, Box<dyn Error>> {
        let mut instructions: Vec<(Direction, usize)> = Vec::new();
        for line in input.lines() {
            let parts: Vec<&str> = line.split(' ').collect();
            if parts.len() != 3 {
                return Err("Invalid number of parts in line".into());
            }
            let direction = match parts[0] {
                "L" => Direction::L,
                "R" => Direction::R,
                "U" => Direction::U,
                "D" => Direction::D,
                _ => return Err("Invalid direction".into()),
            };
            let length: usize = parts[1].parse()?;
            instructions.push((direction, length));
        }
        // Get dimensions and starting position
        let mut x = 0;
        let mut y = 0;
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;
        for (direction, length) in &instructions {
            match direction {
                Direction::L => x -= *length as i32,
                Direction::R => x += *length as i32,
                Direction::U => y -= *length as i32,
                Direction::D => y += *length as i32,
            }
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }
        let w = (max_x - min_x + 1) as usize;
        let h = (max_y - min_y + 1) as usize;
        let mut rows = vec![vec![Tile { dug: false }; w]; h];
        // Starting position
        let mut x = -min_x as usize;
        let mut y = -min_y as usize;
        // Dig
        let mut outline_length = 0;
        for (direction, length) in instructions {
            let mut dig_x = x..x;
            let mut dig_y = y..y;
            match direction {
                Direction::L => dig_x.start = x-length,
                Direction::R => dig_x = x+1..x+length+1,
                Direction::U => dig_y.start = y-length,
                Direction::D => dig_y = y+1..y+length+1,
            };
            for x in dig_x {
                let tile = &mut rows[y][x];
                assert!(!tile.dug);
                tile.dug = true;
            }
            for y in dig_y {
                let tile = &mut rows[y][x];
                assert!(!tile.dug);
                tile.dug = true;
            }
            outline_length += length;
            x = match direction {
                Direction::L => x - length,
                Direction::R => x + length,
                _ => x,
            };
            y = match direction {
                Direction::U => y - length,
                Direction::D => y + length,
                _ => y,
            };
        }
        let row_heights = vec![1; h];
        let column_widths = vec![1; w];
        Ok(Self { rows, w, h, outline_length, row_heights, column_widths })
    }

    fn from_input_part2(input: &str) -> Result<Self, Box<dyn Error>> {
        let mut instructions: Vec<(Direction, usize)> = Vec::new();
        for line in input.lines() {
            let parts: Vec<&str> = line.split(' ').collect();
            if parts.len() != 3 {
                return Err("Invalid number of parts in line".into());
            }
            let mut hex_str: String = parts[2].chars().skip(2).take(parts[2].len() - 3).collect();
            let direction = match hex_str.pop().ok_or("Invalid hex string")? {
                '0' => Direction::R,
                '1' => Direction::D,
                '2' => Direction::L,
                '3' => Direction::U,
                _ => return Err("Invalid direction".into()),
            };
            let length: usize = usize::from_str_radix(&hex_str, 16)?;
            instructions.push((direction, length));
        }
        // Get dimensions and starting position
        let mut x_coordinates = BTreeSet::from([0]);
        let mut y_coordinates = BTreeSet::from([0]);
        let mut x = 0;
        let mut y = 0;
        for (direction, length) in &instructions {
            match direction {
                Direction::L => x -= *length as i32,
                Direction::R => x += *length as i32,
                Direction::U => y -= *length as i32,
                Direction::D => y += *length as i32,
            }
            x_coordinates.insert(x);
            y_coordinates.insert(y);
        }
        let w = 2 * x_coordinates.len() - 1;
        let h = 2 * y_coordinates.len() - 1;
        let mut rows = vec![vec![Tile { dug: false }; w]; h];
        let mut row_heights = Vec::from([1]);
        for (y, yn) in y_coordinates.iter().zip(y_coordinates.iter().skip(1)) {
            row_heights.push((yn - y - 1) as usize);
            row_heights.push(1);
        }
        let mut column_widths = Vec::from([1]);
        for (x, xn) in x_coordinates.iter().zip(x_coordinates.iter().skip(1)) {
            column_widths.push((xn - x - 1) as usize);
            column_widths.push(1);
        }

        // Starting position
        let mut x = 2 * x_coordinates.range(..0).count();
        let mut y = 2 * y_coordinates.range(..0).count();
        // Dig
        let mut outline_length = 0;
        for (direction, length) in instructions {
            let mut dug = 0;
            while dug < length {
                let tile = &mut rows[y][x];
                assert_eq!(tile.dug, false);
                tile.dug = true;
                assert!(row_heights[y] == 1 || column_widths[x] == 1);
                dug += row_heights[y] * column_widths[x];
                match direction {
                    Direction::L => x -= 1,
                    Direction::R => x += 1,
                    Direction::U => y -= 1,
                    Direction::D => y += 1,
                }
            }
            assert_eq!(dug, length);
            outline_length += length;
        }
        Ok(Self { rows, w, h, outline_length, column_widths, row_heights })
    }

    fn get_fill_start(&self) -> (usize, usize) {
        let y = 1;
        let x = self.rows[y].iter().position(|tile| tile.dug).unwrap() + 1;
        assert!(!self.rows[y][x].dug);
        (x, y)
    }

    fn dig_interior(&mut self) -> usize {
        let mut area = 0;
        let mut stack = vec![self.get_fill_start()];
        while let Some((x, y)) = stack.pop() {
            if !self.rows[y][x].dug {
                self.rows[y][x].dug = true;
                area += self.row_heights[y] * self.column_widths[x];
                if x > 0 {
                    stack.push((x - 1, y));
                }
                if x < self.w {
                    stack.push((x + 1, y));
                }
                if y > 0 {
                    stack.push((x, y - 1));
                }
                if y < self.h {
                    stack.push((x, y + 1));
                }
            }
        }
        area
    }
}

fn part1(input: &str) -> Result<usize, Box<dyn Error>> {
    let mut puzzle = Puzzle::from_input_part1(input)?;
    let area_dug = puzzle.dig_interior();
    let area = puzzle.outline_length + area_dug;
    Ok(area)
}

fn part2(input: &str) -> Result<usize, Box<dyn Error>> {
    let mut puzzle = Puzzle::from_input_part2(input)?;
    let area_dug = puzzle.dig_interior();
    let area = puzzle.outline_length + area_dug;
    Ok(area)
}

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    println!("Part 1: Reading file {}", config.file_path1);
    let contents = fs::read_to_string(config.file_path1)?;
    let result = part1(&contents)?;
    println!("Result of part 1: {result}");

    println!("Part 2: Reading file {}", config.file_path2);
    let contents = fs::read_to_string(config.file_path2)?;
    let result = part2(&contents)?;
    println!("Result of part 2: {result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";

    #[test]
    fn example1_part1() -> Result<(), Box<dyn Error>> {
        let result = part1(EXAMPLE1.trim())?;
        assert_eq!(result, 62);
        Ok(())
    }

    #[test]
    fn example1_part2() -> Result<(), Box<dyn Error>> {
        let result = part2(EXAMPLE1.trim())?;
        assert_eq!(result, 952408144115);
        Ok(())
    }
}
