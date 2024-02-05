#![allow(unused)]
#![allow(dead_code)]

use regex::Regex;

#[derive(Debug, Eq, PartialEq)]
struct Node {
    x: u64,
    y: u64,
    size: u64,
    used: u64,
    avail: u64,
    used_percent: u8
}

fn parse_input(lines: &[&str]) -> Vec<Node> {
    let mut nodes = Vec::new();
    let lineregex = Regex::new(r"/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+(\d+)%").unwrap();
    for line in lines.iter().skip(2) {
        let captures = lineregex.captures(line).expect("Regex must match!");
        let nums = (1..=6).map(|i| captures.get(i).unwrap().as_str().parse::<u64>().unwrap()).collect::<Vec<_>>();
        nodes.push(Node{x: nums[0].into(),
                        y: nums[1].into(),
                        size: nums[2].into(),
                        used: nums[3].into(),
                        avail: nums[4].into(),
                        used_percent: nums[5] as u8});
    }
    nodes
}

fn nodes_viable(node1: &Node, node2: &Node) -> bool {
    return node1.used > 0 && node1.used <= node2.avail;
}

fn part1(lines: &Vec<&str>) -> Option<usize> {
    let nodes = parse_input(lines);
    assert_eq!(lines.len() - 2, nodes.len());
    let mut viable_pairs = 0;
    for node1 in nodes.iter() {
        for node2 in nodes.iter() {
            viable_pairs = if nodes_viable(node1, node2) {viable_pairs + 1} else {viable_pairs};
        }
    }
    Some(viable_pairs)
}

fn part2(lines: &Vec<&str>) -> Option<usize> {
    //TODO: implement me
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
