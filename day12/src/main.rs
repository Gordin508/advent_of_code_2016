#![allow(unused)]
#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IValue {
    Register(u8),
    Literal(isize)
}

impl IValue {
    fn unwrap(&self) -> isize {
        match self {
            IValue::Literal(intval) => *intval as isize,
            IValue::Register(index) => *index as isize
        }
    }
}

enum Instruction {
    Cpy(IValue, IValue),
    Inc(IValue),
    Dec(IValue),
    Jnz(IValue, IValue)
}

impl From<&str> for Instruction {
    fn from(line: &str) -> Self {
        let (name, args) = line.split_once(' ').unwrap();
        let arg_to_value = |s: &str| -> IValue {
            if s.chars().all(|c| c == '-' || c.is_numeric()) {
                IValue::Literal(s.parse().unwrap())
            } else {
                assert!(s.len() == 1);
                IValue::Register(s.bytes().next().unwrap() as u8 - 'a' as u8)
            }
        };
        let args = args.split_whitespace().map(arg_to_value).collect::<Vec<_>>();
        match name.to_lowercase().as_str() {
            "cpy" => Instruction::Cpy(args[0], args[1]),
            "inc" => Instruction::Inc(args[0]),
            "dec" => Instruction::Dec(args[0]),
            "jnz" => Instruction::Jnz(args[0], args[1]),
            _ => panic!("Unknown instruction")
        }
    }
}

fn run(instructions: &[Instruction], regs: [isize; 4]) -> [isize; 4] {
    let mut regs = regs;
    let mut ip = 0;
    let resolve = |val: &IValue, regs: &[isize; 4]| -> isize {
        match val {
            IValue::Literal(intval) => *intval,
            IValue::Register(index) => regs[*index as usize]
        }
    };
    while (ip < instructions.len()) {
        let instruction = &instructions[ip];
        ip += 1;
        match instruction {
            Instruction::Cpy(src, dest) => {
                let srcval = resolve(src, &regs);
                regs[dest.unwrap() as usize] = srcval;
            },
            Instruction::Inc(dest) => {
                regs[dest.unwrap() as usize] += 1;
            },
            Instruction::Dec(dest) => {
                regs[dest.unwrap() as usize] -= 1;
            },
            Instruction::Jnz(src, jmprange) => {
                let srcval = resolve(src, &regs);
                if srcval != 0 {
                    ip = (ip as isize + resolve(jmprange, &regs) - 1) as usize;
                }
            }
        }
    }
    regs
}

fn part1(lines: &Vec<&str>) -> Option<usize> {
    let instructions = lines.iter().map(|l| Instruction::from(*l)).collect::<Vec<_>>();
    let mut regs = [0isize; 4];
    let regs = run(&instructions, regs);
    Some(regs[0] as usize)
}

fn part2(lines: &Vec<&str>) -> Option<usize> {
    let instructions = lines.iter().map(|l| Instruction::from(*l)).collect::<Vec<_>>();
    let mut regs = [0isize; 4];
    regs[2] = 1;
    let regs = run(&instructions, regs);
    Some(regs[0] as usize)
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
