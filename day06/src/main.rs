#![allow(unused)]
#![allow(dead_code)]

fn part1(lines: &Vec<&str>) -> Option<String> {
    let pass_len = lines[0].len();
    let mut pass = Vec::new();
    for i in (0..pass_len) {
        let mut frequencies = [0usize; 26];
        for c in lines.iter().map(|l| l.as_bytes()[i]) {
            frequencies[c as usize - 0x61] += 1;
        }
        pass.push(frequencies.into_iter()
                             .enumerate()
                             .max_by(|(i, x), (i2, x2)| x.cmp(x2))
                             .unwrap().0 as u8 + 0x61);
    }

    Some(pass.iter().map(|x| *x as char).collect())
}

fn part2(lines: &Vec<&str>) -> Option<String> {
    let pass_len = lines[0].len();
    let mut pass = Vec::new();
    for i in (0..pass_len) {
        let mut frequencies = [0usize; 26];
        for c in lines.iter().map(|l| l.as_bytes()[i]) {
            frequencies[c as usize - 0x61] += 1;
        }
        pass.push(frequencies.into_iter()
                             .enumerate()
                             .filter(|(i, x)| *x > 0)
                             .min_by(|(i, x), (i2, x2)| x.cmp(x2))
                             .unwrap().0 as u8 + 0x61);
    }

    Some(pass.iter().map(|x| *x as char).collect())
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
    static TESTINPUT: &str = "eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar";

    #[test]
    fn test_part1() {
        let lines: Vec<&str> = TESTINPUT.lines().collect();
        assert_eq!(Some("easter".to_string()), part1(&lines));
    }

    #[test]
    #[ignore]
    fn test_part2() {
        let lines: Vec<&str> = TESTINPUT.lines().collect();
        assert_eq!(Some("replaceme".to_string()), part2(&lines));
    }
}
