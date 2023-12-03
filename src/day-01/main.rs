#![feature(test)]
extern crate test;

const INPUT: &[u8] = include_bytes!("input.txt");

fn part_1(input: &[u8]) -> i32 {
    input
        .iter()
        .map(|&c| match c {
            b'(' => 1,
            b')' => -1,
            _ => unreachable!("Invalid input"),
        })
        .sum()
}

fn part_2(input: &[u8]) -> usize {
    input
        .iter()
        .scan(0, |floor, &c| {
            *floor += match c {
                b'(' => 1,
                b')' => -1,
                _ => unreachable!("Invalid input"),
            };
            Some(*floor)
        })
        .position(|floor| floor == -1)
        .unwrap()
        + 1
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
