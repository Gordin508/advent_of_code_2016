#![allow(unused)]
#![allow(dead_code)]

struct Room {
    x: usize,
    y: usize
}

impl Room {
    fn new(x: usize, y: usize) -> Room {
        Room { x, y }
    }

    fn up(&self) -> Room {
        Room { x: self.x, y: self.y - 1 }
    }
    fn down(&self) -> Room {
        Room { x: self.x, y: self.y + 1 }
    }
    fn left(&self) -> Room {
        Room { x: self.x - 1, y: self.y }
    }
    fn right(&self) -> Room {
        Room { x: self.x + 1, y: self.y }
    }
}

struct BFSDFSFrame {
    room: Room,
    history: Vec<u8>,
    steps: usize,
}

impl BFSDFSFrame {
    fn new(room: Room, history: Vec<u8>, steps: usize) -> BFSDFSFrame {
        BFSDFSFrame { room, history , steps }
    }

    fn hash(&self, puzzle_input: &[u8]) -> md5::Digest {
        let mut hash_input = puzzle_input.to_vec();
        hash_input.append(&mut self.history.clone());
        md5::compute(hash_input)
    }

    fn new_history(&self, newchar: char) -> Vec<u8> {
        let mut result = self.history.clone();
        result.push(newchar as u8);
        result
    }

    fn adjascent(&self, puzzle_input: &[u8]) -> Vec<BFSDFSFrame> {
        let digest = self.hash(puzzle_input);
        let hash = digest.iter().take(2).collect::<Vec<_>>();
        let hash = [hash[0] >> 4, hash[0] & 0xf, hash[1] >> 4, hash[1] & 0xf];
        assert_eq!(4, hash.len());
        let mut result = Vec::new();
        if self.room.y > 0 && hash[0] > 10 {
            result.push(BFSDFSFrame::new(self.room.up(), self.new_history('U'), self.steps + 1));
        }
        if self.room.y < 3 && hash[1] > 10 {
            result.push(BFSDFSFrame::new(self.room.down(), self.new_history('D'), self.steps + 1));
        }
        if self.room.x > 0 && hash[2] > 10 {
            result.push(BFSDFSFrame::new(self.room.left(), self.new_history('L'), self.steps + 1));
        }
        if self.room.x < 3 && hash[3] > 10 {
            result.push(BFSDFSFrame::new(self.room.right(), self.new_history('R'), self.steps + 1));
        }
        result
    }
}


fn part1(lines: &Vec<&str>) -> Option<String> {
    assert_eq!(1, lines.len());
    let puzzle_input = lines[0].bytes().collect::<Vec<_>>();
    let mut queue = Vec::new();
    queue.push(BFSDFSFrame::new(Room::new(0, 0), Vec::new(), 0));
    let mut steps = 0;
    while queue.len() > 0 {
        let mut newqueue = Vec::new();
        for frame in queue {
            if frame.room.x == 3 && frame.room.y == 3 {
                let result = frame.history.into_iter().map(|v| v as char).collect::<String>();
                return Some(result);
            }
            newqueue.append(&mut frame.adjascent(&puzzle_input));
        }


        queue = newqueue;
        steps += 1;
    }
    None
}

fn part2(lines: &Vec<&str>) -> Option<String> {
    use std::cmp::max;
    assert_eq!(1, lines.len());
    let puzzle_input = lines[0].bytes().collect::<Vec<_>>();
    let mut stack = Vec::new();
    stack.push(BFSDFSFrame::new(Room::new(0, 0), Vec::new(), 0));
    let mut longest = 0;
    // all paths are (stochastically) finite,
    // i.e. will end in a situation where all doors are closed
    while let Some(frame) = stack.pop() {
        if frame.room.x == 3 && frame.room.y == 3 {
            longest = max(longest, frame.steps);
        } else {
            stack.append(&mut frame.adjascent(&puzzle_input));
        }
    }
    Some(longest.to_string())
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
    }

    #[test]
    fn test_part2() {
        let lines: Vec<&str> = TESTINPUT.lines().collect();
    }
}
