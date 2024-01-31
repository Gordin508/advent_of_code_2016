#![allow(unused)]
#![allow(dead_code)]

fn last_elf_standing_josephus(num_elves: usize) -> usize {
    // Josephus problem
    // https://www.youtube.com/watch?v=uCsD3ZGzMgE
    let msb = num_elves.ilog2();
    ((num_elves ^ (1 << msb)) << 1) + 1
}

fn last_elf_standing(num_elves: usize) -> usize {
    // my original logarithmic solution
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

fn last_elf_standing_pt2_brute_force(num_elves: usize) -> usize {
    // implement me!
    let mut elves = vec![1u8; num_elves];
    let mut num_elves = num_elves;
    let mut idx: usize = 0;
    while num_elves > 1 {
        let skip = num_elves / 2;
        let mut skipped: usize = 0;
        let mut index: usize = idx;
        loop {
            skipped += elves[index] as usize;
            if skipped > skip {
                break;
            }
            index = (index + 1) % elves.len();
        }
        elves[index] = 0;
        num_elves -= 1;
        idx = (idx + 1) % elves.len();
        while (elves[idx] == 0) {
            idx = (idx + 1) % elves.len();
        }
    }
    elves.into_iter().enumerate().filter(|(i, elf)| *elf == 1).next().unwrap().0 + 1
}

fn last_elf_standing_pt2(num_elves: usize) -> usize {
    if num_elves < 6 {
        return last_elf_standing_pt2_brute_force(num_elves);
    }
    let mut root = 1;
    let mut three_pow = 3;
    loop {
        let newpow = three_pow * 3;
        if newpow > num_elves {
            break;
        }
        three_pow = newpow;
        root += 2;
    }
    // you can find this pattern by bruteforcing the first 100 solutions
    // and manually inspecting them
    if num_elves == three_pow {
        return num_elves;
    }
    if (num_elves - three_pow) < three_pow {
        return num_elves - three_pow;
    } else {
        let next_three_pow = 3 * three_pow;
        return (num_elves - (next_three_pow - num_elves));
    }
}

fn part1(lines: &Vec<&str>) -> Option<usize> {
    assert_eq!(1, lines.len());
    let puzzle_input = lines[0].parse::<usize>().unwrap();
    Some(last_elf_standing_josephus(puzzle_input))
}

fn part2(lines: &Vec<&str>) -> Option<usize> {
    assert_eq!(1, lines.len());
    let puzzle_input = lines[0].parse::<usize>().unwrap();
    Some(last_elf_standing_pt2(puzzle_input))
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
    fn test_last_elf_numberphile() {
        assert_eq!(3, last_elf_standing_josephus(5));
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
        assert_eq!(5, last_elf_standing_pt2(7));
        assert_eq!(1, last_elf_standing_pt2_brute_force(10));
        assert_eq!(1, last_elf_standing_pt2(10));
    }
}
