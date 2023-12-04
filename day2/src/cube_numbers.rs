use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug)]
#[derive(Default)]
pub struct CubeNumbers {
    red: u32,
    green: u32,
    blue: u32,
}

impl CubeNumbers {
    pub fn new(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }

    pub fn from_str(s: &str, line_nr: usize) -> Result<Self, String> {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(?<n>[0-9]+) (?<color>[a-z]+)$").unwrap());
        let mut result = Self { ..Default::default() };
        for part in s.split(", ") {
            let Some(groups) = RE.captures(part) else {
                return Err(format!("Could not parse '{part}' on line {line_nr}"));
            };
            let n: u32 = groups["n"].parse().unwrap();
            let color = &groups["color"];
            match color {
                "red" => { result.red = n },
                "green" => { result.green = n },
                "blue" => { result.blue = n },
                _ => return Err(format!("Invalid color '{color}' on line {line_nr}")),
            }
        }
        Ok(result)
    }

    pub fn at_most(&self, other: &Self) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }

    pub fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }

    pub fn make_fit(&mut self, other: &Self) {
        if self.red < other.red {
            self.red = other.red;
        }
        if self.green < other.green {
            self.green = other.green;
        }
        if self.blue < other.blue {
            self.blue = other.blue;
        }
    }
}
