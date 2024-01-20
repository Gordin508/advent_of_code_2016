#![allow(unused)]
#![allow(dead_code)]

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\w+|\[\w+\])").unwrap();
}

fn has_abba(s: &str) -> bool {
    if s.len() < 4 {
        return false;
    }
    for window in (0..s.len() - 3).map(|i| &s.as_bytes()[i..i+4]) {
        assert!(window.len() == 4);
        if window[0] != window[1] && window[0] == window[3] && window[1] == window[2] {
            return true;
        }
    }
    false
}

fn add_abas(s: &str, aba_set: &mut HashSet<[u8; 3]>, bab_set: &HashSet<[u8; 3]>) -> bool {
    if s.len() < 3 {
        return false;
    }
    for window in (0..s.len() - 2).map(|i| &s.as_bytes()[i..i+3]) {
        assert!(window.len() == 3);
        if window[0] != window[1] && window[0] == window[2] {
            let new = [window[0], window[1], window[0]];
            if bab_set.contains(&new) {
                return true;
            }
            aba_set.insert(new);
        }
    }
    false
}

fn add_babs(s: &str, aba_set: &HashSet<[u8; 3]>, bab_set: &mut HashSet<[u8; 3]>) -> bool {
    if s.len() < 3 {
        return false;
    }
    for window in (0..s.len() - 2).map(|i| &s.as_bytes()[i..i+3]) {
        assert!(window.len() == 3);
        if window[0] != window[1] && window[0] == window[2] {
            let new = [window[1], window[0], window[1]]; // inverted
            if aba_set.contains(&new) {
                return true;
            }
            bab_set.insert(new);
        }
    }
    false
}

fn supports_ssl(line: &str) -> bool {
    let mut aba_set = HashSet::new();
    let mut bab_set = HashSet::new();
    for capture in RE.captures_iter(line) {
        let captured_string = &capture[1];
        let hypernet = captured_string.starts_with('[');
        let string_slice = if !hypernet {captured_string} else {&captured_string[1..captured_string.len() - 1]};
        if (!hypernet && add_abas(string_slice, &mut aba_set, &bab_set))
            || (hypernet && add_babs(string_slice, &aba_set, &mut bab_set)) {
            return true;
        }
    }
    false
}

fn supports_tls(line: &str) -> bool {
    let mut result = false;
    for capture in RE.captures_iter(line) {
        let captured_string = &capture[1];
        let hypernet = captured_string.starts_with('[');
        let string_slice = if !hypernet {captured_string} else {&captured_string[1..captured_string.len() - 1]};
        let abba = has_abba(string_slice);
        if abba && hypernet {
            return false;
        } else if abba {
            result = true;
        }
    }
    result
}

fn part1(lines: &Vec<&str>) -> Option<usize> {
    Some(lines.iter().filter(|l| supports_tls(l)).count())
}

fn part2(lines: &Vec<&str>) -> Option<usize> {
    Some(lines.iter().filter(|l| supports_ssl(l)).count())
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
    fn test_has_abba() {
        assert!(has_abba("abba"));
    }

    #[test]
    fn test_has_abba_middle() {
        assert!(has_abba("asdfasdxzzxhtper"));
        assert!(has_abba("asdfasdxzzx"));
        assert!(has_abba("xzzxhtper"));
    }

    #[test]
    fn test_has_no_abba() {
        assert!(!has_abba("abbb"));
        assert!(!has_abba("aaaa"));
    }

    #[test]
    fn test_tls_support() {
        assert!(supports_tls("abba[mnop]qrst"));
        assert!(!supports_tls("abcd[bddb]xyyx"));
        assert!(!supports_tls("aaaa[qwer]tyui"));
        assert!(supports_tls("ioxxoj[asdfgh]zxcvbn"));
    }

    #[test]
    fn test_ssl_support() {
        assert!(supports_ssl("aba[bab]xyz"));
        assert!(!supports_ssl("xyx[xyx]xyx"));
        assert!(supports_ssl("aaa[kek]eke"));
        assert!(supports_ssl("zazbz[bzb]cdb"));
    }
}
