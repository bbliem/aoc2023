pub mod config;

use std::error::Error;
use std::cmp;
use std::fmt::Display;
use std::fs;

#[derive(Debug)]
struct Pattern {
    rows: Vec<String>,
    columns: Vec<String>, // the same, but transposed
}

impl Pattern {
    fn next_from_iter<'a>(iter: &mut impl Iterator<Item = &'a str>) -> Result<Option<Self>, Box<dyn Error>> {
        let mut rows = Vec::new();
        let mut line_len = None;
        while let Some(line) = iter.next() {
            if line.is_empty() {
                break;
            }
            if let Some(len) = line_len {
                if line.len() != len {
                    return Err("Lines with different length".into())
                }
            } else {
                line_len = Some(line.len());
            }
            rows.push(line.to_owned());
        }
        if rows.is_empty() {
            return Ok(None)
        }
        let line_len = line_len.unwrap();
        let mut columns = Vec::with_capacity(line_len);
        for i in 0..line_len {
            let column = rows.iter().map(|l| l.chars().nth(i).unwrap()).collect();
            columns.push(column);
        }
        Ok(Some(Self { rows, columns }))
    }

    fn symmetric(strings_before: &[String], strings_after: &[String]) -> bool {
        strings_before.iter().zip(strings_after.iter().rev()).all(|(r, s)| r == s)
    }

    fn symmetric_after_fixing_smudge(strings_before: &[String], strings_after: &[String]) -> bool {
        let num_errors: usize = strings_before.iter().zip(strings_after.iter().rev())
            .map(|(r, s)| r.chars().zip(s.chars()).filter(|(c, d)| c != d).count()).sum();
        num_errors == 1
    }

    fn find_symmetry_number(&self, strings: &[String]) -> Option<usize> {
        for i in 1..strings.len() {
            let symmetry_size = cmp::min(i, strings.len() - i);
            let strings_before = &strings[i - symmetry_size..i];
            let strings_after = &strings[i..i + symmetry_size];
            if Self::symmetric(&strings_before, &strings_after) {
                return Some(i);
            }
        }
        None
    }

    fn find_symmetry_number_after_fixing_smudge(&self, strings: &[String]) -> Option<usize> {
        // Find symmetries after changing exactly one character
        for i in 1..strings.len() {
            let symmetry_size = cmp::min(i, strings.len() - i);
            let strings_before = &strings[i - symmetry_size..i];
            let strings_after = &strings[i..i + symmetry_size];
            if Self::symmetric_after_fixing_smudge(&strings_before, &strings_after) {
                return Some(i);
            }
        }
        None
    }
}

impl Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            write!(f, "{row}\n")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Puzzle {
    patterns: Vec<Pattern>,
}

impl Puzzle {
    fn from_input(input: &str) -> Result<Self, Box<dyn Error>> {
        let mut patterns: Vec<Pattern> = Vec::new();
        let mut iter = input.lines();
        while let Some(pattern) = Pattern::next_from_iter(&mut iter)? {
            patterns.push(pattern);
        }
        Ok(Self { patterns })
    }
}

fn part1(input: &str) -> Result<usize, Box<dyn Error>> {
    let puzzle = Puzzle::from_input(input)?;
    let mut result = 0;
    for pattern in puzzle.patterns {
        if let Some(i) = pattern.find_symmetry_number(&pattern.rows) {
            result += 100 * i;
        }
        else if let Some(i) = pattern.find_symmetry_number(&pattern.columns) {
            result += i;
        }
        else {
            return Err("No symmetry".into());
        }
    }
    Ok(result)
}

fn part2(input: &str) -> Result<usize, Box<dyn Error>> {
    let puzzle = Puzzle::from_input(input)?;
    let mut result = 0;
    for pattern in puzzle.patterns {
        if let Some(i) = pattern.find_symmetry_number_after_fixing_smudge(&pattern.rows) {
            result += 100 * i;
        }
        else if let Some(i) = pattern.find_symmetry_number_after_fixing_smudge(&pattern.columns) {
            result += i;
        }
        else {
            return Err("No symmetry".into());
        }
    }
    Ok(result)
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
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";

    #[test]
    fn example1_part1() -> Result<(), Box<dyn Error>> {
        let result = part1(EXAMPLE1.trim())?;
        assert_eq!(result, 405);
        Ok(())
    }

    #[test]
    fn example1_part2() -> Result<(), Box<dyn Error>> {
        let result = part2(EXAMPLE1.trim())?;
        assert_eq!(result, 400);
        Ok(())
    }
}
