pub mod config;

use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

#[derive(Debug)]
struct Card {
    id: u32,
    winning_numbers: Vec<i32>,
    own_numbers: Vec<i32>,
}

impl Card {
    pub fn from_line(line: &str, line_nr: usize) -> Result<Self, String> {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(
                r"^Card +(?<id>[0-9]+): (?<winning_numbers>[0-9 ]+) \| (?<own_numbers>[0-9 ]+)$"
                ).unwrap());
        let Some(result) = RE.captures(line) else {
            return Err(format!("Syntax error on line {line_nr}"));
        };
        let id: u32 = result["id"].parse().map_err(|err| format!("Could not parse ID on line {line_nr}: {err}"))?;
        let winning_numbers = Self::parse_whitespace_separated_numbers(&result["winning_numbers"])
            .map_err(|err| format!("Could not parse winning numbers on line {line_nr}: {err}"))?;
        let own_numbers = Self::parse_whitespace_separated_numbers(&result["own_numbers"])
            .map_err(|err| format!("Could not parse own numbers on line {line_nr}: {err}"))?;
        Ok(Card {id, winning_numbers, own_numbers})
    }

    fn num_winning_numbers(&self) -> usize {
        let winning_set: HashSet<i32> = self.winning_numbers.iter().cloned().collect();
        let own_set: HashSet<i32> = self.own_numbers.iter().cloned().collect();
        own_set.intersection(&winning_set).count()
    }

    fn points(&self) -> i32 {
        let n = self.num_winning_numbers();
        if n > 0 {
            1 << (n - 1)
        } else {
            0
        }
    }

    fn parse_whitespace_separated_numbers(string: &str) -> Result<Vec<i32>, String> {
        let mut numbers = vec![];
        for s in string.split(' ').map(|s| s.trim()).filter(|s| !s.is_empty()) {
            if let Ok(number) = s.parse::<i32>() {
                numbers.push(number);
            } else {
                return Err(format!("Could not parse number '{s}'"));
            }
        }
        Ok(numbers)
    }
}

#[derive(Debug)]
pub struct Pile {
    cards: Vec<Card>,
}

impl Pile {
    fn from_input(input: &str) -> Result<Self, String> {
        let mut cards = vec![];
        for (i, line) in input.lines().map(|line| line.trim()).filter(|line| !line.is_empty()).enumerate() {
            let card = Card::from_line(line, i)?;
            cards.push(card);
        }
        Ok(Pile {cards})
    }

    fn points(&self) -> i32 {
        let mut sum = 0;
        for card in &self.cards {
            sum += card.points()
        }
        sum
    }

    fn num_cards_after_copying(&self) -> usize {
        let mut num_cards_with_id = self.cards.iter().map(|card| (card.id, 1)).collect::<HashMap<u32, usize>>();
        let mut num_cards_total = self.cards.len();
        for card in &self.cards {
            let num_cards_with_this_id = num_cards_with_id.get(&card.id).unwrap().to_owned();
            let num_winning_numbers = u32::try_from(card.num_winning_numbers()).unwrap();
            for i in card.id + 1 .. card.id + 1 + num_winning_numbers {
                if let Some(num_cards) = num_cards_with_id.get_mut(&i) {
                    *num_cards += num_cards_with_this_id;
                    num_cards_total += num_cards_with_this_id;
                }
            }
        }
        num_cards_total
    }
}

 fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let pile = Pile::from_input(input)?;
    Ok(pile.points())
}

fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    let pile = Pile::from_input(input)?;
    Ok(i32::try_from(pile.num_cards_after_copying())?)
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
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    ";

    #[test]
    fn example_part1() -> Result<(), Box<dyn Error>> {
        let result = part1(EXAMPLE)?;
        assert_eq!(result, 13);
        Ok(())
    }

    #[test]
    fn example_part2() -> Result<(), Box<dyn Error>> {
        let result = part2(EXAMPLE)?;
        assert_eq!(result, 30);
        Ok(())
    }
}
