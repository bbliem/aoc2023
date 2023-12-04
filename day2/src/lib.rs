pub mod config;

mod cube_numbers;
mod game;

use std::error::Error;
use std::fs;

use cube_numbers::CubeNumbers;
use game::Game;

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let bag_contents = CubeNumbers::new(12, 13, 14);
    let mut sum = 0;
    for (i, line) in input.lines().enumerate() {
        let game = Game::from_line(&line, i)?;
        if game.is_possible(&bag_contents) {
            sum += game.id();
        }
    }
    println!("Sum for part 1: {sum}");
    Ok(())
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let mut sum = 0;
    for (i, line) in input.lines().enumerate() {
        let game = Game::from_line(&line, i)?;
        let fitting_set = game.smallest_fitting_set();
        sum += fitting_set.power();
    }
    println!("Sum for part 2: {sum}");
    Ok(())
}

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    println!("Part 1: Reading file {}", config.file_path1);
    let contents = fs::read_to_string(config.file_path1)?;
    part1(&contents)?;

    println!("Part 2: Reading file {}", config.file_path2);
    let contents = fs::read_to_string(config.file_path2)?;
    part2(&contents)?;

    Ok(())
}
