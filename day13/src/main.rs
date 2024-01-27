#![allow(unused)]
#![allow(dead_code)]

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
struct Coordinate {
    x: usize,
    y: usize
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Coordinate {
        Coordinate{x, y}
    }

    fn neighbors(&self) -> impl Iterator<Item=Coordinate> {
        let x = self.x;
        let y = self.y;
        let mut neighbors = Vec::new();
        if x > 0 {
            neighbors.push(Coordinate{x: x-1, y});
        }
        if y > 0 {
            neighbors.push(Coordinate{x, y: y-1});
        }
        neighbors.push(Coordinate{x: x+1, y});
        neighbors.push(Coordinate{x, y: y+1});
        neighbors.into_iter()
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct QueueEntry {
    pos: Coordinate,
    steps: usize
}

impl QueueEntry {
    fn new(position: Coordinate, steps: usize) -> QueueEntry {
        QueueEntry{pos: position, steps}
    }
}

impl std::cmp::Ord for QueueEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.steps.cmp(&other.steps)
    }
}

impl std::cmp::PartialOrd for QueueEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

fn is_wall(coordinate: Coordinate, favorite_number: usize) -> bool {
    let x = coordinate.x;
    let y = coordinate.y;
    let magic = x * x + 3 * x + 2 * x * y + y + y * y + favorite_number;
    if magic.count_ones() % 2 == 1 { true } else { false }
}

fn fastest_path(favorite_number: usize, destination: Coordinate) -> usize {
    use std::collections::{HashSet, VecDeque}; 
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back(QueueEntry::new(Coordinate::new(1, 1), 0));
    while let Some(entry) = queue.pop_front() {
        if entry.pos == destination {
            return entry.steps;
        }
        for neighbor in entry.pos.neighbors() {
            if !is_wall(neighbor, favorite_number) && visited.insert(neighbor) {
                queue.push_back(QueueEntry::new(neighbor, entry.steps + 1));
            }
        }
    }
    usize::MAX
}

fn num_reachable_locations(favorite_number: usize, max_steps: usize) -> usize {
    use std::collections::{HashSet, VecDeque}; 
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back(QueueEntry::new(Coordinate::new(1, 1), 0));
    while let Some(entry) = queue.pop_front() {
        if entry.steps >= max_steps {
            continue;
        }
        for neighbor in entry.pos.neighbors() {
            if !is_wall(neighbor, favorite_number) && visited.insert(neighbor) {
                queue.push_back(QueueEntry::new(neighbor, entry.steps + 1));
            }
        }
    }
    visited.len()
}

fn part1(lines: &Vec<&str>) -> Option<usize> {
    assert_eq!(lines.len(), 1);
    let puzzle_input = lines[0].parse::<usize>().unwrap();
    let destination = Coordinate{x: 31, y: 39};
    Some(fastest_path(puzzle_input, destination))
}

fn part2(lines: &Vec<&str>) -> Option<usize> {
    assert_eq!(lines.len(), 1);
    let puzzle_input = lines[0].parse::<usize>().unwrap();
    Some(num_reachable_locations(puzzle_input, 50))
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

    #[test]
    fn test_shortest_path() {
        let favorite = 10;
        let destination = Coordinate::new(7, 4);
        assert_eq!(11, fastest_path(favorite, destination));
    }
}
