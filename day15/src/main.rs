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

#[derive(Debug)]
struct EucledianResult {
    s: i64,
    t: i64,
    coefficients: (i64, i64),
    gcd: i64
}

fn extended_euclidian(a: i64, b: i64) -> EucledianResult {
    let mut s = 0;
    let mut t = 1;
    let mut r = b;
    let mut old_s = 1;
    let mut old_t = 0;
    let mut old_r = a;
    while r != 0 {
        let quotient = old_r / r;
        (old_r, r) = (r, old_r - quotient * r); // r = old_r % r
        (old_s, s) = (s, old_s - quotient * s);
        (old_t, t) = (t, old_t - quotient * t);
    }
    return EucledianResult { s,
                             t,
                             coefficients: (old_s, old_t),
                             gcd: old_r }
}

fn inverse_mod(x: i64, n: i64) -> i64 {
    let eea = extended_euclidian(x, n);
    (eea.coefficients.0 + n) % n
}

struct Disc {
    len: u32,
    startpos: u32
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
        if discs.iter().enumerate().all(|(t, d)| (d.startpos + i + t as u32 + 1) % d.len == 0) {
           return Some(i as usize);
        }
    }
    None
}

#[allow(non_snake_case)]
fn fallthrough_chinese_remainder_theorem(discs: &[Disc]) -> Option<u64> {
    let N: i64 = discs.iter().map(|d| d.len).product::<u32>().into();
    let mut result: i64 = discs.iter().enumerate()
                      .map(|(t, disc)| -> i64 {
                           let n_disk: i64 = N / disc.len as i64;
                           let inverse = inverse_mod(n_disk, disc.len.into());
                           n_disk * inverse * (disc.len as i64 - disc.startpos as i64 - t as i64 - 1)
                       })
                      .sum();
    if result < 0 {
        result = (result % N) + N;
    }
    Some((result % N) as u64)
}

fn part1(lines: &Vec<&str>) -> Option<u64> {
    let discs = lines.iter().map(|l| Disc::from(*l)).collect::<Vec<_>>();
    fallthrough_chinese_remainder_theorem(&discs)
}

fn part2(lines: &Vec<&str>) -> Option<u64> {
    let mut discs = lines.iter().map(|l| Disc::from(*l)).collect::<Vec<_>>();
    discs.push(Disc{len: 11, startpos: 0});
    fallthrough_chinese_remainder_theorem(&discs)
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

    #[test]
    fn test_part1_cmt() {
        let discs = [Disc{len: 5, startpos: 4}, Disc{len: 2, startpos: 1}];
        assert_eq!(Some(5), fallthrough_chinese_remainder_theorem(&discs))
    }

    #[test]
    fn test_eea() {
        let eea_result = extended_euclidian(6, 12);
        assert_eq!(6, eea_result.gcd);
    }

    #[test]
    fn test_inverse() {
        let inverse = inverse_mod(3, 5);
        assert_eq!(2, inverse);
        let inverse = inverse_mod(inverse, 5);
        assert_eq!(3, inverse);
    }

    #[test]
    fn test_inverse_2() {
        let inverse = inverse_mod(11, 26);
        assert_eq!(19, inverse);
        let inverse = inverse_mod(inverse, 26);
        assert_eq!(11, inverse);
    }
}
