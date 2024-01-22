#![allow(unused)]
#![allow(dead_code)]

use regex::{Regex, Captures};
use std::cmp::max;

fn decompress(text: &str) -> String {
    // we assume that text is ascii only

    let re = Regex::new(r"\((\d+)x(\d+)\)").unwrap();
    let mut i = 0;
    let mut result = String::new();
    for mtch in re.captures_iter(text) {
        let mtch1 = mtch.get(1).unwrap();
        let mtch2 = mtch.get(2).unwrap();
        let startidx = mtch1.start() - 1;
        let endidx = mtch2.end() + 1;
        if startidx < i {
            // lies inside repeated data
            continue;
        }
        if startidx > i {
            result.push_str(&text[i..startidx]);
            i = endidx;
        }
        let letters = mtch1.as_str().parse::<usize>().unwrap();
        let repeats = mtch2.as_str().parse::<usize>().unwrap();
        for _ in (0..repeats) {
            result.push_str(&text[endidx..endidx + letters]);
        }
        i = endidx + letters;
    }
    if i < text.len() {
        result.push_str(&text[i..text.len()]);
    }
    result
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Marker {
    startidx: usize,
    endidx: usize,
    letters: usize,
    repeats: usize,
    virtualrepeats: usize,
    virtualletters: usize
}

impl Marker {
    fn new(startidx: usize, endidx: usize, letters: usize, repeats: usize) -> Marker {
        Marker { startidx, endidx, letters, repeats, virtualrepeats: repeats, virtualletters: letters}
    }
}

fn decompress_v2(text: &str) -> usize {
    // we assume that text is ascii only
    // only gets size of decompressed content
    //
    // we make some simplifying assumptions, mainly that
    // markers do no partial repeats if other markers' scopes.
    // i.e. something like (6x2)(3x3)ABC
    // would be not be correctly dealt with,
    // as we assume that the whole scope of (3x3) gets copied
    use std::collections::HashMap;

    let re = Regex::new(r"\((\d+)x(\d+)\)").unwrap();
    let mut i = text.len();
    let mut result = 0;
    let match_vec = re.captures_iter(text).collect::<Vec<_>>();
    let mut markers = Vec::new();
    for (i, mtch) in match_vec.iter().enumerate() {
        let mtch1 = mtch.get(1).unwrap();
        let mtch2 = mtch.get(2).unwrap();
        let startidx = mtch1.start() - 1;
        let endidx = mtch2.end() + 1;
        let letters = mtch1.as_str().parse::<usize>().unwrap();
        let repeats = mtch2.as_str().parse::<usize>().unwrap();
        markers.push(Marker::new(startidx, endidx, letters, repeats));
        for j in (0..i).rev() {
            if markers[j].endidx + markers[j].letters <= startidx {
                continue;
            }
            // another assumption: if marker has another marker in scope,
            // there are no terminals before the inner marker
            markers[j].virtualletters = 0;
            markers[i].virtualrepeats *= markers[j].repeats;
        }
    }
    for (j, marker) in markers.iter().enumerate() {
        result += marker.virtualrepeats * marker.virtualletters;
    }
    let mut i = 0;
    // find any unbound letters
    i = markers[0].endidx + markers[0].letters;
    result += markers[0].startidx;
    for (j, marker) in markers.iter().enumerate() {
        if marker.startidx > i {
            result += marker.startidx - i;
        }
        i = max(i, marker.endidx + marker.letters);
    }
    result
}

fn part1(lines: &Vec<&str>) -> Option<usize> {
    assert!(lines.len() == 1);
    let decompressed = decompress(lines[0]);
    Some(decompressed.chars().filter(|c| !c.is_ascii_whitespace()).count())
}

fn part2(lines: &Vec<&str>) -> Option<usize> {
    assert!(lines.len() == 1);
    Some(decompress_v2(lines[0]))
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
    fn test_decompress() {
        assert_eq!("ABBBBBC", decompress("A(1x5)BC").as_str());
        assert_eq!("ADVENT", decompress("ADVENT").as_str());
        assert_eq!("(1x3)A", decompress("(6x1)(1x3)A").as_str());
        assert_eq!("X(3x3)ABC(3x3)ABCY", decompress("X(8x2)(3x3)ABCY").as_str());
    }

    #[test]
    fn test_decompress_v2() {
        assert_eq!(9, decompress_v2("(3x3)XYZ"));
        assert_eq!(241920, decompress_v2("(27x12)(20x12)(13x14)(7x10)(1x12)A"));
    }

    #[test]
    fn test_decompress_v2_hard() {
        assert_eq!(445, decompress_v2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"));
    }
}
