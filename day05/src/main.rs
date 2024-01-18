#![allow(unused)]
#![allow(dead_code)]

fn crack_pass(puzzle_input: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    for nonce in (0..) { 
        let mut input = puzzle_input.to_vec();
        input.extend(nonce.to_string().as_bytes());
        let digest = md5::compute(&input);
        let mut prefix = digest[0..2].iter().fold(0u8, |acc, &x| acc | x);
        prefix |= digest[2] & 0xf0;
        if prefix == 0 {
            result.push(digest[2] & 0x0f);
            if result.len() >= 8 {
                return result;
            }
        }
    }
    result
}

fn crack_pass_pt2(puzzle_input: &[u8]) -> [u8; 8] {
    let mut result = [255; 8];
    let mut num_found = 0;
    for nonce in (0..) { 
        let mut input = puzzle_input.to_vec();
        input.extend(nonce.to_string().as_bytes());
        let digest = md5::compute(&input);
        let mut prefix = digest[0..2].iter().fold(0u8, |acc, &x| acc | x);
        prefix |= digest[2] & 0xf0;
        if prefix == 0 {
            let position = digest[2] & 0x0f;
            if position > 7 || result[position as usize] != 255 {
                continue;
            }
            let chr = digest[3] >> 4;
            num_found += 1;
            result[position as usize] = chr;
            if num_found >= 8 {
                return result;
            }
        }
    }
    result
}

fn part1(lines: &Vec<&str>) -> Option<String> {
    let puzzle_input = lines[0].as_bytes();
    let pass_u8 = crack_pass(&puzzle_input);
    let tochr = |x: u8| -> char {
        assert!(x < 16);
        if x < 10 {
            return (x + 0x30) as char;
        } else {
            return (x - 10 + 0x61) as char;
        }
    };
    let pass: String = pass_u8.into_iter().map(tochr).collect();
    Some(pass)
}

fn part2(lines: &Vec<&str>) -> Option<String> {
    let puzzle_input = lines[0].as_bytes();
    let pass_u8 = crack_pass_pt2(&puzzle_input);
    let tochr = |x: u8| -> char {
        assert!(x < 16);
        if x < 10 {
            return (x + 0x30) as char;
        } else {
            return (x - 10 + 0x61) as char;
        }
    };
    let pass: String = pass_u8.into_iter().map(tochr).collect();
    Some(pass)
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
