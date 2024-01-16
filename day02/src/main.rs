#![allow(unused)]
#![allow(dead_code)]

fn part1(lines: &Vec<&str>) -> Option<i64> {
    let mut btn = 5;
    let mut result = 0;
    for line in lines {
        for c in line.chars() {
            let change = match c {
                'U' => if btn > 3 {-3} else {0},
                'D' => if btn < 7 {3} else {0},
                'L' => if btn % 3 != 1 {-1} else {0},
                'R' => if btn % 3 != 0 {1} else {0},
                _ => panic!("Unknown instruction")
            };
            btn = btn + change;
        }
        result = result * 10 + btn;
    }
    Some(result)
}

fn part2(lines: &Vec<&str>) -> Option<i64> {
    let mut btn = 5;
    let mut result = Vec::new();
    for line in lines {
        for c in line.chars() {
            let change = match c {
                'U' => if ![5,2,1,4,9].contains(&btn) {if btn != 3 && btn != 13 {-4} else {-2} } else {0},
                'D' => if ![5,10,12,13,9].contains(&btn) {if btn != 1 && btn != 11 {4} else {2}} else {0},
                'L' => if ![1,2,5,10,13].contains(&btn) {-1} else {0},
                'R' => if ![1,4,9,12,13].contains(&btn) {1} else {0},
                _ => panic!("Unknown instruction")
            };
            btn = (btn as i32 + change) as u32;
            assert!(btn > 0 && btn < 14);
        }
        result.push(char::from_u32(if btn < 10 {0x30 + btn} else {0x41 + btn - 10}).unwrap());
    }
    println!("{}", String::from_iter(result));
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
    static TESTINPUT: &str = "ULL
RRDDD
LURDL
UUUUD";

    #[test]
    fn test_part1() {
        let lines: Vec<&str> = TESTINPUT.lines().collect();
        assert_eq!(Some(1985), part1(&lines));
    }
}
