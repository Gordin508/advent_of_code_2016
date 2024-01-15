#![allow(unused)]
#![allow(dead_code)]

// i can be statically defined as a complex number
static J: Complex = Complex { real: 0, imag: 1 };

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Complex {
    real: i32,
    imag: i32,
}

impl Complex {
    fn new(real: i32, imag: i32) -> Complex {
        Complex { real, imag }
    }
}

use std::ops::{Add, Sub, Mul, Neg};

// enable -J for complex numbers
impl Neg for Complex {
    type Output = Complex;

    fn neg(self) -> Complex {
        Complex {
            real: -self.real,
            imag: -self.imag,
        }
    }
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl Sub for Complex {
    type Output = Complex;

    fn sub(self, other: Complex) -> Complex {
        Complex {
            real: self.real - other.real,
            imag: self.imag - other.imag,
        }
    }
}

impl Mul for Complex {
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        let real = self.real * other.real - self.imag * other.imag;
        let imag = self.real * other.imag + other.real * self.imag;
        Complex::new(real, imag)    
    }
}

impl Mul<i32> for Complex {
    type Output = Complex;

    fn mul(self, other: i32) -> Complex {
        Complex::new(self.real * other, self.imag * other)
    }
}

fn part1(lines: &Vec<&str>) -> Option<i64> {
    let mut dir = Complex::new(0, 1);
    let mut pos = Complex::new(0, 0);
    assert_eq!(1, lines.len());
    for instr in lines[0].split(", ") {
        let rl = instr.chars().next().unwrap();
        let dist = instr[1..].parse::<i32>().unwrap();
        dir = match(rl) {
            'R' => dir * -J,
            'L' => dir * J,
            _ => panic!("Unknown direction")
        };
        pos = pos + dir * dist;
    }
    Some((pos.imag.abs() + pos.real.abs()) as i64)
}

fn part2(lines: &Vec<&str>) -> Option<i64> {
    let mut dir = Complex::new(0, 1);
    let mut pos = Complex::new(0, 0);
    use std::collections::HashSet;
    let mut seen = HashSet::new();
    seen.insert(pos);
    assert_eq!(1, lines.len());
    for instr in lines[0].split(", ") {
        let rl = instr.chars().next().unwrap();
        let dist = instr[1..].parse::<i32>().unwrap();
        dir = match(rl) {
            'R' => dir * -J,
            'L' => dir * J,
            _ => panic!("Unknown direction")
        };
        for i in (1..=dist) {
            // whether intersecting line segments would be faster depends entirely
            // on the input
            let ipos = pos + dir * i;
            if !seen.insert(ipos) {
                return Some((ipos.imag.abs() + ipos.real.abs()) as i64)
            }
        }
        pos = pos + dir * dist;
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
