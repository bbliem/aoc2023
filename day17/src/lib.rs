pub mod config;

use core::panic;
use std::collections::BinaryHeap;
use std::error::Error;
use std::fs;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    x: usize,
    y: usize,
    cost: usize,
    vertical: bool, // whether node (x, y) was entered vertically
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.x.cmp(&other.x))
            .then_with(|| self.y.cmp(&other.y))
            .then_with(|| self.vertical.cmp(&other.vertical))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Edge {
    x: usize,
    y: usize,
    cost: usize,
}

struct Puzzle {
    rows: Vec<Vec<u8>>,
    w: usize,
    h: usize,
    min_move: usize,
    max_move: usize,
}

impl Puzzle {
    fn from_input(input: &str, min_move: usize, max_move: usize) -> Result<Self, Box<dyn Error>> {
        let mut rows = Vec::new();
        let line_len = input.lines().next().ok_or("Empty input")?.len();
        for line in input.lines() {
            if line.len() != line_len {
                return Err("Not all lines have the same length".into());
            }
            let row = line.chars()
                .map(|c| c.to_digit(10)).collect::<Option<Vec<_>>>().ok_or("Could not parse digit")?.into_iter()
                .map(|i| i as u8).collect();
            rows.push(row);
        }
        Ok(Self { w: line_len, h: rows.len(), rows, min_move, max_move })
    }

    fn edges_h(&self, x: usize, y: usize) -> Vec<Edge> {
        let mut edges = Vec::new();
        let mut cost = 0;
        for distance in 1..=self.max_move {
            if distance > x {
                break;
            }
            let x = x - distance;
            cost += self.rows[y][x] as usize;
            if distance >= self.min_move {
                edges.push(Edge { cost, x, y });
            }
        }
        cost = 0;
        for distance in 1..=self.max_move {
            let x = x + distance;
            if x >= self.w {
                break;
            }
            cost += self.rows[y][x] as usize;
            if distance >= self.min_move {
                edges.push(Edge { cost, x, y});
            }
        }
        edges
    }

    fn edges_v(&self, x: usize, y: usize) -> Vec<Edge> {
        let mut edges = Vec::new();
        let mut cost = 0;
        for distance in 1..=self.max_move {
            if distance > y {
                break;
            }
            let y = y - distance;
            cost += self.rows[y][x] as usize;
            if distance >= self.min_move {
                edges.push(Edge { cost, x, y });
            }
        }
        cost = 0;
        for distance in 1..=self.max_move {
            let y = y + distance;
            if y >= self.h {
                break;
            }
            cost += self.rows[y][x] as usize;
            if distance >= self.min_move {
                edges.push(Edge { cost, x, y});
            }
        }
        edges
    }

    fn shortest_path(&self) -> usize {
        // dist_h[y][x] is the minimum distance to get to node at (x, y) that enters the node
        // horizontally. dist_v[y][x] is analogous.
        let mut dist_h: Vec<Vec<usize>> = (0..self.rows.len()).map(|_| (0..self.w).map(|_| usize::MAX).collect()).collect();
        let mut dist_v: Vec<Vec<usize>> = (0..self.rows.len()).map(|_| (0..self.w).map(|_| usize::MAX).collect()).collect();
        let mut heap = BinaryHeap::new();
        dist_h[0][0] = 0;
        dist_v[0][0] = 0;
        heap.push(State { cost: 0, x: 0, y: 0, vertical: false });
        heap.push(State { cost: 0, x: 0, y: 0, vertical: true });
        while let Some(State { cost, x, y, vertical }) = heap.pop() {
            if x == self.w - 1 && y == self.h - 1 {
                return cost;
            }
            if vertical {
                // Move horizontally now
                if cost <= dist_v[y][x] {
                    for edge in self.edges_h(x, y) {
                        let next = State { cost: cost + edge.cost, x: edge.x, y: edge.y, vertical: false};
                        if next.cost < dist_h[next.y][next.x] {
                            heap.push(next);
                            dist_h[next.y][next.x] = next.cost;
                        }
                    }
                }
            } else {
                // Move vertically now
                if cost <= dist_h[y][x] {
                    for edge in self.edges_v(x, y) {
                        let next = State { cost: cost + edge.cost, x: edge.x, y: edge.y, vertical: true};
                        if next.cost < dist_v[next.y][next.x] {
                            heap.push(next);
                            dist_v[next.y][next.x] = next.cost;
                        }
                    }
                }
            }
        }
        panic!("Goal unreachable");
    }
}

fn part1(input: &str) -> Result<usize, Box<dyn Error>> {
    let puzzle = Puzzle::from_input(input, 1, 3)?;
    Ok(puzzle.shortest_path())
}

fn part2(input: &str) -> Result<usize, Box<dyn Error>> {
    let puzzle = Puzzle::from_input(input, 4, 10)?;
    Ok(puzzle.shortest_path())
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
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";

    const EXAMPLE2: &str = "
111111111111
999999999991
999999999991
999999999991
999999999991
";

    #[test]
    fn example1_part1() -> Result<(), Box<dyn Error>> {
        let result = part1(EXAMPLE1.trim())?;
        assert_eq!(result, 102);
        Ok(())
    }

    #[test]
    fn example1_part2() -> Result<(), Box<dyn Error>> {
        let result = part2(EXAMPLE1.trim())?;
        assert_eq!(result, 94);
        Ok(())
    }

    #[test]
    fn example2_part2() -> Result<(), Box<dyn Error>> {
        let result = part2(EXAMPLE2.trim())?;
        assert_eq!(result, 71);
        Ok(())
    }
}
