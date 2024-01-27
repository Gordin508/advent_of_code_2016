#![allow(unused)]
#![allow(dead_code)]

fn first_triplet(digest: &md5::Digest) -> Option<u8> {
    let mut lastchar = 0;
    let mut streak = 0;
    // consider only first triplet
    for c in digest.iter().flat_map(|x| [(x >> 4) & 0xf, x & 0xf].into_iter()) {
        if c == lastchar {
            streak += 1;
        } else {
            streak = 1;
        }
        if streak >= 3 {
            return Some(c);
        }
        lastchar = c;
    }
    None
}

fn quintuple(digest: &md5::Digest, searchc: u8) -> bool {
    let mut streak = 0;
    for c in digest.iter().flat_map(|x| [(x >> 4) & 0xf, x & 0xf].into_iter()) {
        if c == searchc {
            streak += 1;
        } else {
            streak = 0;
        }
        if streak >= 5 {
            return true;
        }
    }
    false
}

fn gen_keys(puzzle_input: &[u8], num_keys: usize, iterations: usize) -> usize {
    use std::collections::VecDeque;
    let mut keycache = VecDeque::new();
    let mut current_index = 0;
    let mut keys_found = 0;

    let calc_digest = |nonce: usize| {
        let mut hash_input = puzzle_input.to_vec();
        hash_input.extend(nonce.to_string().as_bytes());
        let mut digest = md5::compute(hash_input);
        for _ in (1..iterations) {
            digest = md5::compute(format!("{:x}", digest).as_bytes());
        }
        digest
    };

    // fill cache
    for i in (0..1000) {
        keycache.push_back(calc_digest(i));
    }


    while keys_found < num_keys {
        let hash = keycache.pop_front().unwrap();
        if let Some(c) = first_triplet(&hash) {
            if keycache.iter().any(|digest| quintuple(digest, c)) {
                keys_found += 1;
            }
        }
        // fill queue again
        keycache.push_back(calc_digest(current_index + 1000));
        current_index += 1;
    }
    current_index - 1
}

fn part1(lines: &Vec<&str>) -> Option<usize> {
    assert_eq!(1, lines.len());
    let puzzle_input = lines[0].bytes().collect::<Vec<_>>();
    Some(gen_keys(&puzzle_input, 64, 1))
}

fn part2(lines: &Vec<&str>) -> Option<usize> {
    assert_eq!(1, lines.len());
    let puzzle_input = lines[0].bytes().collect::<Vec<_>>();
    Some(gen_keys(&puzzle_input, 64, 2017))
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
    fn test_part1() {
        let puzzle_input = "abc".as_bytes();
        assert_eq!(22728, gen_keys(puzzle_input, 64, 1));
    }

    #[test]
    fn test_part2() {
        let puzzle_input = "abc".as_bytes();
        assert_eq!(22551, gen_keys(puzzle_input, 64, 2017));
    }
}
