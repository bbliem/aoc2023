pub mod config;

use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::fs;
use std::iter;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Tile {
    O,
    Hash,
    Dot,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::O => write!(f, "O"),
            Tile::Hash => write!(f, "#"),
            Tile::Dot => write!(f, "."),
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'O' => Ok(Tile::O),
            '#' => Ok(Tile::Hash),
            '.' => Ok(Tile::Dot),
            _ => Err("Unexpected tile type"),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Puzzle {
    columns: Vec<Vec<Tile>>,
    num_rows: usize,
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows() {
            for tile in row {
                write!(f, "{tile}")?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Puzzle {
    fn from_input(input: &str) -> Result<Self, Box<dyn Error>> {
        let mut rows = Vec::new();
        let line_len = input.lines().next().ok_or("Empty input")?.len();
        for line in input.lines() {
            if line.len() != line_len {
                return Err("Not all lines have the same length".into());
            }
            rows.push(line.to_owned());
        }
        let num_rows = rows.len();
        let mut columns = Vec::with_capacity(line_len);
        for i in 0..line_len {
            let column = rows.iter().map(|l| Tile::try_from(l.chars().nth(i).unwrap())).collect::<Result<_,_>>()?;
            columns.push(column);
        }
        Ok(Self { columns, num_rows })
    }

    fn transpose_matrix<T: Clone>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
        let mut rows = Vec::with_capacity(matrix[0].len());
        for i in 0..matrix[0].len() {
            let row = matrix.iter().map(|col| col[i].clone()).collect();
            rows.push(row);
        }
        rows
    }

    fn rows(&self) -> Vec<Vec<Tile>> {
        Self::transpose_matrix(&self.columns)
    }

    fn handle_end_of_segment(os_in_segment: usize, segment_len: usize, tilted_column: &mut Vec<Tile>) {
        let dots_in_segment = segment_len - os_in_segment;
        tilted_column.extend(iter::repeat(Tile::O).take(os_in_segment));
        tilted_column.extend(iter::repeat(Tile::Dot).take(dots_in_segment));
    }

    fn tilt_vector(vector: &Vec<Tile>) -> Vec<Tile> {
        // A segment is a slice of a column/row between two "#" tiles. Process segments one by one,
        // counting the numbers of "O" tiles in them and, when we hit the end of the segment,
        // producing a new segment with the "O" tiles in the beginning, followed by "." tiles.
        let mut tilted_vector = Vec::with_capacity(vector.capacity());
        let mut segment_start = 0;
        let mut os_in_segment = 0;
        for (i, &tile) in vector.iter().enumerate() {
            match tile {
                Tile::O => os_in_segment += 1,
                Tile::Hash => {
                    Self::handle_end_of_segment(os_in_segment, i - segment_start, &mut tilted_vector);
                    tilted_vector.push(Tile::Hash);
                    segment_start = i + 1;
                    os_in_segment = 0;
                }
                Tile::Dot => (),
            }
        }
        Self::handle_end_of_segment(os_in_segment, vector.len() - segment_start, &mut tilted_vector);
        assert_eq!(vector.len(), tilted_vector.len());
        tilted_vector
    }

    fn tilt(&mut self) {
        self.columns = self.columns.iter().map(|col| Self::tilt_vector(col)).collect();
    }

    fn transpose(&mut self) {
        self.columns = Self::transpose_matrix(&mut self.columns);
        self.num_rows = self.columns[0].len();
    }

    fn rotate_left(&mut self) {
        self.transpose();
        self.columns.reverse();
    }

    fn tilting_cycle(&mut self) {
        for _ in 0..4 {
            self.tilt();
            self.rotate_left();
        }
    }

    fn load(&self) -> usize {
        let mut sum = 0;
        for column in &self.columns {
            for (i, &tile) in column.iter().enumerate() {
                if let Tile::O = tile {
                    sum += self.num_rows - i;
                }
            }
        }
        sum
    }
}

fn part1(input: &str) -> Result<usize, Box<dyn Error>> {
    let mut puzzle = Puzzle::from_input(input)?;
    puzzle.tilt();
    Ok(puzzle.load())
}

fn part2(input: &str) -> Result<usize, Box<dyn Error>> {
    let mut puzzle = Puzzle::from_input(input)?;
    let mut seen_at_iteration = HashMap::new();
    let num_operations = 1_000_000_000;
    for i in 0..num_operations {
        puzzle.tilting_cycle();
        if let Some(cycle_start_iteration) = seen_at_iteration.insert(puzzle.clone(), i) {
            println!("Cycle at i = {i}; same as in iteration {cycle_start_iteration}");
            let cycle_length = i - cycle_start_iteration;
            // We may still have to do a few tilts because 1 billion may be in the middle of the
            // cycle and not at its end.
            // The state at the end of iteration x is the same as
            // tilting_cycle^{(x - cycle_start_iteration) % cycle_length}(puzzle).
            let remaining_operations = (num_operations - 1 - cycle_start_iteration) % cycle_length;
            for _ in 0..remaining_operations {
                puzzle.tilting_cycle()
            }
            break;
        }
    }
    Ok(puzzle.load())
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
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";

    #[test]
    fn example1_part1() -> Result<(), Box<dyn Error>> {
        let result = part1(EXAMPLE1.trim())?;
        assert_eq!(result, 136);
        Ok(())
    }

    #[test]
    fn example1_part2() -> Result<(), Box<dyn Error>> {
        let result = part2(EXAMPLE1.trim())?;
        assert_eq!(result, 64);
        Ok(())
    }
}
