pub mod config;

use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::str;
use std::str::FromStr;

use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Eq, PartialEq)]
enum Category { X, M, A, S }

impl FromStr for Category {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Category::X),
            "m" => Ok(Category::M),
            "a" => Ok(Category::A),
            "s" => Ok(Category::S),
            _ => Err("Invalid category"),
        }
    }
}

#[derive(Eq, PartialEq)]
enum Rule {
    Greater { lhs: Category, rhs: i32, next: String },
    Less { lhs: Category, rhs: i32, next: String },
    Jump(String),
}

impl TryFrom<&str> for Rule {
    type Error = &'static str;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        if string == "A" || string == "R" { return Ok(Rule::Jump(string.to_owned())) }
        static RULE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(
            r"^(?<category>[a-z]+)(?<op>[<>])(?<rhs>[0-9]+):(?<next>[a-z]+|[AR])$"
        ).unwrap());
        if let Some(captures) = RULE_RE.captures(string) {
            let lhs = Category::from_str(captures.name("category").unwrap().as_str())?;
            let op = captures.name("op").unwrap().as_str();
            let rhs = captures.name("rhs").unwrap().as_str().parse().map_err(|_| "Could not parse rhs")?;
            let next = captures.name("next").unwrap().as_str().to_owned();
            match op {
                "<" => Ok(Rule::Less { lhs, rhs, next }),
                ">" => Ok(Rule::Greater { lhs, rhs, next }),
                _ => Err("Invalid rule".into()),
            }
        } else {
            Ok(Rule::Jump(string.to_owned()))
        }
    }
}

struct Part {
    ratings: [i32; 4],
}

impl Part {
    fn sum_ratings(&self) -> i32 {
        self.ratings.iter().sum()
    }

    fn get_rating(&self, category: &Category) -> i32 {
        match category {
            Category::X => self.ratings[0],
            Category::M => self.ratings[1],
            Category::A => self.ratings[2],
            Category::S => self.ratings[3],
        }
    }
}

struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

type WorkflowMap = HashMap<String, Workflow>;

struct Puzzle {
    workflows: WorkflowMap,
    parts: Vec<Part>,
}

impl Puzzle {
    fn from_input(input: &str) -> Result<Self, Box<dyn Error>> {
        let mut lines = input.lines();
        let workflows = Self::read_workflows(&mut lines)?;
        let parts = Self::read_parts(&mut lines)?;
        Ok(Self { workflows, parts })
    }

    fn read_workflows(lines: &mut str::Lines<'_>) -> Result<WorkflowMap, Box<dyn Error>> {
        let mut workflows = WorkflowMap::new();
        for line in lines {
            if line.is_empty() { break }
            let (name, rest) = line.split_once('{').ok_or("Invalid workflow")?;
            let mut chars = rest.chars();
            if chars.next_back() != Some('}') { return Err("Invalid workflow".into()) }
            let rules = Self::read_rules(chars.as_str())?;
            let name = name.to_owned();
            workflows.insert(name.clone(), Workflow { name, rules });
        }
        if !workflows.contains_key("in") { return Err("No workflow named 'in'".into()) }
        workflows.insert(String::from("A"), Workflow { name: String::from("A"), rules: Vec::new() });
        workflows.insert(String::from("R"), Workflow { name: String::from("R"), rules: Vec::new() });
        Ok(workflows)
    }

    fn read_rules(string: &str) -> Result<Vec<Rule>, &str> {
        string.split(',').map(|r| Rule::try_from(r)).collect::<Result<_,_>>()
    }

    fn read_parts(lines: &mut str::Lines<'_>) -> Result<Vec<Part>, Box<dyn Error>> {
        let mut parts = Vec::new();
        for line in lines {
            static PART_RE: Lazy<Regex> = Lazy::new(|| Regex::new(
                r"^\{x=([0-9]+),m=([0-9]+),a=([0-9]+),s=([0-9]+)\}$"
            ).unwrap());
            let captures = PART_RE.captures(line).ok_or("Failed to parse part")?;
            let ratings: [i32; 4] = (1..=4)
                .map(|i| captures.get(i).unwrap().as_str().parse())
                .collect::<Result<Vec<_>,_>>()?
                .try_into().unwrap();
            parts.push(Part { ratings });
        }
        Ok(parts)
    }

    fn part_accepted(&self, part: &Part) -> bool {
        let mut workflow = self.workflows.get("in").unwrap();
        // FIXME: Code duplication
        loop {
            if workflow.name == "A" { return true }
            if workflow.name == "R" { return false }
            for rule in &workflow.rules {
                match rule {
                    Rule::Jump(next) => {
                        workflow = self.workflows.get(next).unwrap_or_else(|| panic!("Unknown workflow {next}"));
                        break;
                    }
                    Rule::Greater { lhs, rhs, next } => {
                        if part.get_rating(lhs) > *rhs {
                            workflow = self.workflows.get(next).unwrap_or_else(|| panic!("Unknown workflow {next}"));
                            break;
                        }
                    },
                    Rule::Less { lhs, rhs, next } => {
                        if part.get_rating(lhs) < *rhs {
                            workflow = self.workflows.get(next).unwrap_or_else(|| panic!("Unknown workflow {next}"));
                            break;
                        }
                    },
                }
            }
        }
    }

    fn sum_accepted(&self) -> i32 {
        self.parts.iter().filter(|p| self.part_accepted(p)).map(|p| p.sum_ratings()).sum()
    }

    fn possibilities(&self) -> u64 {
        let start = self.workflows.get("in").expect("No 'in' workflow");
        let mut stack = vec![(start, [1u64, 4000u64], [1u64, 4000u64], [1u64, 4000u64], [1u64, 4000u64])];
        let mut sum = 0;
        while let Some((workflow, mut x, mut m, mut a, mut s)) = stack.pop() {
            if workflow.name == "R" { continue }
            if workflow.name == "A" {
                sum += (x[1] - x[0] + 1) * (m[1] - m[0] + 1) * (a[1] - a[0] + 1) * (s[1] - s[0] + 1);
                continue;
            }
            for rule in &workflow.rules {
                match rule {
                    Rule::Jump(next) => {
                        stack.push((self.workflows.get(next).expect("Unknown workflow"), x, m, a, s));
                    },
                    // FIXME: Code duplication
                    Rule::Greater { lhs, rhs, next } => {
                        let rhs = *rhs as u64;
                        let mut xn = x;
                        let mut mn = m;
                        let mut an = a;
                        let mut sn = s;
                        match lhs {
                            Category::X => {
                                xn[0] = rhs + 1;
                                x[1] = rhs;
                            },
                            Category::M => {
                                mn[0] = rhs + 1;
                                m[1] = rhs;
                            },
                            Category::A => {
                                an[0] = rhs + 1;
                                a[1] = rhs;
                            },
                            Category::S => {
                                sn[0] = rhs + 1;
                                s[1] = rhs;
                            },
                        }
                        if xn[1] >= xn[0] && mn[1] >= mn[0] && an[1] >= an[0] && sn[1] >= sn[0] {
                            stack.push((self.workflows.get(next).expect("Unknown workflow"), xn, mn, an, sn))
                        }
                    },
                    Rule::Less { lhs, rhs, next } => {
                        let rhs = *rhs as u64;
                        let mut xn = x;
                        let mut mn = m;
                        let mut an = a;
                        let mut sn = s;
                        match lhs {
                            Category::X => {
                                xn[1] = rhs - 1;
                                x[0] = rhs;
                            },
                            Category::M => {
                                mn[1] = rhs - 1;
                                m[0] = rhs;
                            },
                            Category::A => {
                                an[1] = rhs - 1;
                                a[0] = rhs;
                            },
                            Category::S => {
                                sn[1] = rhs - 1;
                                s[0] = rhs;
                            },
                        }
                        if xn[1] >= xn[0] && mn[1] >= mn[0] && an[1] >= an[0] && sn[1] >= sn[0] {
                            stack.push((self.workflows.get(next).expect("Unknown workflow"), xn, mn, an, sn))
                        }
                    },
                }
            }
        }
        sum
    }
}

fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let puzzle = Puzzle::from_input(input)?;
    Ok(puzzle.sum_accepted())
}

fn part2(input: &str) -> Result<u64, Box<dyn Error>> {
    let puzzle = Puzzle::from_input(input)?;
    Ok(puzzle.possibilities())
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
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";

    #[test]
    fn example1_part1() -> Result<(), Box<dyn Error>> {
        let result = part1(EXAMPLE1.trim())?;
        assert_eq!(result, 19114);
        Ok(())
    }

    #[test]
    fn example1_part2() -> Result<(), Box<dyn Error>> {
        let result = part2(EXAMPLE1.trim())?;
        assert_eq!(result, 167409079868000);
        Ok(())
    }
}
