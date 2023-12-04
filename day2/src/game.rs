use once_cell::sync::Lazy;
use regex::Regex;

use crate::cube_numbers::CubeNumbers;

#[derive(Debug)]
pub struct Game {
    id: u32,
    sets: Vec<CubeNumbers>,
}

impl Game {
    pub fn from_line(line: &str, line_nr: usize) -> Result<Self, String> {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^Game (?<id>[0-9]+): (?<sets>.*)$").unwrap());
        let Some(result) = RE.captures(line) else {
            return Err(format!("Syntax error on line {line_nr}"));
        };
        let id: u32 = result["id"].parse().unwrap();
        let sets_str = &result["sets"];
        let mut sets = vec![];
        for set_str in sets_str.split("; ") {
            let set = CubeNumbers::from_str(set_str, line_nr)?;
            sets.push(set);
        }
        Ok(Self {
            id,
            sets,
        })
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn is_possible(&self, bag: &CubeNumbers) -> bool {
        self.sets.iter().all(|set| set.at_most(bag))
    }

    pub fn smallest_fitting_set(&self) -> CubeNumbers {
        let mut fitting_set = CubeNumbers::new(0, 0, 0);
        for set in &self.sets {
            fitting_set.make_fit(set);
        }
        fitting_set
    }
}
