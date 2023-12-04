pub mod config;

use std::error::Error;
use std::fs;

fn is_symbol(c: char) -> bool {
    !(c.is_ascii_digit() || c == '.')
}

fn symbol_at_nth_char(n: usize, line: &str) -> Option<char> {
    return line.chars().nth(n).filter(|&c| is_symbol(c));
}

#[derive(Debug)]
struct Number {
    value: u32,
    // coordinates of first character
    x: usize,
    y: usize,
    // number of digits
    len: usize,
}

impl Number {
    fn adjacent_symbol(&self, lines: &[&str]) -> Option<char> {
        if self.y > 0 {
            let symbol = lines.get(self.y - 1).and_then(|line_before| self.adjacent_symbol_in_line(line_before));
            if let Some(s) = symbol {
                return Some(s);
            }
        }
        if let Some(same_line) = lines.get(self.y) {
            if self.x > 0 {
                if let Some(s) = symbol_at_nth_char(self.x - 1, same_line) {
                    return Some(s);
                }
            }
            if let Some(s) = symbol_at_nth_char(self.x + self.len, same_line) {
                return Some(s);
            }
        }
        let symbol = lines.get(self.y + 1).and_then(|line_after| self.adjacent_symbol_in_line(line_after));
        if let Some(s) = symbol {
            return Some(s);
        }
        None
    }

    fn adjacent_symbol_in_line(&self, line: &str) -> Option<char> {
        let x_minus_one = self.x.saturating_sub(1);
        let x_plus_len_plus_one = self.x + self.len + 1;
        let end_index = x_plus_len_plus_one.min(line.len());
        let slice = &line[x_minus_one..end_index];
        slice.chars().find(|&c| is_symbol(c))
    }

    fn is_adjacent_to(&self, x: usize, y: usize) -> bool {
        let in_box_x = x >= self.x.saturating_sub(1) && x <= self.x + self.len;
        let in_box_y = y >= self.y.saturating_sub(1) && y <= self.y + 1;
        let in_number_x = x >= self.x && x < self.x + self.len;
        let in_number_y = y == self.y;
        in_box_x && in_box_y && !(in_number_x && in_number_y)
    }
}

#[derive(Debug)]
struct Gear {
    x: usize,
    y: usize,
}

impl Gear {
    fn adjacent_numbers<'a>(&self, numbers: &'a [Number]) -> Vec<&'a Number> {
        let mut result: Vec<&Number> = Vec::new();
        for number in numbers {
            if number.is_adjacent_to(self.x, self.y) {
                result.push(&number);
            }
        }
        result
    }
}

fn build_numbers(lines: &[&str]) -> Vec<Number> {
    let mut numbers: Vec<Number> = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        let mut cur_number: Option<Number> = None;
        for (x, c) in line.chars().enumerate() {
            if let Some(digit) = c.to_digit(10) {
                let n = cur_number.get_or_insert(Number { value: 0, x, y, len: 0 });
                n.len += 1;
                n.value = 10 * n.value + digit;
            } else {
                // Commit cur_number to vector and set to None
                if let Some(n) = cur_number.take() {
                    numbers.push(n);
                }
            }
            // println!("({x},{y}): {c}");
        }
        // Commit number at end of line even if no characters after it are read
        if let Some(n) = cur_number.take() {
            numbers.push(n);
        }
    }
    numbers
}

fn build_gears(lines: &[&str]) -> Vec<Gear> {
    // TODO: Could be done in one read together with build_numbers
    let mut gears: Vec<Gear> = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '*' {
                gears.push(Gear { x, y })
            }
        }
    }
    gears
}

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let lines: Vec<&str> = input.lines().collect();
    let numbers = build_numbers(&lines);
    let mut sum = 0;
    for number in numbers {
        if number.adjacent_symbol(&lines).is_some() {
            sum += number.value;
        }
    }
    println!("Sum for part 1: {sum}");
    Ok(())
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let lines: Vec<&str> = input.lines().collect();
    let numbers = build_numbers(&lines);
    let gears = build_gears(&lines);
    let mut sum = 0;
    for gear in gears {
        let numbers = gear.adjacent_numbers(&numbers);
        if numbers.len() == 2 {
            sum += numbers[0].value * numbers[1].value;
        }
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
