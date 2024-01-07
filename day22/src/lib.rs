pub mod config;

use std::collections::{HashSet, HashMap};
use std::error::Error;
use std::fs;

type BlockId = usize;

#[derive(Clone, Debug)]
struct Block {
    id: BlockId,
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
    min_z: usize,
    max_z: usize,
}

struct Puzzle {
    blocks: Vec<Block>,
    heights: Vec<Vec<usize>>,
    max_x: usize,
    max_y: usize,
    supports: Vec<HashSet<BlockId>>,
    supported_by: Vec<HashSet<BlockId>>,
    supports_exclusively: Vec<HashSet<BlockId>>,
}

impl Puzzle {
    fn from_input(input: &str) -> Result<Self, Box<dyn Error>> {
        let mut blocks = Vec::new();
        let mut global_max_x = 0;
        let mut global_max_y = 0;
        let mut id = 0;
        for line in input.lines() {
            let (pos1, pos2) = line.split_once("~").ok_or("Tilde not found")?;
            let mut iter1 = pos1.splitn(3, ",");
            let mut iter2 = pos2.splitn(3, ",");
            let x1: usize = iter1.next().ok_or("Syntax error")?.parse()?;
            let y1: usize = iter1.next().ok_or("Syntax error")?.parse()?;
            let z1: usize = iter1.next().ok_or("Syntax error")?.parse()?;
            let x2: usize = iter2.next().ok_or("Syntax error")?.parse()?;
            let y2: usize = iter2.next().ok_or("Syntax error")?.parse()?;
            let z2: usize = iter2.next().ok_or("Syntax error")?.parse()?;
            let min_x = x1.min(x2);
            let min_y = y1.min(y2);
            let min_z = z1.min(z2);
            let max_x = x1.max(x2);
            let max_y = y1.max(y2);
            let max_z = z1.max(z2);
            global_max_x = global_max_x.max(x1).max(x2);
            global_max_y = global_max_y.max(y1).max(y2);
            let block = Block { id, min_x, max_x, min_y, max_y, min_z, max_z };
            blocks.push(block);
            id += 1;
        }
        blocks.sort_by_key(|b| b.min_z);
        let mut heights = vec![vec![0; global_max_y+1]; global_max_x+1];
        for block in &blocks {
            for x in block.min_x..=block.max_x {
                for y in block.min_y..=block.max_y {
                    assert!(heights[x][y] < block.max_z, "{} >= {}", heights[x][y], block.max_z);
                    heights[x][y] = block.max_z;
                }
            }
        }
        Ok(Self {
            heights,
            max_x: global_max_y,
            max_y: global_max_y,
            supports: vec![HashSet::new(); blocks.len()],
            supported_by: vec![HashSet::new(); blocks.len()],
            supports_exclusively: vec![HashSet::new(); blocks.len()],
            blocks,
        })
    }

    fn drop_blocks(&mut self) {
        for vec in &mut self.heights {
            vec.fill(0);
        }
        // Block ID and its max_z
        let mut highest_block: Vec<Vec<Option<(BlockId, usize)>>> = vec![vec![None; self.max_y + 1]; self.max_x + 1];
        for block in &mut self.blocks {
            let mut z = 0;  // greatest height value under the block
            for x in block.min_x..=block.max_x {
                for y in block.min_y..=block.max_y {
                    z = z.max(self.heights[x][y]);
                }
            }
            for x in block.min_x..=block.max_x {
                for y in block.min_y..=block.max_y {
                    if let Some((block_below_id, block_below_max_z)) = highest_block[x][y] {
                        if block_below_max_z == z {
                            self.supported_by[block.id].insert(block_below_id);
                            self.supports[block_below_id].insert(block.id);
                        }
                    }
                }
            }
            let block_height = block.max_z - block.min_z + 1;
            block.min_z = z + 1;
            block.max_z = z + block_height;
            for x in block.min_x..=block.max_x {
                for y in block.min_y..=block.max_y {
                    assert!(self.heights[x][y] < block.max_z, "{} >= {}", self.heights[x][y], block.max_z);
                    self.heights[x][y] = block.max_z;
                    highest_block[x][y] = Some((block.id, block.max_z));
                }
            }
        }
        for block in &self.blocks {
            self.supports_exclusively[block.id] = self.supports[block.id].iter().cloned().filter(
                |&id| self.supported_by[id].len() == 1
            ).collect();
        }
    }

    fn num_disintegratable(&self) -> usize {
        self.blocks.iter().filter(|b| self.supports_exclusively[b.id].is_empty()).count()
    }

    fn sum_falling(&self) -> usize {
        let mut n = 0;
        let mut support_for = HashMap::new();
        for (id, support) in self.supported_by.iter().enumerate() {
            if !support.is_empty() {
                let mut support: Vec<_> = support.iter().cloned().collect();
                support.sort();
                let support_supports = support_for.entry(support).or_insert(HashSet::new());
                support_supports.insert(id);
            }
        }
        let mut blocks = self.blocks.clone();
        blocks.sort_by_key(|b| usize::MAX - b.max_z);
        // Observation:
        // Let A be a block. To compute the set F of blocks that would fall by disintegrating A, we
        // initialize F to the blocks exclusively supported by A. We repeat the following until
        // nothing changes anymore: Add to F all blocks whose support is a subset of F.
        for block in blocks {
            let mut falling = self.supports_exclusively[block.id].clone();
            loop {
                let mut change = false;
                for (support, supported) in &support_for {
                    // Is support a subset of falling?
                    if support.iter().all(|id| falling.contains(id)) {
                        for b in supported {
                            let inserted = falling.insert(*b);
                            change = change || inserted;
                        }
                    }
                }
                if !change {
                    break;
                }
            }
            n += falling.len();
        }
        n
    }
}

fn part1(input: &str) -> Result<usize, Box<dyn Error>> {
    let mut puzzle = Puzzle::from_input(input)?;
    puzzle.drop_blocks();
    Ok(puzzle.num_disintegratable())
}

fn part2(input: &str) -> Result<usize, Box<dyn Error>> {
    let mut puzzle = Puzzle::from_input(input)?;
    puzzle.drop_blocks();
    Ok(puzzle.sum_falling())
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
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
";

    #[test]
    fn example1_part1() -> Result<(), Box<dyn Error>> {
        let result = part1(EXAMPLE1.trim())?;
        assert_eq!(result, 5);
        Ok(())
    }

    #[test]
    fn example1_part2() -> Result<(), Box<dyn Error>> {
        let result = part2(EXAMPLE1.trim())?;
        assert_eq!(result, 7);
        Ok(())
    }
}
