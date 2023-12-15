pub mod config;

use std::error::Error;
use std::{fs, array};

fn hash(string: &str) -> u8 {
    string.chars().fold(0, |value, c| 17u8.wrapping_mul(value.wrapping_add(c as u8)))
}

struct MapEntry<'a>(&'a str, u8);

struct HashMap<'a> {
    buckets: [Vec<MapEntry<'a>>; 256],
}

impl<'a> HashMap<'a> {
    fn new() -> Self {
        Self { buckets: array::from_fn(|_| Vec::new()) }
    }

    fn set(&mut self, key: &'a str, value: u8) {
        let bucket = &mut self.buckets[hash(&key) as usize];
        for MapEntry(k, v) in &mut *bucket {
            if *k == key {
                *v = value;
                return;
            }
        }
        bucket.push(MapEntry(key, value));
    }

    fn remove(&mut self, key: &str) {
        let bucket = &mut self.buckets[hash(&key) as usize];
        bucket.retain(|MapEntry(k, _)| *k != key)
    }

    fn focusing_power(&self) -> u32 {
        let mut sum = 0;
        for (i, bucket) in self.buckets.iter().enumerate() {
            for (j, MapEntry(_, v)) in bucket.iter().enumerate() {
                sum += (i+1) * (j+1) * (*v as usize);
            }
        }
        sum as u32
    }
}

fn part1(input: &str) -> Result<u32, Box<dyn Error>> {
    Ok(input.trim().split(',').map(|s| hash(s) as u32).sum())
}

fn part2(input: &str) -> Result<u32, Box<dyn Error>> {
    let steps = input.trim().split(',');
    let mut map = HashMap::new();
    for step in steps {
        if let Some((key, value)) = step.split_once('=') {
            map.set(key, value.parse()?);
        }
        else if let Some((key, _)) = step.split_once('-') {
            map.remove(key);
        }
        else {
            return Err("Invalid step".into());
        }
    }
    Ok(map.focusing_power())
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
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
";

    #[test]
    fn example1_part1() -> Result<(), Box<dyn Error>> {
        let result = part1(EXAMPLE1.trim())?;
        assert_eq!(result, 1320);
        Ok(())
    }

    #[test]
    fn example1_part2() -> Result<(), Box<dyn Error>> {
        let result = part2(EXAMPLE1.trim())?;
        assert_eq!(result, 145);
        Ok(())
    }
}
