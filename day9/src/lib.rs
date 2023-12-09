pub mod config;

use std::error::Error;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

struct Sequence {
    elements: Vec<i32>,
}

impl FromStr for Sequence {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elements = s.split(' ').map(|v| v.trim()).filter(|v| !v.is_empty())
            .map(|v| v.parse())
            .collect::<Result<_, _>>()?;
        Ok(Self { elements })
    }
}

impl Sequence {
    fn from_differences(&self) -> Self {
        if self.elements.len() < 2 {
            Self { elements: vec![] }
        } else {
            Self {
                elements: self.elements.iter().zip(self.elements.iter().skip(1)).map(|(a, b)| b - a).collect()
            }
        }
    }

    fn extrapolate_next_value(&self, reverse: bool) -> i32 {
        if self.elements.iter().all(|&i| i == 0) {
            0
        } else {
            let next_sequence = self.from_differences();
            let extrapolated = next_sequence.extrapolate_next_value(reverse);
            if reverse {
                self.elements.first().unwrap() - extrapolated
            } else {
                self.elements.last().unwrap() + extrapolated
            }
        }
    }
}

struct Puzzle {
    histories: Vec<Sequence>,
}

impl Puzzle {
    fn from_input(input: &str) -> Result<Self, Box<dyn Error>> {
        let histories = input.lines().map(|line| line.trim()).filter(|line| !line.is_empty())
            .map(|line| Sequence::from_str(line)).collect::<Result<_, _>>()?;
        Ok(Self { histories })
    }

    fn sum_extrapolated_values(&self, reverse: bool) -> i32 {
        self.histories.iter().map(|seq| seq.extrapolate_next_value(reverse)).sum()
    }
}

fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let puzzle = Puzzle::from_input(input)?;
    Ok(puzzle.sum_extrapolated_values(false))
}

fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    let puzzle = Puzzle::from_input(input)?;
    Ok(puzzle.sum_extrapolated_values(true))
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

    const EXAMPLE: &str = "
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    #[test]
    fn example_part1() -> Result<(), Box<dyn Error>> {
        let result = part1(EXAMPLE.trim())?;
        assert_eq!(result, 114);
        Ok(())
    }

    #[test]
    fn example_part2() -> Result<(), Box<dyn Error>> {
        let result = part2(EXAMPLE.trim())?;
        assert_eq!(result, 2);
        Ok(())
    }
}
