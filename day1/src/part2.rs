use std::error::Error;

const DIGITS: [(&str, u32); 20] = [
    ("0", 0),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("zero", 0),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn find_first_and_last_digits(line: &str, line_nr: usize) -> Result<(u32, u32), String> {
    let mut first_digit_index: Option<usize> = None;
    let mut last_digit_index: Option<usize> = None;
    let mut first_digit: Option<u32> = None;
    let mut last_digit: Option<u32> = None;
    for (pattern, digit) in DIGITS {
        // Find first occurrence of this digit
        match line.find(pattern) {
            Some(i) => {
                if first_digit_index.map_or(true, |old_i| old_i > i) {
                    first_digit_index = Some(i);
                    first_digit = Some(digit);
                }
            },
            None => (),
        }

        // Find last occurrence of this digit
        match line.rfind(pattern) {
            Some(i) => {
                if last_digit_index.map_or(true, |old_i| old_i < i) {
                    last_digit_index = Some(i);
                    last_digit = Some(digit);
                }
            },
            None => (),
        }
    }
    match (first_digit, last_digit) {
        (Some(fd), Some(ld)) => Ok((fd, ld)),
        _ => Err(format!("Line {} does not contain a digit (spelled out or not)", line_nr)),
    }
}

pub fn run(contents: &String) -> Result<(), Box<dyn Error>> {
    let mut sum = 0;
    for (i, line) in contents.lines().enumerate() {
        let (fd, ld) = find_first_and_last_digits(line, i+1)?;
        sum += 10 * fd + ld;
    }
    println!("Sum for part 2: {sum}");
    Ok(())
}
