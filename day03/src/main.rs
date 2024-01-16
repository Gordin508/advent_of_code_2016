fn triangle_valid(sides: &mut [usize]) -> bool {
    sides.sort_unstable();
    sides[0] + sides[1] > sides[2]
}

fn part1(lines: &Vec<&str>) -> Option<i64> {
    let mut result = 0;
    for line in lines {
        let mut sides = line.split_whitespace().map(|w| w.parse::<usize>().unwrap()).collect::<Vec<_>>();
        assert_eq!(3, sides.len());
        if triangle_valid(&mut sides) {
            result += 1;
        }
    }
    Some(result)
}

fn part2(lines: &Vec<&str>) -> Option<i64> {
    let mut result = 0;
    for i in (0..lines.len()).step_by(3) {
        let mut sides = [[0usize; 3]; 3];
        for j in 0..3 {
            let line = &lines[i + j];
            let nums = line.split_whitespace().map(|w| w.parse::<usize>().unwrap()).collect::<Vec<_>>();
            assert_eq!(3, nums.len());
            for i in 0..3 {
                sides[i][j] = nums[i];
            }
        }
        result += sides.into_iter().filter(|s| triangle_valid(&mut s.to_owned())).count();
    }
    Some(result as i64)
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
