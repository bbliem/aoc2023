pub mod config;

use std::error::Error;
use std::fmt::Display;
use std::fs;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile { Empty, Galaxy }

impl TryFrom<char> for Tile {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Tile::Empty),
            '#' => Ok(Tile::Galaxy),
            _ => Err("Unexpected tile type"),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::Empty => ".",
            Tile::Galaxy => "#",
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug)]
struct Galaxy {
    // These coordinates take emptiness size into account, so they do not correspond to row /
    // column indices
    x: usize,
    y: usize,
}

impl Galaxy {
    fn distance_to(&self, other: &Galaxy) -> usize {
        // Manhattan distance
        return ((self.x as isize - other.x as isize).abs() + (self.y as isize - other.y as isize).abs()) as usize;
    }
}

#[derive(Debug)]
struct Puzzle {
    rows: Vec<Vec<Tile>>,
    galaxies: Vec<Galaxy>,
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
    fn get_row_with_size_for_line(line: &str, emptiness_size: usize) -> Result<(Vec<Tile>, usize), &'static str> {
        let tiles: Vec<Tile> = line.chars().map(|c| Tile::try_from(c)).collect::<Result<_, _>>()?;
        let size = if tiles.iter().all(|&t| t == Tile::Empty) { emptiness_size } else { 1 };
        Ok((tiles, size))
    }

    fn from_input(input: &str, emptiness_size: usize) -> Result<Self, Box<dyn Error>> {
        let mut rows = Vec::new();
        let mut row_sizes = Vec::new();
        let line_len = input.lines().next().ok_or("Empty input")?.len();
        let mut col_sizes = vec![emptiness_size; line_len];
        for line in input.lines() {
            if line.len() != line_len {
                return Err("Not all lines have the same length".into());
            }
            let (row, row_size) = Self::get_row_with_size_for_line(line, emptiness_size)?;
            for (&tile, size) in row.iter().zip(col_sizes.iter_mut()) {
                if tile != Tile::Empty {
                    *size = 1;
                }
            }
            rows.push(row);
            row_sizes.push(row_size);
        }
        // Get galaxies
        let mut galaxies = Vec::new();
        let mut y = 0;
        for (i, row) in rows.iter().enumerate() {
            let mut x = 0;
            for (j, &tile) in row.iter().enumerate() {
                if tile == Tile::Galaxy {
                    galaxies.push(Galaxy { x, y });
                }
                x += col_sizes[j];
            }
            y += row_sizes[i];
        }
        Ok(Self { rows, galaxies })
    }

    fn sum_of_galaxy_pair_distances(&self) -> usize {
        let mut sum = 0;
        for (i, g1) in self.galaxies.iter().enumerate() {
            for g2 in self.galaxies.iter().skip(i + 1) {
                let distance = g1.distance_to(g2);
                sum += distance;
            }
        }
        sum
    }
}

fn part1(input: &str) -> Result<usize, Box<dyn Error>> {
    let puzzle = Puzzle::from_input(input, 2)?;
    Ok(puzzle.sum_of_galaxy_pair_distances())
}

fn part2(input: &str) -> Result<usize, Box<dyn Error>> {
    let puzzle = Puzzle::from_input(input, 1000000)?;
    Ok(puzzle.sum_of_galaxy_pair_distances())
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
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

    #[test]
    fn example1_part1() -> Result<(), Box<dyn Error>> {
        let result = part1(EXAMPLE1.trim())?;
        assert_eq!(result, 374);
        Ok(())
    }

    #[test]
    fn example1_part2() -> Result<(), Box<dyn Error>> {
        let input = EXAMPLE1.trim();
        let puzzle = Puzzle::from_input(input, 10)?;
        assert_eq!(puzzle.sum_of_galaxy_pair_distances(), 1030);
        let puzzle = Puzzle::from_input(input, 100)?;
        assert_eq!(puzzle.sum_of_galaxy_pair_distances(), 8410);
        Ok(())
    }
}
