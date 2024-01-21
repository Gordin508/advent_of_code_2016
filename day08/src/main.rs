#![allow(unused)]
#![allow(dead_code)]

use std::fmt::Display;


#[derive(Debug, PartialEq, Eq)]
enum ScreenOp {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateColumn(usize, usize)
}

impl From<&str> for ScreenOp {
    fn from(value: &str) -> Self {
        if value.starts_with("rect") {
            let (num1, num2) = &value[5..].split_once('x').unwrap();
            return ScreenOp::Rect(num1.parse().unwrap(), num2.parse().unwrap());
        } else if value.starts_with("rotate") {
            let (_, nums) = value.split_once("=").unwrap();
            let (num1, num2) = nums.split_once(" by ").unwrap();
            let num1 = num1.parse().unwrap();
            let num2 = num2.parse().unwrap();
            if value.contains("row") {
                return ScreenOp::RotateRow(num1, num2);
            } else {
                return ScreenOp::RotateColumn(num1, num2);
            }
        } else {
            panic!("Unknown operation")
        }
    }
}

struct Screen {
    pixels: Vec<i8>,
    width: usize,
    height: usize
}


impl Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for yi in 0..self.height {
            for xi in 0..self.width {
                let pixel = self.pixels[yi * self.width + xi];
                if pixel == 0 {
                    write!(f, " ")?;
                } else {
                    write!(f, "#")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Screen {
    fn new(width: usize, height: usize) -> Screen {
        let mut pixels = Vec::new();
        pixels.resize(width * height, 0);
        Screen {pixels, width, height}
    }

    fn perform_mut(&mut self, op: ScreenOp) {
        match op {
            ScreenOp::Rect(x, y) => {
                for yi in 0..y {
                    for xi in 0..x {
                        self.pixels[yi * self.width + xi] = 1;
                    }
                }
            },
            ScreenOp::RotateColumn(x, amount) => {
                let mut original = vec![0; self.height];
                for yi in (0..self.height) {
                    original[yi] = self.pixels[yi * self.width + x];
                }
                for yi in (0..self.height) {
                    let sourcepos = (yi + self.height - amount) % self.height;
                    self.pixels[yi * self.width + x] = original[sourcepos];
                }
            },
            ScreenOp::RotateRow(y, amount) => {
                let mut original = vec![0; self.width];
                for xi in (0..self.width) {
                    original[xi] = self.pixels[y * self.width + xi];
                }
                for xi in (0..self.width) {
                    let sourcepos = (xi + self.width - amount) % self.width;
                    self.pixels[y * self.width + xi] = original[sourcepos];
                }
            },
            _ => panic!("Untreated op")
        }
    }
}

fn part1(lines: &Vec<&str>) -> Option<usize> {
    let mut screen = Screen::new(50, 6);
    for line in lines {
        let op = ScreenOp::from(*line);
        screen.perform_mut(op);
    }
    println!("{}", screen);
    Some(screen.pixels.iter().filter(|p| **p != 0).count())
}

fn part2(lines: &Vec<&str>) -> Option<usize> {
    // see console output for part 1
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
