#![allow(unused)]
#![allow(dead_code)]

use regex::Regex;
use lazy_static::lazy_static;
use std::cmp::{min, max};

lazy_static!{
    static ref VALUE_RE: Regex = Regex::new(r"value (?<value>\d+) goes to (?<destination>\w+ \d+)").unwrap();
    static ref BOT_RE: Regex = Regex::new(r"bot (?<botid>\d+) gives low to (?<lowdest>\w+ \d+) and high to (?<highdest>\w+ \d+)").unwrap();
}

#[derive(Debug, Clone, Copy)]
enum Destination {
    Bot(usize),
    Output(usize)
}

impl From<&str> for Destination {
    fn from(value: &str) -> Self {
        let (type_str, num_str) = value.split_once(' ').unwrap();
        let num = num_str.parse().unwrap();
        match type_str {
            "bot" => Destination::Bot(num),
            "output" => Destination::Output(num),
            _ => panic!("Unknown destination")
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    ValueTo(usize, Destination),
    BotLowHigh(usize, Destination, Destination)
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        if value.starts_with("value") {
            let mtch = VALUE_RE.captures(value).unwrap();
            let value = mtch.name("value").unwrap().as_str().parse().unwrap();
            let dest = mtch.name("destination").unwrap().as_str();
            let dest = Destination::from(dest);
            return Instruction::ValueTo(value, dest);
        } else if value.starts_with("bot") {
            let mtch = BOT_RE.captures(value).unwrap();
            let botid = mtch.name("botid").unwrap().as_str().parse().unwrap();
            let lowdest = Destination::from(mtch.name("lowdest").unwrap().as_str());
            let highdest = Destination::from(mtch.name("highdest").unwrap().as_str());
            return Instruction::BotLowHigh(botid, lowdest, highdest);
        } else {
            panic!("Unknown instruction format");
        }
    }
}

#[derive(Debug, Clone)]
struct Bot {
    id: usize,
    left: Option<usize>,
    right: Option<usize>,
    instruction: Option<Instruction>
}

impl Bot {
    fn new(id: usize) -> Bot {
        Bot{id, left: None, right: None, instruction: None}
    }

    fn add_chip(&mut self, value: usize) {
        if self.left.is_none() {
            self.left = Some(value)
        } else if self.right.is_none() {
            self.right = Some(value)
        } else {
            panic!("Got all hands full already");
        }
    }

    fn has_two_chips(&self) -> bool {
        self.left.is_some() && self.right.is_some()
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Output {
    id: usize,
    content: Option<usize>
}

impl Output {
    fn new(id: usize) -> Output {
        Output{id, content: None}
    }

    fn set_content(&mut self, content: usize) {
        if self.content.is_some() {
            panic!("Output already full");
        }
        self.content = Some(content);
    }
}

// hash bot solely based on id
impl std::hash::Hash for Bot {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

// impl eq for bot solely based on id
impl std::cmp::PartialEq for Bot {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl std::cmp::Eq for Bot {}


fn simulate_process(instructions: &[Instruction], part2: bool) -> Option<usize> {
    let mut bots = HashMap::new();
    let mut outputs = HashMap::new();

    let addbot = |id: usize, bots: &mut HashMap<usize, Bot>| {
        if !bots.contains_key(&id) {
            bots.insert(id, Bot::new(id));
        }
    };

    let addoutput = |id: usize, outputs: &mut HashMap<usize, Output>| {
        if !outputs.contains_key(&id) {
            outputs.insert(id, Output::new(id));
        }
    };

    let adddest = |destination: Destination, bots: &mut HashMap<usize, Bot>, outputs: &mut HashMap<usize, Output>| {
        match destination {
            Destination::Bot(id) => addbot(id, bots),
            Destination::Output(id) => addoutput(id, outputs)
        }
    };
    let mut queue = Vec::new();
    // create all bots and outputs
    for instruction in instructions.iter() {
        match instruction {
            Instruction::ValueTo(value, dest) => {
                adddest(*dest, &mut bots, &mut outputs);
            },
            Instruction::BotLowHigh(botid, dest1, dest2) => {
                adddest(*dest1, &mut bots, &mut outputs);
                adddest(*dest2, &mut bots, &mut outputs);
                addbot(*botid, &mut bots);
                bots.get_mut(botid).unwrap().instruction = Some(*instruction);
            }
        }
    }
    // deal out initial chips
    for instruction in instructions.iter() {
        match instruction {
            Instruction::ValueTo(value, dest) => {
                match dest {
                    Destination::Bot(id) => {
                        // test if bot already exists
                        // add value to bot
                        let mut bot = bots.get_mut(id).unwrap();
                        bot.add_chip(*value);
                        if bot.has_two_chips() {
                            queue.push(bot.clone());
                        }
                    },
                    Destination::Output(id) => {
                        // test if output already exists
                        // add value to output
                        let output = outputs.get_mut(id).unwrap();
                        output.set_content(*value);
                    }
                }
            },
            _ => {}
        }
    }

    'queueloop: while let Some(bot) = queue.pop() {
        let left = bot.left.unwrap();
        let right = bot.right.unwrap();
        let (low, high) = if left < right {(left, right)} else {(right, left)};
        if !part2 && low == 17 && high == 61 {
            return Some(bot.id);
        }
        if let Some(Instruction::BotLowHigh(botid, lowdest, highdest)) = bot.instruction {
            match lowdest {
                Destination::Bot(botid) => {
                    let mut lowbot = bots.get_mut(&botid).unwrap();
                    lowbot.add_chip(low);
                    if lowbot.has_two_chips() {
                        queue.push(lowbot.clone());
                    }
                },
                Destination::Output(outid) => outputs.get_mut(&outid).unwrap().set_content(low)
            };
            // DRY lost :(
            match highdest {
                Destination::Bot(botid) => {
                    let mut lowbot = bots.get_mut(&botid).unwrap();
                    lowbot.add_chip(high);
                    if lowbot.has_two_chips() {
                        queue.push(lowbot.clone());
                    }
                },
                Destination::Output(outid) => outputs.get_mut(&outid).unwrap().set_content(high)
            }
        }
        if part2 {
            let mut result = 1;
            for i in (0..3) {
                if let Some(value) = outputs.get(&i).unwrap().content {
                    result *= value;
                } else {
                    continue 'queueloop; 
                }
            }
            return Some(result)
        }
    }
    None
}

use std::collections::HashMap;
fn part1(lines: &Vec<&str>) -> Option<usize> {
    let instructions: Vec<Instruction> = lines.iter().map(|l| Instruction::from(*l)).collect();
    simulate_process(&instructions, false)
}

fn part2(lines: &Vec<&str>) -> Option<usize> {
    let instructions: Vec<Instruction> = lines.iter().map(|l| Instruction::from(*l)).collect();
    simulate_process(&instructions, true)
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
