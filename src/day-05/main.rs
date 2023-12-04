#![feature(test)]
extern crate test;

use bstr::ByteSlice;
use std::collections::HashMap;

const INPUT: &[u8] = include_bytes!("input.txt");

fn part_1(input: &[u8]) -> usize {
    input
        .lines()
        .filter(|line| {
            let mut vowel_count = u32::from(
                line[0] == b'a'
                    || line[0] == b'e'
                    || line[0] == b'i'
                    || line[0] == b'o'
                    || line[0] == b'u',
            );
            let mut has_double = false;

            for (&a, &b) in line.iter().zip(line.iter().skip(1)) {
                if a == b {
                    has_double = true;
                }

                if b == b'a' || b == b'e' || b == b'i' || b == b'o' || b == b'u' {
                    vowel_count += 1;
                }

                if (a == b'a' && b == b'b')
                    || (a == b'c' && b == b'd')
                    || (a == b'p' && b == b'q')
                    || (a == b'x' && b == b'y')
                {
                    return false;
                }
            }

            vowel_count >= 3 && has_double
        })
        .count()
}

fn part_2(input: &[u8]) -> usize {
    input
        .lines()
        .filter(|line| {
            let has_sandwich = line.iter().zip(line.iter().skip(2)).any(|(&a, &b)| a == b);

            if !has_sandwich {
                return false;
            }

            let mut all_pairs = HashMap::new();
            for idx in 0..(line.len() - 1) {
                let pair = [line[idx], line[idx + 1]];
                if let Some(&prev_idx) = all_pairs.get(&pair) {
                    if idx - prev_idx > 1 {
                        return true;
                    }
                } else {
                    all_pairs.insert(pair, idx);
                }
            }
            false
        })
        .count()
}

fn main() {
    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[bench]
    fn bench_part_1(b: &mut test::Bencher) {
        b.iter(|| part_1(INPUT));
    }

    #[bench]
    fn bench_part_2(b: &mut test::Bencher) {
        b.iter(|| part_2(INPUT));
    }
}
