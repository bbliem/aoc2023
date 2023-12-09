pub mod config;

use num::integer::lcm;
use once_cell::sync::Lazy;
use regex::Regex;
use std::cell::Cell;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use typed_arena::Arena;

enum Instruction {
    L,
    R,
}

impl TryFrom<char> for Instruction {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Instruction::L),
            'R' => Ok(Instruction::R),
            _ => Err("Unexpected instruction"),
        }
    }
}

// For some interesting reads on how to define graphs in Rust, see these:
// https://github.com/nrc/r4cppp/blob/master/graphs/README.md
// https://crates.io/crates/typed-arena
struct Node<'a> {
    label: &'a str,
    left: Cell<Option<&'a Node<'a>>>,
    right: Cell<Option<&'a Node<'a>>>,
}

impl<'a> Node<'a> {
    fn apply_instructions(&self, instructions: &Vec<Instruction>) -> &Node<'a> {
        let mut node = self;
        for instruction in instructions {
            match instruction {
                Instruction::L => node = node.left.get().unwrap(),
                Instruction::R => node = node.right.get().unwrap(),
            }
        }
        node
    }
}

struct Network<'a> {
    nodes: HashMap<&'a str, &'a Node<'a>>,
}

impl<'a> Network<'a> {
    fn from_iter(iter: impl Iterator<Item = &'a str>, arena: &'a Arena<Node<'a>>) -> Result<Self, &'a str> {
        let mut nodes: HashMap<&str, &Node<'a>> = HashMap::new();
        let mut edges: HashMap<&str, (&str, &str)> = HashMap::new();
        static NODE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(
            r"^(?<node>[A-Z0-9]{3}) = \((?<left>[A-Z0-9]{3}), (?<right>[A-Z0-9]{3})\)$"
        ).unwrap());
        for line in iter {
            let captures = NODE_RE.captures(line).ok_or("Syntax error")?;
            let label = captures.name("node").unwrap().as_str();
            let left = captures.name("left").unwrap().as_str();
            let right = captures.name("right").unwrap().as_str();
            edges.insert(label, (left, right));
            nodes.insert(label, arena.alloc(Node {
                label,
                left: Cell::new(None),
                right: Cell::new(None),
            }));
        }
        for (node, (left, right)) in edges {
            let node = nodes.get(node).unwrap();
            let left = nodes.get(left).unwrap();
            let right = nodes.get(right).unwrap();
            node.left.set(Some(left));
            node.right.set(Some(right));
        }
        Ok(Self { nodes })
    }

    fn num_applications_to_goal(
        &self, instructions: &Vec<Instruction>, node: &Node, goal_condition: impl Fn(&Node) -> bool
    ) -> usize {
        // How many times we must fully apply all instructions to reach a goal from the given node
        let mut node = node;
        let mut i = 0;
        // Apparently we can only reach the goal when applying the whole instructions a whole
        // number of times, but I'm not sure why we couldn't reach the goal after, say, applying
        // the instructions once fully and then only half of them...
        while !goal_condition(node) {
            node = node.apply_instructions(&instructions);
            i += 1;
        }
        i
    }

    fn distance_to_goal(&self, instructions: &Vec<Instruction>) -> usize {
        let applications = self.num_applications_to_goal(&instructions, self.nodes["AAA"], |node| node.label == "ZZZ");
        applications * instructions.len()
    }

    fn ghost_distance_to_goal(&self, instructions: &Vec<Instruction>) -> usize {
        let goal_condition = |node: &Node| node.label.chars().last().unwrap() == 'Z';
        // Apparently, when a ghost reaches a goal and applies the same instructions for reaching
        // it again, it will visit the exact same nodes. I'm not sure why.
        self.nodes.iter()
            .filter(|(&label, _)| label.chars().last().unwrap() == 'A')
            .map(|(_, &node)| node)
            .map(|node| self.num_applications_to_goal(instructions, node, goal_condition))
            .reduce(|acc, x| lcm(acc, x))
            .expect("Found no starting nodes")
            * instructions.len()
    }
}

struct Puzzle<'a> {
    instructions: Vec<Instruction>,
    network: Network<'a>,
}

impl<'a> Puzzle<'a> {
    fn from_input(input: &'a str, arena: &'a Arena<Node<'a>>) -> Result<Self, Box<dyn Error>> {
        let mut iter = input.lines();
        let instructions: Vec<Instruction> = iter.next().ok_or("Expected instructions")?
            .chars().map(|c| Instruction::try_from(c))
            .collect::<Result<_, _>>()?;
        if !iter.next().ok_or("Expected separating line")?.is_empty() {
            return Err("Separating line not empty".into());
        }
        let network = Network::from_iter(iter, &arena)?;
        Ok(Self { instructions, network })
    }

    fn distance_to_goal(&self) -> usize {
        self.network.distance_to_goal(&self.instructions)
    }

    fn ghost_distance_to_goal(&self) -> usize {
        self.network.ghost_distance_to_goal(&self.instructions)
    }
}

fn part1(input: &str) -> Result<usize, Box<dyn Error>> {
    let arena = Arena::new();
    let puzzle = Puzzle::from_input(input, &arena)?;
    Ok(puzzle.distance_to_goal())
}

fn part2(input: &str) -> Result<usize, Box<dyn Error>> {
    let arena = Arena::new();
    let puzzle = Puzzle::from_input(input, &arena)?;
    Ok(puzzle.ghost_distance_to_goal())
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
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

    const EXAMPLE2: &str = "
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

    const EXAMPLE3: &str = "
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

    #[test]
    fn example1_part1() -> Result<(), Box<dyn Error>> {
        let result = part1(EXAMPLE1.trim())?;
        assert_eq!(result, 2);
        Ok(())
    }

    #[test]
    fn example2_part1() -> Result<(), Box<dyn Error>> {
        let result = part1(EXAMPLE2.trim())?;
        assert_eq!(result, 6);
        Ok(())
    }

    #[test]
    fn example3_part2() -> Result<(), Box<dyn Error>> {
        let result = part2(EXAMPLE3.trim())?;
        assert_eq!(result, 6);
        Ok(())
    }
}
