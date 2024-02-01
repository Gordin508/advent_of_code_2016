#![allow(unused)]
#![allow(dead_code)]

use std::cmp::max;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct IPRange {
    start: u32,
    end: u32
}

impl IPRange {
    fn new(start: u32, end: u32) -> IPRange {
        IPRange { start, end }
    }
}

impl From<&str> for IPRange {
    fn from(value: &str) -> Self {
        let (num1, num2) = value.split_once('-').unwrap();
        IPRange { start: num1.parse().unwrap(), end: num2.parse().unwrap() }
    }
}

fn part1(lines: &Vec<&str>) -> Option<usize> {
    let mut ranges = lines.iter().map(|l| IPRange::from(*l)).collect::<Vec<_>>();
    ranges.sort();
    let mut result = 0;
    for range in ranges.into_iter() {
        if range.start > result {
            return Some(result as usize)
        }
        result = max(result, range.end + 1)
    }
    None
}

fn part2(lines: &Vec<&str>) -> Option<usize> {
    let mut ranges = lines.iter().map(|l| IPRange::from(*l)).collect::<Vec<_>>();
    ranges.sort();
    let mut result = 0;
    let mut idx: usize = 0;
    let mut maxip = 0;
    for range in ranges.into_iter() {
        if range.start as usize > idx {
            result += range.start as usize - idx;
        }
        idx = max(idx, range.end as usize + 1);        
        maxip = max(maxip, range.end);
    }
    result += u32::MAX as usize - maxip as usize;
    Some(result as usize)
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
