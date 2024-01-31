#![allow(unused)]
#![allow(dead_code)]

fn last_elf_standing(num_elves: usize) -> usize {
    let mut num_elves = num_elves;
    let mut winning_elf = 0;
    let mut num_rounds = 1;
    while num_elves > 1 {
        let odd = num_elves % 2 != 0;
        if num_elves % 2 == 0 {
            // even
        } else {
            // odd
            winning_elf += 1 << num_rounds;
        }
        num_elves /= 2;
        num_rounds += 1;
    }
    winning_elf + 1
}

fn last_elf_standing_pt2(num_elves: usize) -> usize {
    // implement me!
    0
}

fn part1(lines: &Vec<&str>) -> Option<usize> {
    assert_eq!(1, lines.len());
    let puzzle_input = lines[0].parse::<usize>().unwrap();
    Some(last_elf_standing(puzzle_input))
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
    fn test_last_elf() {
        assert_eq!(3, last_elf_standing(5));
    }

    #[test]
    fn test_last_elf_pow2() {
        assert_eq!(1, last_elf_standing(8));
        assert_eq!(1, last_elf_standing(256));
    }

    #[test]
    fn test_last_elf_trivial() {
        assert_eq!(1, last_elf_standing(1));
        assert_eq!(1, last_elf_standing(2));
        assert_eq!(3, last_elf_standing(3));
    }

    #[test]
    fn test_last_elf_pt2() {
        assert_eq!(2, last_elf_standing_pt2(5));
    }
}
