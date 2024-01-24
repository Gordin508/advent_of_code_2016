#![allow(unused)]
#![allow(dead_code)]

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fmt::{self,Formatter,Display};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    chips: Vec<Vec<bool>>,
    generators: Vec<Vec<bool>>,
    elevator_pos: usize
}

impl State {
    fn next_states(&self) -> Vec<State> {
        let mut newstates = Vec::new();

        newstates
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut floors = vec![];
        for (floorid, (chips, generators)) in self.chips.iter().zip(self.generators.iter()).enumerate().rev() {
            let mut floor = vec![];
            for (chip, generator) in chips.iter().zip(generators.iter()) {
                if *chip && *generator {
                    floor.push("X");
                } else if *chip {
                    floor.push("M");
                } else if *generator {
                    floor.push("G");
                } else {
                    floor.push(".");
                }
            }
            floors.push(format!("F{} {}", floorid, floor.join(" ")));
        }
        write!(f, "{}", floors.join("\n"))
    }
}

fn parse_input(lines: &[&str]) -> State {
    let re = Regex::new(r"(?<name>[a-z]+)(-compatible)?\s(?<type>(generator|microchip))").unwrap();
    let mut material_id_map = HashMap::new();
    for line in lines {
        for cap in re.captures_iter(line) {
            let name = cap.name("name").unwrap().as_str();
            if !material_id_map.contains_key(&name) {
                material_id_map.insert(name, material_id_map.len());
            }
        }
    }
    let num_materials = material_id_map.len();
    let mut generators = vec![vec![false; num_materials];4];
    let mut chips = vec![vec![false; num_materials];4];
    for (floor, line) in lines.iter().enumerate() {
        for cap in re.captures_iter(line) {
            let name = cap.name("name").unwrap().as_str();
            let ntype = cap.name("type").unwrap().as_str();
            let material_id = *material_id_map.get(name).unwrap();
            if ntype == "generator" {
                generators[floor][material_id] = true;
            } else if ntype == "microchip" {
                chips[floor][material_id] = true;
            }
        }
    }
    State{generators, chips, elevator_pos: 0}
}

fn part1(lines: &Vec<&str>) -> Option<usize> {
    let initial_state = parse_input(lines);
    let mut queue = vec![initial_state];
    let mut seen = HashSet::new();
    let mut step = 0;
    while queue.len() != 0 {
        let mut newqueue = Vec::new();
        for state in queue {
            if state.generators[3].iter().zip(state.chips[3].iter()).all(|(c, g)| *c && *g) {
                return Some(step);
            }
            for next_state in state.next_states() {
                if !seen.contains(&next_state) {
                    seen.insert(next_state.clone());
                    newqueue.push(next_state);
                }
            }
        }
        queue = newqueue;
        step += 1;
    }

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
