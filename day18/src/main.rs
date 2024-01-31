#![allow(unused)]
#![allow(dead_code)]

fn print_row(row: &[u8]) {
    let prntout = row.iter().map(|v| if *v == 0 {'.'} else {'^'}).collect::<String>();
    println!("{:}", prntout);
}

fn get_safe_tiles(input: &[u8], num_rows: usize) -> usize {
    let mut row = input.to_vec();
    let rowlen = row.len();
    let mut result = row.iter().filter(|tile| **tile == 0).count();
    for _ in (1..num_rows) {
        let mut newrow = vec![0; rowlen];
        for (i, tile) in row.iter().enumerate().filter(|(i, t)| **t != 0) {
            newrow[i] += 1;
            if i > 0 {
                newrow[i - 1] += 4;
            }
            if i < rowlen - 1 {
                newrow[i + 1] += 4;
            }
        }
        newrow.iter_mut().for_each(|v| *v = if 1 < *v && *v < 8 {1} else {0});
        result += newrow.iter().filter(|v| **v == 0).count();
        row = newrow;
    }
    result
}

fn part1(lines: &Vec<&str>) -> Option<usize> {
    let puzzle_input = lines[0].chars().map(|c| if c == '.' {0} else {1}).collect::<Vec<_>>();
    Some(get_safe_tiles(&puzzle_input, 40))
}

fn part2(lines: &Vec<&str>) -> Option<usize> {
    let puzzle_input = lines[0].chars().map(|c| if c == '.' {0} else {1}).collect::<Vec<_>>();
    Some(get_safe_tiles(&puzzle_input, 400000))
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
    static TESTINPUT: &str = ".^^.^.^^^^";

    #[test]
    fn test_part1() {
        let lines: Vec<&str> = TESTINPUT.lines().collect();
        let puzzle_input = lines[0].chars().map(|c| if c == '.' {0} else {1}).collect::<Vec<_>>();
        assert_eq!(38, get_safe_tiles(&puzzle_input, 10));
    }

    #[test]
    #[ignore]
    fn test_part2() {
        let lines: Vec<&str> = TESTINPUT.lines().collect();
        assert_eq!(Some(13337), part2(&lines));
    }
}
