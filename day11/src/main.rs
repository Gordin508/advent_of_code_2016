#![allow(unused)]
#![allow(dead_code)]

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fmt::{self,Formatter,Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Material {
    generator: usize,
    chip: usize
}

impl Material {
    fn new(generator: usize, chip: usize) -> Material {
        Material{generator, chip}
    }

    fn on_floor(&self, floor_id: usize) -> bool {
        self.generator == floor_id || self.chip == floor_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    materials: Vec<Material>,
    elevator_pos: usize
}

fn materials_valid(materials: &[Material]) -> bool {
    // check if material configuration is allowed
    let num_materials = materials.len();
    for floor in (0..4) {
        // all chips without RTG
        let chipsonfloor: Vec<usize> = (0..num_materials).filter(|i| materials[*i].chip == floor && materials[*i].generator != floor).collect();
        // all RTGs
        let rtgsonfloor: Vec<usize> = (0..num_materials).filter(|i| materials[*i].generator == floor).collect();
        if chipsonfloor.len() > 0 && rtgsonfloor.len() > 0 {
            return false;
        }
    }
    true
}

impl State {
    fn new(materials: Vec<Material>, elevator_pos: usize) -> State {
        let mut materials = materials;
        materials.sort_unstable();
        State{materials, elevator_pos}
    }
    fn next_states(&self) -> Vec<State> {
        let mut newstates = Vec::new();
        let mut newmaterials = self.materials.clone();
        let mut add_state = |materials: Vec<Material>, elevator_pos: usize| {
            if !materials_valid(&materials) {
                return;
            }
            newstates.push(State::new(materials, elevator_pos));
        };
        for direction in [-1, 1] {
            if direction == -1 && self.elevator_pos == 0 {
                continue;
            }
            if direction == 1 && self.elevator_pos == 3 {
                continue;
            }
            let nextpos = (self.elevator_pos as isize + direction) as usize;
            for (i, material) in self.materials.iter().enumerate().filter(|(i, m)| m.on_floor(self.elevator_pos)) {
                // check if we can move the chip up (RTGs can not be moved alone)
                for (j, material2) in self.materials.iter().enumerate().skip(i).filter(|(i, m)| m.on_floor(self.elevator_pos)) {
                    // if i==j and both are chips or both are generators, we move just a single
                    // item (which is allowed)
                    if material.chip == self.elevator_pos && material2.generator == self.elevator_pos {
                        newmaterials[i].chip = nextpos;
                        newmaterials[j].generator = nextpos;
                        add_state(newmaterials, nextpos);
                        newmaterials = self.materials.clone();
                    }
                    if material.chip == self.elevator_pos && material2.chip == self.elevator_pos {
                        newmaterials[i].chip = nextpos;
                        newmaterials[j].chip = nextpos;
                        add_state(newmaterials, nextpos);
                        newmaterials = self.materials.clone();
                    }
                    if material.generator == self.elevator_pos && material2.chip == self.elevator_pos {
                        // again, i == j is possible and allowed
                        newmaterials[i].generator = nextpos;
                        newmaterials[j].chip = nextpos;
                        add_state(newmaterials, nextpos);
                        newmaterials = self.materials.clone();
                    }
                    if material.generator == self.elevator_pos && material2.generator == self.elevator_pos {
                        newmaterials[i].generator = nextpos;
                        newmaterials[j].generator = nextpos;
                        add_state(newmaterials, nextpos);
                        newmaterials = self.materials.clone();
                    }
                }
            }
        }
        newstates
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut floors = vec![];
        for floorid in (0..4).rev() {
            let mut floor = vec![];
            for material in self.materials.iter() {
                if material.chip == floorid && material.generator == floorid {
                    floor.push("X");
                } else if material.chip == floorid {
                    floor.push("M");
                } else if material.generator == floorid {
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
    let mut materials = vec![Material::new(0, 0); num_materials];
    for (floor, line) in lines.iter().enumerate() {
        for cap in re.captures_iter(line) {
            let name = cap.name("name").unwrap().as_str();
            let ntype = cap.name("type").unwrap().as_str();
            let material_id = *material_id_map.get(name).unwrap();
            if ntype == "generator" {
                materials[material_id].generator = floor;
            } else if ntype == "microchip" {
                materials[material_id].chip = floor;
            }
        }
    }
    State{materials, elevator_pos: 0}
}

fn part1(lines: &Vec<&str>) -> Option<usize> {
    let initial_state = parse_input(lines);
    let mut queue = vec![initial_state];
    let mut seen = HashSet::new();
    let mut step = 0;
    while queue.len() != 0 {
        let mut newqueue = Vec::new();
        for state in queue {
            if state.materials.iter().all(|m| m.chip == 3 && m.generator == 3) {
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
    let mut initial_state = parse_input(lines);
    // add elerium and dilithium
    initial_state.materials.push(Material::new(0, 0));
    initial_state.materials.push(Material::new(0, 0));
    let mut queue = vec![initial_state];
    let mut seen = HashSet::new();
    let mut step = 0;
    while queue.len() != 0 {
        let mut newqueue = Vec::new();
        for state in queue {
            if state.materials.iter().all(|m| m.chip == 3 && m.generator == 3) {
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
