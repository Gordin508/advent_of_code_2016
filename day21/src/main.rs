#![allow(unused)]
#![allow(dead_code)]

use std::mem::swap;

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
        let rotate_re = Regex::new(r"rotate (left|right) (\d+) steps?").unwrap();
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
            eprintln!("Could not parse {}", line);
            panic!("Parsing failed");
        }
 
        return instructions;
    }

    fn revert(&self, state: &[char]) -> Vec<char> {
        match *self {
            Instruction::SwapPosition(_, _)
                | Instruction::SwapLetter(_, _)
                | Instruction::Reverse(_, _) => {
                self.apply(state)
            },
            Instruction::RotateLeft(r) => {
                Instruction::RotateRight(r).apply(state)
            },
            Instruction::RotateRight(r) => {
                Instruction::RotateLeft(r).apply(state)
            },
            Instruction::RotateLetter(a) => {
                let mut result = state.to_vec();
                let (charpos, _) = state.iter().enumerate().filter(|(i, c)| **c == a).next().unwrap();
                // too tired for a closed form sulution right now
                // therefore we brute force the 'old' position of a
                for i in (0..state.len()) {
                    let r = 1 + i + if i >= 4 {1} else {0};
                    if (i + r) % state.len() == charpos {
                        // found rotation amount
                        return Instruction::RotateLeft(r).apply(state);
                    }
                }
                result
            },
            Instruction::Move(x, y) => {
                Instruction::Move(y, x).apply(state)
            }
        }
    }

    fn rotate_right(state: &[char], r: usize) -> Vec<char> {
        let r = r % state.len();
        assert!(r <= state.len());
        if r == state.len() {
            return state.to_vec();
        }
        let rlen = state.len();
        let mut result = Vec::new();
        result.resize(rlen, '\0');

        result[0..r].copy_from_slice(&state[rlen - r..rlen]);
        result[r..rlen].copy_from_slice(&state[0..rlen - r]);
        result
    }

    fn apply(&self, state: &[char]) -> Vec<char> {
        let mut result = state.to_vec();
        match *self {
            Instruction::SwapPosition(x, y) => {
                (result[x], result[y]) = (result[y], result[x]);
            },
            Instruction::SwapLetter(a, b) => {
                for l in result.iter_mut() {
                    if *l == a {
                        *l = b;
                    } else if *l == b {
                        *l = a;
                    }
                }
            },
            Instruction::RotateRight(r) => {
                return Self::rotate_right(state, r);
            },
            Instruction::RotateLeft(r) => {
                return Self::rotate_right(state, state.len() - r);
            },
            Instruction::RotateLetter(a) => {
                let (charpos, _) = state.iter().enumerate().filter(|(i, c)| **c == a).next().unwrap();
                assert!(charpos < state.len());
                let r = 1 + charpos + if charpos >= 4 {1} else {0};
                return Self::rotate_right(state, r);
            },
            Instruction::Reverse(x, y) => {
                let (x, y) = if x < y {(x, y)} else {(y, x)};
                for i in (x..=((x + y)/2)) {
                    result.swap(i, y - i + x);
                }
            },
            Instruction::Move(x, y) => {
                result[y] = state[x];
                if x < y {
                    result[x..y].copy_from_slice(&state[x + 1..=y]);
                } else if x > y {
                    result[y + 1..x + 1].copy_from_slice(&state[y..x]);
                }
            }
        }
        return result
    }
}

fn part1(lines: &Vec<&str>) -> Option<String> {
    let instructions = Instruction::parse_input(lines);
    assert_eq!(lines.len(), instructions.len());
    let startvalue = "abcdefgh";
    // let startvalue = "abcde";
    let mut result = startvalue.chars().collect::<Vec<_>>();
    for instr in instructions {
        result = instr.apply(&result);
    }
    Some(result.into_iter().collect::<String>())
}

fn part2(lines: &Vec<&str>) -> Option<String> {
    let instructions = Instruction::parse_input(lines);
    assert_eq!(lines.len(), instructions.len());
    let startvalue = "fbgdceah";
    let mut result = startvalue.chars().collect::<Vec<_>>();
    for instr in instructions.iter().rev() {
        result = instr.revert(&result);
    }
    Some(result.into_iter().collect::<String>())
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

    #[test]
    #[ignore]
    fn test_part1() {
        let testinput = "swap position 4 with position 0
swap letter d with letter b
reverse positions 0 through 4
rotate left 1 step
move position 1 to position 4
move position 3 to position 0
rotate based on position of letter b
rotate based on position of letter d";
        // the test won't work unless you change the startvalue inside part1 to "abcde"
        assert_eq!(part1(&testinput.lines().collect::<Vec<_>>()), Some(String::from("decab")));
    }
}
