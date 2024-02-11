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

    fn is_literal(&self) -> bool {
        matches!(self, IValue::Literal(_))
    }

    fn is_register(&self) -> bool {
        matches!(self, IValue::Register(_))
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Cpy(IValue, IValue),
    Inc(IValue),
    Dec(IValue),
    Jnz(IValue, IValue),
    Tgl(IValue),
    Out(IValue)
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
            "tgl" => Instruction::Tgl(args[0]),
            "out" => Instruction::Out(args[0]),
            _ => panic!("Unknown instruction")
        }
    }
}

impl Instruction {
    fn toggle(&self) -> Instruction {
        match *self {
            Instruction::Inc(val) => Instruction::Dec(val),
            Instruction::Dec(val) | Instruction::Tgl(val) | Instruction::Out(val) => Instruction::Inc(val),
            Instruction::Cpy(val1, val2) => Instruction::Jnz(val1, val2),
            Instruction::Jnz(val1, val2) => Instruction::Cpy(val1, val2)
        }
    }
}

fn run(instructions: &[Instruction], regs: [isize; 4]) -> [isize; 4] {
    let mut virtual_instructions = instructions.to_vec();
    let mut toggled = vec![false; virtual_instructions.len()];
    let mut regs = regs;
    let mut ip = 0;
    let resolve = |val: &IValue, regs: &[isize; 4]| -> isize {
        match val {
            IValue::Literal(intval) => *intval,
            IValue::Register(index) => regs[*index as usize]
        }
    };
    let mut expected_signal = 0;
    let mut num_signals = 0;
    while (ip < virtual_instructions.len()) {
        // we naively assume that if it sends the correct sequence for a length of 128,
        // it will stay that way
        if num_signals > 128 {
            regs[0] = 1;
            return regs;
        }
        let instruction = &virtual_instructions[ip];
        ip += 1;
        match instruction {
            Instruction::Cpy(src, dest) => {
                if dest.is_register() {
                    let srcval = resolve(src, &regs);
                    regs[dest.unwrap() as usize] = srcval;
                }
            },
            Instruction::Inc(dest) => {
                if dest.is_register() {
                    regs[dest.unwrap() as usize] += 1;
                }
            },
            Instruction::Dec(dest) => {
                if dest.is_register() {
                    regs[dest.unwrap() as usize] -= 1;
                }
            },
            Instruction::Jnz(src, jmprange) => {
                let srcval = resolve(src, &regs);
                if srcval != 0 {
                    ip = (ip as isize + resolve(jmprange, &regs) - 1) as usize;
                }
            },
            Instruction::Out(val) => {
                let signal = resolve(val, &regs);
                if signal != expected_signal {
                    regs[0] = 0;
                    return regs;
                }
                expected_signal = if expected_signal == 0 {1} else {0};
                num_signals += 1;
            }
            Instruction::Tgl(dest) => {
                let index = ip as isize + resolve(dest, &regs) - 1;
                if index >= 0 && (index as usize) < virtual_instructions.len() {
                    let index = index as usize;
                    // TODO: It is unclear whether we want to actually toggle like this,
                    // or just reapply the toggling rule as if it weren't toggled already
                    if toggled[index] {
                        virtual_instructions[index] = instructions[index].clone();
                        toggled[index] = false;
                    } else {
                        virtual_instructions[index as usize] = virtual_instructions[index as usize].toggle();
                        toggled[index] = true;
                    }
                }
            }
        }
    }
    regs
}

fn part1(lines: &Vec<&str>) -> Option<usize> {
    let instructions = lines.iter().map(|l| Instruction::from(*l)).collect::<Vec<_>>();
    for i in (0..) {
        let mut regs = [0isize; 4];
        regs[0] = i;
        let result = run(&instructions, regs);
        if result[0] == 1 {
            return Some(i as usize);
        }
    }
    None
}


fn part2(lines: &Vec<&str>) -> Option<usize> {
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
    static TESTINPUT: &str = "cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a";

    #[test]
    fn test_part1() {
        let lines: Vec<&str> = TESTINPUT.lines().collect();
        let instructions = lines.iter().map(|l| Instruction::from(*l)).collect::<Vec<_>>();
        let regs = run(&instructions, [0isize; 4]);
        assert_eq!(3, regs[0]);
    }

    #[test]
    #[ignore]
    fn test_part2() {
        let lines: Vec<&str> = TESTINPUT.lines().collect();
        assert_eq!(Some(13337), part2(&lines));
    }
}
