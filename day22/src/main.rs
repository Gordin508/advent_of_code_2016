#![allow(unused)]
#![allow(dead_code)]

use std::usize;

use regex::Regex;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Node {
    x: usize,
    y: usize,
    size: u64,
    used: u64,
}

impl Node {
    fn new(x: usize, y: usize, size: u64, used: u64, avail: u64, used_percent: u8) -> Node {
        Node {x, y, size, used}
    }

    fn avail(&self) -> u64 {
        self.size - self.used
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct NodeGrid {
    nodes: Vec<Node>,
    width: usize,
    height: usize
}

use std::fmt::Display;

impl Display for NodeGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let node = &self[(x, y)];
                write!(f, "({}/{}) ", node.used, node.size)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl NodeGrid {
    fn new(nodes: &[Node], width: usize, height: usize) -> Self {
        assert_eq!(nodes.len(), width * height);
        NodeGrid { nodes: nodes.to_vec(), width, height }
    }
}

impl std::ops::Index<(usize, usize)> for NodeGrid {
    type Output = Node;
    fn index(&self, index: (usize, usize)) -> &Node {
        &self.nodes[index.0 * self.height + index.1]
    }
}

impl std::ops::IndexMut<(usize, usize)> for NodeGrid {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Node {
        &mut self.nodes[index.0 * self.height + index.1]
    }
}

fn parse_input(lines: &[&str]) -> Vec<Node> {
    let mut nodes = Vec::new();
    let lineregex = Regex::new(r"/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+(\d+)%").unwrap();
    for line in lines.iter().skip(2) {
        let captures = lineregex.captures(line).expect("Regex must match!");
        let nums = (1..=6).map(|i| captures.get(i).unwrap().as_str().parse::<u64>().unwrap()).collect::<Vec<_>>();
        nodes.push(Node::new(nums[0] as usize,
                             nums[1] as usize,
                             nums[2].into(),
                             nums[3].into(),
                             nums[4].into(),
                             nums[5] as u8));
    }
    nodes
}

fn nodes_viable(node1: &Node, node2: &Node, strict: bool) -> bool {
    if strict {
        if (usize::abs_diff(node1.x, node2.x) + usize::abs_diff(node1.y, node2.y)) != 1 {
            return false;
        }
    }
    return node1.used > 0 && node1.used <= node2.avail();
}

fn part1(lines: &Vec<&str>) -> Option<usize> {
    let nodes = parse_input(lines);
    assert_eq!(lines.len() - 2, nodes.len());
    let mut viable_pairs = 0;
    for node1 in nodes.iter() {
        for node2 in nodes.iter() {
            viable_pairs = if nodes_viable(node1, node2, false) {viable_pairs + 1} else {viable_pairs};
        }
    }
    Some(viable_pairs)
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct QueueEntry {
    emptypos: (usize, usize),
    datapos: (usize, usize),
}

impl QueueEntry {
    fn from_grid(grid: &NodeGrid, datapos: (usize, usize)) -> Self {
        let emptypos = grid.nodes.iter().filter(|n| n.used == 0).map(|n| (n.x, n.y)).next().unwrap();
        QueueEntry { emptypos, datapos }
    }

    fn new(grid: NodeGrid, emptypos: (usize, usize), datapos: (usize, usize)) -> Self {
        QueueEntry { emptypos, datapos }
    }
}

fn part2(lines: &Vec<&str>) -> Option<usize> {
    use std::collections::HashSet;
    let nodes = parse_input(lines);
    assert_eq!(lines.len() - 2, nodes.len());
    let (width, height) = nodes.iter().map(|n| (n.x + 1, n.y + 1)).max().unwrap();
    let grid = NodeGrid::new(&nodes, width as usize, height as usize);
    assert_eq!(grid[(1, 4)].x, 1); // sanity check
    assert_eq!(grid[(1, 4)].y, 4);
    let mut step = 0;
    let mut queue = vec![QueueEntry::from_grid(&grid, (width - 1, 0))];
    let mut seen = HashSet::new();
    seen.insert(queue[0].clone());

    let neighbors = |grid: &NodeGrid, (x, y)| -> Vec<Node> {
        let mut result = Vec::new();
        if x > 0 {
            result.push(grid[(x - 1, y)].clone());
        }
        if x < grid.width - 1 {
            result.push(grid[(x + 1, y)].clone());
        }
        if y > 0 {
            result.push(grid[(x, y - 1)].clone());
        }
        if y < grid.height - 1 {
            result.push(grid[(x, y + 1)].clone());
        }
        result
    };
    while queue.len() != 0 {
        let mut newqueue = Vec::new();
        for entry in queue {
            if entry.datapos == (0, 0) {
                return Some(step)
            }
            // for all neighbors in emptypos
            let emptypos = entry.emptypos;
            for n in neighbors(&grid, emptypos) {
                if n.used <= grid[emptypos].size {
                    let datapos = if (n.x, n.y) == entry.datapos {emptypos} else {entry.datapos};
                    let newentry = QueueEntry::new(grid.clone(), (n.x, n.y), datapos);
                    if seen.insert(newentry.clone()) {
                        newqueue.push(newentry);
                    }
                }
            }
        }
        queue = newqueue;
        step += 1;
    }

    None
}

fn main() {
    use std::fs;
    use std::env;
    use std::time::Instant;
    let args: Vec<String> =  env::args().collect();
    let infile = args.get(1).unwrap_or_else(|| {
        println!("Usage: {} <puzzle input>", args[0]);
        std::process::exit(1);
    });

    let contents = fs::read_to_string(infile)
        .expect("Could not read in file");

    let lines: Vec<&str> = contents.lines().collect();

    // execute part 1 and part 2, print their results if they exist
    // later parts may follow, so we loop over the part functions
    let parts = [part1, part2];
    for (index, part) in parts.iter().enumerate() {
        let partstart = Instant::now();
        let result = part(&lines);
        match result {
            Some(result) => println!("Part {}: {}\t({:?})", index+1, result, partstart.elapsed()),
            None => println!("Part {}: No result", index+1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static TESTINPUT: &str = "CHANGEME";

    #[test]
    fn test_part1() {
        let lines: Vec<&str> = TESTINPUT.lines().collect();
        assert_eq!(Some(1337), part1(&lines));
    }

    #[test]
    fn test_part2() {
        let lines: Vec<&str> = TESTINPUT.lines().collect();
        assert_eq!(Some(13337), part2(&lines));
    }
}
