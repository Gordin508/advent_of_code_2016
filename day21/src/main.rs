#![allow(unused)]
#![allow(dead_code)]

use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    SwapPosition(usize, usize),
    SwapLetter(char, char),
    RotateRight(usize),
    RotateLeft(usize),
    RotateLetter(char),
    Reverse(usize, usize),
    Move(usize, usize)
}

impl Instruction {
    fn parse_input(lines: &[&str]) -> Vec<Instruction> {
        let swap_re = Regex::new(r"swap position (\d+) with position (\d+)").unwrap();
        let swap_letter_re = Regex::new(r"swap letter (\w+) with letter (\w+)").unwrap();
        let rotate_re = Regex::new(r"rotate (left|right) (\d+) steps").unwrap();
        let rotate_letter_re = Regex::new(r"rotate based on position of letter (\w+)").unwrap();
        let reverse_re = Regex::new(r"reverse positions (\d+) through (\d+)").unwrap();
        let move_re = Regex::new(r"move position (\d+) to position (\d+)").unwrap();


        let mut instructions = Vec::new();

        for line in lines {
            if let Some(caps) = swap_re.captures(line) {
                instructions.push(Instruction::SwapPosition(caps[1].parse().unwrap(), caps[2].parse().unwrap()));
                continue;
            }
            if let Some(caps) = swap_letter_re.captures(line) {
                instructions.push(Instruction::SwapLetter(caps[1].chars().next().unwrap(), caps[2].chars().next().unwrap()));
                continue;
            }
            if let Some(caps) = rotate_re.captures(line) {
                let instruction = match(caps[1].to_string().as_ref()) {
                    "left" => Instruction::RotateLeft(caps[2].parse().unwrap()),
                    "right" => Instruction::RotateRight(caps[2].parse().unwrap()),
                    _ => panic!("Regex seems incorrect")
                };
                instructions.push(instruction);
                continue;
            }
            if let Some(caps) = rotate_letter_re.captures(line) {
                instructions.push(Instruction::RotateLetter(caps[1].chars().next().unwrap()));
                continue;
            }

            if let Some(caps) = reverse_re.captures(line) {
                instructions.push(Instruction::Reverse(caps[1].parse().unwrap(), caps[2].parse().unwrap()));
                continue;
            }
            if let Some(caps) = move_re.captures(line) {
                instructions.push(Instruction::Move(caps[1].parse().unwrap(), caps[2].parse().unwrap()));
                continue;
            }
        }

        return instructions;
    }
}

fn part1(lines: &Vec<&str>) -> Option<usize> {
    let instructions = Instruction::parse_input(lines);
    None
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

    #[test]
    fn test_parse() {
        let line = "move position 5 to position 12";
        let instrunctions = Instruction::parse_input(&[line]);
        assert_eq!(Instruction::Move(5, 12), instrunctions[0]);
    }
}
