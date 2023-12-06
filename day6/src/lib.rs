pub mod config;

use once_cell::sync::Lazy;
use regex::Regex;
use std::error::Error;
use std::iter::zip;
use std::fs;

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,  // record distance
}

impl Race {
    fn num_ways_to_win(&self) -> u64 {
        // Solve quadratic inequality (time - x) * x > distance
        let time = self.time as f64;
        let distance = self.distance as f64;
        let sqrt = f64::sqrt(time*time - 4.0 * distance);
        let solution1 = (-time + sqrt) / -2.0;
        let solution2 = (-time - sqrt) / -2.0;
        assert!(solution1 < solution2);
        let mut at_least = solution1.ceil();
        if at_least == solution1 {
            at_least += 1.0;
        }
        let at_least = at_least as u64;
        let mut at_most = solution2.floor();
        if at_most == solution2 {
            at_most -= 1.0;
        }
        let at_most = at_most as u64;
        assert!(at_most >= at_least);
        at_most - at_least + 1
    }
}

struct Puzzle {
    races: Vec<Race>,
}

impl Puzzle {
    fn from_input(input: &str, ignore_spaces: bool) -> Result<Self, String> {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(
                r"^\s*Time:(?<times>(\s+[0-9]+)+).*\nDistance:(?<distances>(\s+[0-9]+)+)[.\n]*$"
                ).unwrap());
        let result = RE.captures(input).ok_or("Syntax error")?;
        let times: Vec<u64>;
        let distances: Vec<u64>;
        if ignore_spaces {
            let time_str = result["times"].chars().filter(|&c| c != ' ').collect::<String>();
            times = vec![time_str.parse().unwrap()];
            let distance_str = result["distances"].chars().filter(|&c| c != ' ').collect::<String>();
            distances = vec![distance_str.parse().unwrap()];
        } else {
            times = result["times"].split(' ').filter_map(|s| s.trim().parse().ok()).collect();
            distances = result["distances"].split(' ').filter_map(|s| s.trim().parse().ok()).collect();
        }
        if times.len() != distances.len() {
            return Err("Number of times different from number of distances".into());
        }
        let races = zip(times, distances).map(|(time, distance)| Race { time, distance }).collect();
        Ok(Self { races })
    }
}

 fn part1(input: &str) -> Result<u64, Box<dyn Error>> {
    let puzzle = Puzzle::from_input(input, false)?;
    let mut product = 1;
    for race in puzzle.races {
        product *= race.num_ways_to_win();
    }
    Ok(product)
}

fn part2(input: &str) -> Result<u64, Box<dyn Error>> {
    let puzzle = Puzzle::from_input(input, true)?;
    let mut product = 1;
    for race in puzzle.races {
        product *= race.num_ways_to_win();
    }
    Ok(product)
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

    const EXAMPLE: &str = "Time:      7  15   30\nDistance:  9  40  200";

    #[test]
    fn example_part1() -> Result<(), Box<dyn Error>> {
        let result = part1(EXAMPLE.trim())?;
        assert_eq!(result, 288);
        Ok(())
    }

    #[test]
    fn example_part2() -> Result<(), Box<dyn Error>> {
        let result = part2(EXAMPLE.trim())?;
        assert_eq!(result, 71503);
        Ok(())
    }
}
