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
    grid: NodeGrid,
    datapos: (usize, usize)
}

impl QueueEntry {
    fn new(grid: NodeGrid, datapos: (usize, usize)) -> Self {
        QueueEntry { grid, datapos }
    }
}

fn part2(lines: &Vec<&str>) -> Option<usize> {
    use std::collections::HashSet;
    let nodes = parse_input(lines);
    assert_eq!(lines.len() - 2, nodes.len());
    let (width, height) = nodes.iter().map(|n| (n.x + 1, n.y + 1)).max().unwrap();
    println!("Nodes: {}, width: {}, height: {}", nodes.len(), width, height);
    let grid = NodeGrid::new(&nodes, width as usize, height as usize);
    assert_eq!(grid[(1, 4)].x, 1);
    assert_eq!(grid[(1, 4)].y, 4);
    let mut step = 0;
    let mut queue = vec![QueueEntry::new(grid, (width, 0))];
    let mut seen = HashSet::new();
    seen.insert(queue[0].clone());
    while queue.len() != 0 {
        let mut newqueue = Vec::new();
        for entry in queue {
            if entry.datapos == (0, 0) {
                return Some(step)
            }
            for node1 in entry.grid.nodes.iter() {
                for node2 in entry.grid.nodes.iter() {
                    if nodes_viable(node1, node2, true) {
                        // new queueentry with updated notes
                        let mut newgrid = entry.grid.clone();
                        newgrid[(node2.x, node2.y)].used += newgrid[(node1.x, node1.y)].used;
                        newgrid[(node1.x, node1.y)].used = 0;
                        let datapos = if (node1.x, node1.y) == entry.datapos {(node2.x, node2.y)} else {entry.datapos};
                        let new_entry = QueueEntry::new(newgrid, datapos);
                        if seen.insert(new_entry.clone()) {
                            newqueue.push(new_entry);
                        }
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
