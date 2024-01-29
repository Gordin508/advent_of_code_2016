#![allow(unused)]
#![allow(dead_code)]

fn dragon_curve(initial_state: &[u8], required_len: usize) -> Vec<u8> {
    let mut state = initial_state.to_vec();
    while state.len() < required_len {
        let mut b: Vec<_> = state.iter().rev().map(|i| if *i > 0 {0} else {1}).collect();
        state.push(0);
        state.append(&mut b);
    }
    state
}

fn checksum(state: &[u8], limit_len: Option<usize>) -> Vec<u8> {
    let limit_len = match limit_len {
        Some(limit) => limit,
        None => state.len()
    };
    let calc_checksum = |array: &[u8]| -> Vec<u8> {
        let mut newchecksum = Vec::new();
        for (x, y) in array.iter().step_by(2).zip(array.iter().skip(1).step_by(2)) {
            newchecksum.push(if *x == *y {1} else {0});
        }
        newchecksum
    };

    let mut checksum = calc_checksum(&state[..limit_len]);
    while checksum.len() % 2 == 0 {
        checksum = calc_checksum(&checksum);
    }
    checksum
}

fn part1(lines: &Vec<&str>) -> Option<String> {
    assert_eq!(1, lines.len());
    let initial_state = lines[0].chars().map(|c| if c == '0' {0} else {1}).collect::<Vec<_>>();
    let target_len = 272;
    let checksum = checksum(&dragon_curve(&initial_state, target_len), Some(target_len));
    let result = checksum.into_iter().map(|v| v.to_string()).collect::<Vec<_>>();
    Some(result.join(""))
}

fn part2(lines: &Vec<&str>) -> Option<String> {
    assert_eq!(1, lines.len());
    let initial_state = lines[0].chars().map(|c| if c == '0' {0} else {1}).collect::<Vec<_>>();
    let target_len = 35651584;
    let checksum = checksum(&dragon_curve(&initial_state, target_len), Some(target_len));
    let result = checksum.into_iter().map(|v| v.to_string()).collect::<Vec<_>>();
    Some(result.join(""))
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
    fn test_dragon_curve() {
        assert_eq!(vec![0, 0, 1], dragon_curve(&[0], 3));
        assert_eq!(vec![1, 0, 0], dragon_curve(&[1], 3));
        assert_eq!(vec![1,1,1,1,0,0,0,0,1,0,1,0,0,1,0,1,0,1,1,1,1,0,0,0,0], dragon_curve(&[1,1,1,1,0,0,0,0,1,0,1,0], 13));
    }

}
