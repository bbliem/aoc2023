use std::error::Error;

fn nth_char_to_digit(s: &str, n: usize) -> u32 {
    s.chars().nth(n).unwrap().to_digit(10).unwrap()
}

fn find_first_and_last_digits(line: &str, line_nr: usize) -> Result<(u32, u32), String> {
    let first_digit = line.find(char::is_numeric).map(|i| nth_char_to_digit(line, i));
    let last_digit = line.rfind(char::is_numeric).map(|i| nth_char_to_digit(line, i));
    match (first_digit, last_digit) {
        (Some(fd), Some(ld)) => Ok((fd, ld)),
        _ => Err(format!("Line {line_nr} does not contain a digit")),
    }
}

pub fn run(contents: &String) -> Result<(), Box<dyn Error>> {
    let mut sum = 0;
    for (i, line) in contents.lines().enumerate() {
        let (fd, ld) = find_first_and_last_digits(line, i+1)?;
        sum += 10 * fd + ld;
    }
    println!("Sum for part 1: {sum}");
    Ok(())
}
