pub mod config;

use std::error::Error;
use std::fmt::Display;
use std::fs;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    NS, EW, NE, NW, SW, SE, Ground, Start
}

impl TryFrom<char> for Tile {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Tile::NS),
            '-' => Ok(Tile::EW),
            'L' => Ok(Tile::NE),
            'J' => Ok(Tile::NW),
            '7' => Ok(Tile::SW),
            'F' => Ok(Tile::SE),
            '.' => Ok(Tile::Ground),
            'S' => Ok(Tile::Start),
            _ => Err("Unexpected tile type"),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::NS => "│",
            Tile::EW => "─",
            Tile::NE => "└",
            Tile::NW => "┘",
            Tile::SW => "┐",
            Tile::SE => "┌",
            Tile::Ground => " ",
            Tile::Start => "S",
        };
        write!(f, "{}", c)
    }
}

impl Tile {
    fn connects_up(&self) -> bool {
        *self == Tile::NS || *self == Tile::NE || *self == Tile::NW
    }

    fn connects_down(&self) -> bool {
        *self == Tile::NS || *self == Tile::SE || *self == Tile::SW
    }

    fn connects_left(&self) -> bool {
        *self == Tile::EW || *self == Tile::SW || *self == Tile::NW
    }

    fn connects_right(&self) -> bool {
        *self == Tile::EW || *self == Tile::SE || *self == Tile::NE
    }

    fn neighbors(&self, (x, y): (usize, usize)) -> [(usize, usize); 2] {
        // The two resulting coordinates are ordered depending on whether they are N, E, S or W of
        // the given coordinates, so that N before E before S before W.
        match self {
            Tile::NS => [(x, y-1), (x, y+1)],
            Tile::EW => [(x+1, y), (x-1, y)],
            Tile::NE => [(x, y-1), (x+1, y)],
            Tile::NW => [(x, y-1), (x-1, y)],
            Tile::SW => [(x, y+1), (x-1, y)],
            Tile::SE => [(x+1, y), (x, y+1)],
            _ => panic!("Unexpected tile"),
        }
    }
}

#[derive(Debug)]
struct Puzzle {
    width: usize,
    height: usize,
    rows: Vec<Vec<Tile>>,
    start_row: usize,
    start_col: usize,
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            for tile in row {
                write!(f, "{}", tile)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Puzzle {
    fn read_row(line: &str, expected_length: usize) -> Result<Vec<Tile>, &str> {
        let mut tiles = Vec::with_capacity(expected_length);
        if line.len() != expected_length {
            return Err("Not all lines have equal length");
        }
        for maybe_tile in line.chars().map(|c| Tile::try_from(c)) {
            match maybe_tile {
                Ok(tile) => tiles.push(tile),
                Err(err) => return Err(err),
            }
        }
        Ok(tiles)
    }

    fn update_start_position(row: &Vec<Tile>, row_index: usize, start_position: &mut Option<(usize, usize)>) -> Result<(), &'static str> {
        match row.iter().position(|t| *t == Tile::Start) {
            Some(i) => {
                if start_position.is_some() {
                    return Err("Multiple starting positions found");
                }
                *start_position = Some((i, row_index));
            },
            None => (),
        }
        Ok(())
    }

    fn from_input(input: &str) -> Result<Self, Box<dyn Error>> {
        let mut iter = input.lines();
        let first_line = iter.next().ok_or("Input empty")?;
        let width = first_line.len();
        let mut height = 1;
        let mut rows = Vec::new();
        let mut start_position: Option<(usize, usize)> = None;
        let row = Self::read_row(first_line, width)?;
        Self::update_start_position(&row, 0, &mut start_position)?;
        rows.push(row);
        for line in iter {
            let row = Self::read_row(line, width)?;
            Self::update_start_position(&row, height, &mut start_position)?;
            rows.push(row);
            height += 1;
        }
        let start_position = start_position.ok_or("No starting position found")?;
        let mut puzzle = Self { width, height, rows, start_row: start_position.1, start_col: start_position.0 };
        puzzle.rows[puzzle.start_row][puzzle.start_col] = puzzle.infer_starting_tile()?;
        Ok(puzzle)
    }

    fn infer_starting_tile(&self) -> Result<Tile, String> {
        let y = self.start_row;
        let x = self.start_col;
        let above = if y > 0 { Some(&self.rows[y - 1][x]) } else { None };
        let below = if y + 1 < self.height { Some(&self.rows[y + 1][x]) } else { None };
        let left = if x > 0 { Some(&self.rows[y][x - 1]) } else { None };
        let right = if x + 1 < self.width { Some(&self.rows[y][x + 1]) } else { None };
        let connects_up = above.map_or_else(||false, |t| t.connects_down());
        let connects_down = below.map_or_else(||false, |t| t.connects_up());
        let connects_left = left.map_or_else(||false, |t| t.connects_right());
        let connects_right = right.map_or_else(||false, |t| t.connects_left());
        let mut candidates = Vec::new();
        if connects_up {
            if connects_down {
                candidates.push(Tile::NS);
            }
            if connects_left {
                candidates.push(Tile::NW);
            }
            if connects_right {
                candidates.push(Tile::NE);
            }
        }
        if connects_down {
            if connects_left {
                candidates.push(Tile::SW);
            }
            if connects_right {
                candidates.push(Tile::SE);
            }
        }
        if connects_left && connects_right {
            candidates.push(Tile::EW);
        }
        if candidates.len() != 1 {
            return Err(format!("There are {} possibilities for the starting tile", candidates.len()));
        }
        Ok(candidates.into_iter().next().unwrap())
    }

    fn get_cycle(&self) -> Vec<(usize, usize)> {
        let mut cycle = Vec::new();
        let mut x = self.start_col;
        let mut y = self.start_row;
        cycle.push((x, y));
        let mut prev_x = x;
        let mut prev_y = y;
        loop {
            let neighbors = self.rows[y][x].neighbors((x, y));
            let next = if neighbors[0] == (prev_x, prev_y) { neighbors[1] } else { neighbors[0] };
            (prev_x, prev_y) = (x, y);
            (x, y) = next;
            cycle.push((x, y));
            if x == self.start_col && y == self.start_row {
                break;
            }
        }
        cycle
    }

    fn cycle_length(&self) -> usize {
        self.get_cycle().len() - 1
    }

    fn area_in_cycle(&self) -> usize {
        let mut cycle = self.get_cycle();
        // Remove the last element of the cycle because it's the same as the first and the
        // duplicate would mess up our algorithm
        assert_eq!(&cycle.pop().unwrap(), cycle.first().unwrap());
        cycle.sort_by(|(x1, y1), (x2, y2)| y1.cmp(y2).then(x1.cmp(x2)));
        // Similar to crossing number algorithm
        // https://en.wikipedia.org/wiki/Point_in_polygon
        // We cast a ray from left to right along each row. We switch between outside and inside
        // every time we traverse | (NS tile), L-*7 (SE, EW*, SW) and F-*J (NE, EW*, NW).
        let mut area = 0;
        let mut inside;
        let mut iter = cycle.iter();
        let mut next_cycle_pos = iter.next().expect("No cycle");
        for y in 0..self.height {
            inside = false;
            let mut last_angle_read = None;
            for x in 0..self.width {
                // let tile = &self.rows[y][x];
                let tile = &self.rows[y][x];
                if (x, y) == *next_cycle_pos {
                    match tile {
                        Tile::NS => {
                            inside = !inside;
                        },
                        Tile::EW => (),
                        Tile::NE => {
                            last_angle_read = Some(*tile);
                        },
                        Tile::NW => {
                            if last_angle_read == Some(Tile::SE) {
                                inside = !inside;
                            }
                            last_angle_read = Some(*tile);
                        },
                        Tile::SW => {
                            if last_angle_read == Some(Tile::NE) {
                                inside = !inside;
                            }
                            last_angle_read = Some(*tile);
                        },
                        Tile::SE => {
                            last_angle_read = Some(*tile);
                        },
                        _ => panic!("Unexpected tile in cycle"),
                    }
                    if let Some(next) = iter.next() {
                        next_cycle_pos = next;
                    } else {
                        return area;
                    }
                } else if inside {
                    area += 1;
                }
            }
        }
        area
    }
}

fn part1(input: &str) -> Result<usize, Box<dyn Error>> {
    let puzzle = Puzzle::from_input(input)?;
    println!("{}", puzzle);
    Ok(puzzle.cycle_length() / 2)
}

fn part2(input: &str) -> Result<usize, Box<dyn Error>> {
    let puzzle = Puzzle::from_input(input)?;
    println!("{}", puzzle);
    Ok(puzzle.area_in_cycle())
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
.....
.S-7.
.|.|.
.L-J.
";

    const EXAMPLE2: &str = "
-L|F7
7S-7|
L|7||
-L-J|
L|-JF
";

    const EXAMPLE3: &str = "
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";

    const EXAMPLE4: &str = "
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
";

    const EXAMPLE5: &str = "
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";

    const EXAMPLE6: &str = "
..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........
";

    const EXAMPLE7: &str = "
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";

    const EXAMPLE8: &str = "
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";

    #[test]
    fn example1_and_2_part1() -> Result<(), Box<dyn Error>> {
        for input in [EXAMPLE1, EXAMPLE2].iter() {
            let result = part1(input.trim())?;
            assert_eq!(result, 4);
        }
        Ok(())
    }

    #[test]
    fn example3_and_4_part1() -> Result<(), Box<dyn Error>> {
        for input in [EXAMPLE3, EXAMPLE4].iter() {
            let result = part1(input.trim())?;
            assert_eq!(result, 8);
        }
        Ok(())
    }

    #[test]
    fn example5_part2() -> Result<(), Box<dyn Error>> {
        let result = part2(EXAMPLE5.trim())?;
        assert_eq!(result, 4);
        Ok(())
    }

    #[test]
    fn example6_part2() -> Result<(), Box<dyn Error>> {
        let result = part2(EXAMPLE6.trim())?;
        assert_eq!(result, 4);
        Ok(())
    }

    #[test]
    fn example7_part2() -> Result<(), Box<dyn Error>> {
        let result = part2(EXAMPLE7.trim())?;
        assert_eq!(result, 8);
        Ok(())
    }

    #[test]
    fn example8_part2() -> Result<(), Box<dyn Error>> {
        let result = part2(EXAMPLE8.trim())?;
        assert_eq!(result, 10);
        Ok(())
    }
}
