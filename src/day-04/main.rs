#![feature(test)]
extern crate test;

use md5::{Digest, Md5};
use rayon::prelude::*;

const INPUT: &[u8] = b"bgvyzdsv";

fn part_1(input: &[u8]) -> usize {
    (0..usize::MAX)
        .into_par_iter()
        .find_first(|i| {
            let mut hasher = Md5::new();
            hasher.update(input);
            hasher.update(i.to_string().as_bytes());
            let hash = hasher.finalize_reset();

            hash[0] == 0 && hash[1] == 0 && hash[2] < 16
        })
        .unwrap()
}

fn part_2(input: &[u8]) -> usize {
    (0..usize::MAX)
        .into_par_iter()
        .find_first(|i| {
            let mut hasher = Md5::new();
            hasher.update(input);
            hasher.update(i.to_string().as_bytes());
            let hash = hasher.finalize_reset();

            hash[0] == 0 && hash[1] == 0 && hash[2] == 0
        })
        .unwrap()
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1a() {
        assert_eq!(part_1(b"abcdef"), 609043);
    }

    #[test]
    fn test_part_1b() {
        assert_eq!(part_1(b"pqrstuv"), 1048970);
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
