#![feature(test)]
extern crate test;

use bstr::ByteSlice;

const INPUT: &[u8] = include_bytes!("input.txt");

fn part_1(input: &[u8]) -> u32 {
    let re = regex::bytes::Regex::new(r"(\d+)x(\d+)x(\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            let l = caps
                .get(1)
                .unwrap()
                .as_bytes()
                .to_str()
                .unwrap()
                .parse::<u32>()
                .unwrap();
            let w = caps
                .get(2)
                .unwrap()
                .as_bytes()
                .to_str()
                .unwrap()
                .parse::<u32>()
                .unwrap();
            let h = caps
                .get(3)
                .unwrap()
                .as_bytes()
                .to_str()
                .unwrap()
                .parse::<u32>()
                .unwrap();

            let lw = l * w;
            let wh = w * h;
            let hl = h * l;

            let min = lw.min(wh).min(hl);
            2 * (lw + wh + hl) + min
        })
        .sum()
}

fn part_2(input: &[u8]) -> u32 {
    let re = regex::bytes::Regex::new(r"(\d+)x(\d+)x(\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            let l = caps
                .get(1)
                .unwrap()
                .as_bytes()
                .to_str()
                .unwrap()
                .parse::<u32>()
                .unwrap();
            let w = caps
                .get(2)
                .unwrap()
                .as_bytes()
                .to_str()
                .unwrap()
                .parse::<u32>()
                .unwrap();
            let h = caps
                .get(3)
                .unwrap()
                .as_bytes()
                .to_str()
                .unwrap()
                .parse::<u32>()
                .unwrap();

            let max = l.max(w).max(h);
            2 * (l + w + h - max) + l * w * h
        })
        .sum()
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(b"2x3x4"), 58);
        assert_eq!(part_1(b"1x1x10"), 43);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(b"2x3x4"), 34);
        assert_eq!(part_2(b"1x1x10"), 14);
    }

    #[bench]
    fn bench_part_1(b: &mut test::Bencher) {
        b.iter(|| part_1(INPUT));
    }

    #[bench]
    fn bench_part_2(b: &mut test::Bencher) {
        b.iter(|| part_2(INPUT));
    }
}
