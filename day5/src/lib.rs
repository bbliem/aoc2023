pub mod config;

use std::error::Error;
use std::fs;

#[derive(Debug)]
struct MapEntry {
    source_start: u32,
    dest_start: u32,
    range_length: u32,
}

impl MapEntry {
    fn from_line(line: &str) -> Result<Self, String> {
        let mut iter = line.split(' ');
        let dest_start = iter.next().ok_or("Could not read destination range start")?
            .parse().map_err(|_| "Could not parse destination range start")?;
        let source_start = iter.next().ok_or("Could not read source range start")?
            .parse().map_err(|_| "Could not parse source range start")?;
        let range_length = iter.next().ok_or("Could not read range length")?
            .parse().map_err(|_| "Could not parse range length")?;
        Ok(MapEntry {source_start, dest_start, range_length})
    }

    fn apply(&self, x: u32) -> Option<u32> {
        if x >= self.source_start && x - self.source_start < self.range_length {
            Some(self.dest_start + (x - self.source_start))
        } else {
            None
        }
    }

    fn source_overlaps(&self, other: &MapEntry) -> bool {
        (self.source_start >= other.source_start && self.source_start < other.source_start + other.range_length)
        || (other.source_start >= self.source_start && other.source_start < self.source_start + self.range_length)
    }
}

#[derive(Debug)]
struct Map<'a> {
    from_type: &'a str,
    to_type: &'a str,
    entries: Vec<MapEntry>,
}

impl<'a> Map<'a> {
    fn next_from_iter(iter: &mut impl Iterator<Item = &'a str>) -> Result<Option<Self>, String> {
        let from_type;
        let to_type;
        if let Some(header) = iter.next() {
            // expect map name of the form "<from_type>-to-<to_type> map:"
            let mut header_iter = header.split(' ');
            let map_name = header_iter.next().ok_or(format!("Expected map name in header {header}"))?;
            let error = format!("Expected static string 'map:' in header {header}");
            if header_iter.next().ok_or(error.clone())? != "map:" {
                return Err(error);
            }
            let mut from_to_iter = map_name.split("-to-");
            from_type = from_to_iter.next().ok_or(format!("Could not read from_type in header {header}"))?;
            to_type = from_to_iter.next().ok_or(format!("Could not read to_type in header {header}"))?;
        } else {
            return Ok(None)
        }

        let entries = Self::entries_from_iter_until_end_of_block(iter)?;
        Ok(Some(Map {from_type, to_type, entries}))
    }

    fn entries_from_iter_until_end_of_block<'b>(iter: &mut impl Iterator<Item = &'b str>) -> Result<Vec<MapEntry>, String> {
        let mut entries = vec![];
        // Read until empty line or EOF
        loop {
            if let Some(line) = iter.next() {
                if line.is_empty() {
                    break;
                }
                let entry = MapEntry::from_line(line)?;
                if entries.iter().any(|e| entry.source_overlaps(e)) {
                    println!("Overlap");
                    return Err(String::from("Overlapping sources"));
                }
                entries.push(entry);
            } else {
                break
            }
        }
        Ok(entries)
    }

    fn apply(&self, x: u32) -> u32 {
        for entry in &self.entries {
            if let Some(y) = entry.apply(x) {
                return y;
            }
        }
        x
    }
}

#[derive(Debug)]
struct Puzzle<'a> {
    seeds: Vec<u32>,
    maps: Vec<Map<'a>>,
}

impl<'a> Puzzle<'a> {
    fn from_input(input: &'a str) -> Result<Self, Box<dyn Error>> {
        let mut iter = input.lines();

        // Read seeds
        let seeds_line = iter.next().ok_or("Expected seeds line")?;
        let error = "Invalid seeds line";
        let mut seeds_line_iter = seeds_line.split(' ');
        if seeds_line_iter.next().ok_or(error)? != "seeds:" {
            return Err(error.into());
        }
        let seeds = seeds_line_iter.map(|s| s.parse()).collect::<Result<_, _>>()?;

        let error = "Expected empty line";
        if !iter.next().ok_or(error)?.is_empty() {
            let x = error.into();
            return Err(x);
        }

        // Read maps
        let mut maps: Vec<Map> = vec![];
        while let Some(map) = Map::next_from_iter(&mut iter)? {
            maps.push(map);
        }

        Ok(Self { seeds, maps })
    }

    fn seeds_to_ranges(&mut self) -> Result<(), Box<dyn Error>> {
        let mut result = vec![];
        let mut seed_iter = self.seeds.iter();
        while let Some(start) = seed_iter.next() {
            let range = seed_iter.next().ok_or("Expected range")?;
            for i in *start..=*start+*range {
                result.push(i)
            }
        }
        self.seeds = result;
        Ok(())
    }

    fn min_for_seeds(&self) -> Result<u32, Box<dyn Error>> {
        // Apply all maps in turn to the seeds and remember minimum of the results
        let mut maybe_min: Option<u32> = None;
        for seed in &self.seeds {
            let mut value = *seed;
            let mut value_type = "seed";
            for map in &self.maps {
                // Map must have the right type as input
                if map.from_type != value_type {
                    return Err("Map has from_type {map.from_type}, but expected {type}".into());
                }
                value_type = map.to_type;
                value = map.apply(value);
            }
            maybe_min = match maybe_min {
                None => Some(value),
                Some(m) => Some(m.min(value)),
            }
        }
        Ok(maybe_min.ok_or("Expected at least one seed")?)
    }
}

 fn part1(input: &str) -> Result<u32, Box<dyn Error>> {
    let puzzle = Puzzle::from_input(input)?;
    Ok(puzzle.min_for_seeds()?)
}

fn part2(input: &str) -> Result<u32, Box<dyn Error>> {
    let mut puzzle = Puzzle::from_input(input)?;
    puzzle.seeds_to_ranges()?;
    // FIXME: This is slow as hell.
    Ok(puzzle.min_for_seeds()?)
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
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
    ";

    #[test]
    fn example_part1() -> Result<(), Box<dyn Error>> {
        let result = part1(EXAMPLE.trim())?;
        assert_eq!(result, 35);
        Ok(())
    }

    #[test]
    fn example_part2() -> Result<(), Box<dyn Error>> {
        let result = part2(EXAMPLE.trim())?;
        assert_eq!(result, 46);
        Ok(())
    }

    #[test]
    #[should_panic(expected="Overlapping sources")]
    fn map_entries_overlapping_sources() {
        let input = concat!(
            "2 1 1\n", // 1 -> 2
            "1 2 1\n", // 2 -> 1
            "10 0 2\n", // 0 -> 10, 1 -> 11 but 1 is already mapped to 2
        );
        let mut iter = input.lines();
        Map::entries_from_iter_until_end_of_block(&mut iter).unwrap();
    }
}
