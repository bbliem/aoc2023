pub mod config;

use std::error::Error;
use std::fs;

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn apply(&self, x: i32, y: i32) -> (i32, i32) {
        match self {
            Direction::Left => (x-1, y),
            Direction::Right => (x+1, y),
            Direction::Up => (x, y-1),
            Direction::Down => (x, y+1),
        }
    }
}

#[derive(Copy, Clone)]
enum Tile {
    Empty,
    MirrorSlash,
    MirrorBackslash,
    VSplit,
    HSplit,
}

impl TryFrom<char> for Tile {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Tile::Empty),
            '/' => Ok(Tile::MirrorSlash),
            '\\' => Ok(Tile::MirrorBackslash),
            '|' => Ok(Tile::VSplit),
            '-' => Ok(Tile::HSplit),
            _ => Err("Unexpected tile type"),
        }
    }
}

impl Tile {
    fn out_directions(&self, in_direction: Direction) -> Vec<Direction> {
        // in_direction is the direction of the beam when entering this tile, not the direction
        // from where that beam comes seen from the tile.
        match (self, in_direction) {
            (Tile::Empty, _) => vec![in_direction],
            (Tile::MirrorSlash, Direction::Left) => vec![Direction::Down],
            (Tile::MirrorSlash, Direction::Right) => vec![Direction::Up],
            (Tile::MirrorSlash, Direction::Up) => vec![Direction::Right],
            (Tile::MirrorSlash, Direction::Down) => vec![Direction::Left],
            (Tile::MirrorBackslash, Direction::Left) => vec![Direction::Up],
            (Tile::MirrorBackslash, Direction::Right) => vec![Direction::Down],
            (Tile::MirrorBackslash, Direction::Up) => vec![Direction::Left],
            (Tile::MirrorBackslash, Direction::Down) => vec![Direction::Right],
            (Tile::VSplit, Direction::Left) => vec![Direction::Up, Direction::Down],
            (Tile::VSplit, Direction::Right) => vec![Direction::Up, Direction::Down],
            (Tile::VSplit, _) => vec![in_direction],
            (Tile::HSplit, Direction::Up) => vec![Direction::Left, Direction::Right],
            (Tile::HSplit, Direction::Down) => vec![Direction::Left, Direction::Right],
            (Tile::HSplit, _) => vec![in_direction],
        }
    }
}

struct LightedTile {
    tile: Tile,
    left: bool,
    right: bool,
    up: bool,
    down: bool,
}

impl LightedTile {
    fn is_energized(&self) -> bool {
        self.left || self.right || self.up || self.down
    }

    fn is_energized_in_direction(&self, direction: Direction) -> bool {
        match direction {
            Direction::Left => self.left,
            Direction::Right => self.right,
            Direction::Up => self.up,
            Direction::Down => self.down,
        }
    }

    fn energize(&mut self, direction: Direction) {
        match direction {
            Direction::Left => {
                self.left = true;
            },
            Direction::Right => {
                self.right = true;
            },
            Direction::Up => {
                self.up = true;
            },
            Direction::Down => {
                self.down = true;
            },
        }
    }
}

struct Puzzle {
    rows: Vec<Vec<LightedTile>>,
}

impl Puzzle {
    fn from_input(input: &str) -> Result<Self, Box<dyn Error>> {
        let mut rows = Vec::new();
        let line_len = input.lines().next().ok_or("Empty input")?.len();
        for line in input.lines() {
            if line.len() != line_len {
                return Err("Not all lines have the same length".into());
            }
            let tiles: Vec<Tile> = line.chars().map(|c| Tile::try_from(c)).collect::<Result<_,_>>()?;
            let lighted_tiles = tiles.into_iter()
                .map(|tile| LightedTile { tile, left: false, right: false, up: false, down: false })
                .collect();
            rows.push(lighted_tiles);
        }
        // let light = rows.iter().map(|row| row.iter().map(|_| Light::None).collect()).collect();
        Ok(Self { rows })
    }

    fn in_range(&self, x: i32, y: i32) -> bool {
        y >= 0 && y < self.rows.len() as i32 && x >= 0 && x < self.rows[0].len() as i32
    }

    fn energize(&mut self, x: i32, y: i32, direction: Direction) {
        let mut stack = vec![(x, y, direction)];
        while !stack.is_empty() {
            let (x, y, direction) = stack.pop().unwrap();
            let lighted_tile = &mut self.rows[y as usize][x as usize];
            if !lighted_tile.is_energized_in_direction(direction) {
                lighted_tile.energize(direction);
                let out_directions = lighted_tile.tile.out_directions(direction);
                for next_direction in out_directions {
                    let (xn, yn) = next_direction.apply(x, y);
                    if self.in_range(xn, yn) {
                        stack.push((xn, yn, next_direction));
                    }
                }
            }
        }
    }

    fn energized_tiles(&self) -> usize {
        self.rows.iter().map(|row| row.iter().filter(|t| t.is_energized()).count()).sum()
    }

    fn reset(&mut self) {
        for row in &mut self.rows {
            row.iter_mut().for_each(|t| { t.left = false; t.right = false; t.up = false; t.down = false });
        }
    }

    fn entry_points(&self) -> Vec<(i32, i32, Direction)> {
        let mut result = Vec::new();
        let h = self.rows.len() as i32;
        let w = self.rows[0].len() as i32;
        result.extend((0..h).map(|y| (0, y, Direction::Right)));
        result.extend((0..h).map(|y| (w-1, y, Direction::Left)));
        result.extend((0..w).map(|x| (x, 0, Direction::Down)));
        result.extend((0..w).map(|x| (x, h-1, Direction::Up)));
        result
    }
}

fn part1(input: &str) -> Result<usize, Box<dyn Error>> {
    let mut puzzle = Puzzle::from_input(input)?;
    puzzle.energize(0, 0, Direction::Right);
    Ok(puzzle.energized_tiles())
}

fn part2(input: &str) -> Result<usize, Box<dyn Error>> {
    let mut puzzle = Puzzle::from_input(input)?;
    let mut max_energized = 0;
    for (x, y, direction) in puzzle.entry_points() {
        puzzle.energize(x, y, direction);
        max_energized = max_energized.max(puzzle.energized_tiles());
        puzzle.reset();
    }
    Ok(max_energized)
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

    const EXAMPLE1: &str = r"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
";

    #[test]
    fn example1_part1() -> Result<(), Box<dyn Error>> {
        let result = part1(EXAMPLE1.trim())?;
        assert_eq!(result, 46);
        Ok(())
    }

    #[test]
    fn example1_part2() -> Result<(), Box<dyn Error>> {
        let result = part2(EXAMPLE1.trim())?;
        assert_eq!(result, 51);
        Ok(())
    }
}
