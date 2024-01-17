#![allow(unused)]
#![allow(dead_code)]

fn checkroom(line: &str) -> i64 {
    // return 0 if not a real room, its sector ID otherwise
    let split = line.split("-").collect::<Vec<_>>();

    // checksum includes the trailing ']', but that doesn't matter
    let (roomid, checksum) = split.last().unwrap().split_once('[').unwrap();
    let checksum = checksum.chars()
                           .take_while(|c| *c != ']')
                           .map(|c| c as usize - 0x61)
                           .collect::<Vec<usize>>();
    let mut charcounts = [0isize; 26];
    for c in line.chars().take_while(|c| !c.is_numeric()) {
        if c == '-' {
            continue;
        }
        charcounts[c as usize - 0x61] += 1;
    }

    // by negating the counts and swapping counts with the char value,
    // the default sorting does exactly what we need.
    // (highest count first, sort alphabetically for ties)
    let mut charvec = charcounts.into_iter()
                                .enumerate()
                                .map(|(i, count)| (-count, i))
                                .collect::<Vec<(isize, usize)>>();
    charvec.sort_unstable();
    let calculated: Vec<usize> = charvec.into_iter().map(|(count, i)| i).take(5).collect();
    if calculated == checksum {
        return roomid.parse().unwrap();
    }
    0
}

struct Sector {
    id: i64,
    name: String
}

impl Sector {
    fn decrypt(line: &str) -> Sector {
        let split = line.split("-").collect::<Vec<_>>();
        let (sectorid, _) = split.last().unwrap().split_once('[').unwrap();
        let sectorid = sectorid.parse::<i64>().unwrap();
        let mut result = Vec::new();
        for c in line.chars().take_while(|c| !c.is_numeric()) {
            if c == '-' {
                result.push(0x20);
                continue;
            }
            let dec = ((((c as i64 - 0x61) + sectorid) % 26) + 0x61) as u8;
            result.push(dec);
        }
        Sector {id: sectorid, name: String::from_utf8(result).unwrap()}
    }
}


fn part1(lines: &Vec<&str>) -> Option<i64> {
    Some(lines.iter().map(|l| checkroom(l)).sum())
}

fn part2(lines: &Vec<&str>) -> Option<i64> {
    for line in lines {
        let sector = Sector::decrypt(line);
        if sector.name.trim() == "northpole object storage" {
            return Some(sector.id);
        }
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
    static TESTINPUT: &str = "aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]";

    #[test]
    fn test_part1() {
        let lines: Vec<&str> = TESTINPUT.lines().collect();
        assert_eq!(Some(1514), part1(&lines));
    }

    #[test]
    #[ignore]
    fn test_part2() {
        let lines: Vec<&str> = TESTINPUT.lines().collect();
        assert_eq!(Some(13337), part2(&lines));
    }
}
