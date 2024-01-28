#![allow(unused)]
#![allow(dead_code)]

fn gcd(x: usize, y: usize) -> usize {
    let mut x = x;
    let mut y = y;
    while y > 0 {
        (x, y) = (y, x % y);
    }
    x
}

fn lcm(x: usize, y: usize) -> usize {
    x * y / gcd(x, y)
}

struct Disc {
    len: usize,
    startpos: usize
}

impl From<&str> for Disc {
    fn from(value: &str) -> Self {
        let nums = value.split_whitespace()
                        .filter_map(|w| w.trim_end_matches('.').parse().ok())
                        .collect::<Vec<_>>();
        assert_eq!(2, nums.len());
        Disc{len: nums[0], startpos: nums[1]}
    }
}

fn fallthrough(discs: &[Disc]) -> Option<usize> {
    for i in (0..) {
        if discs.iter().enumerate().all(|(t, d)| (d.startpos + i + t + 1) % d.len == 0) {
           return Some(i);
        }
    }
    None
}

fn part1(lines: &Vec<&str>) -> Option<usize> {
    let discs = lines.iter().map(|l| Disc::from(*l)).collect::<Vec<_>>();
    fallthrough(&discs)
}

fn part2(lines: &Vec<&str>) -> Option<usize> {
    let mut discs = lines.iter().map(|l| Disc::from(*l)).collect::<Vec<_>>();
    discs.push(Disc{len: 11, startpos: 0});
    fallthrough(&discs)
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
    fn test_part1() {
        let discs = [Disc{len: 5, startpos: 4}, Disc{len: 2, startpos: 1}];
        assert_eq!(Some(5), fallthrough(&discs))
    }
}
