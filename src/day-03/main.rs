#![feature(test)]
extern crate test;

use std::collections::HashSet;

const INPUT: &[u8] = include_bytes!("input.txt");

fn part_1(input: &[u8]) -> usize {
    input
        .iter()
        .scan((0, 0), |pos, &c| {
            let (x, y) = *pos;
            *pos = match c {
                b'>' => (x + 1, y),
                b'<' => (x - 1, y),
                b'^' => (x, y + 1),
                b'v' => (x, y - 1),
                _ => unreachable!("Invalid input"),
            };
            Some(*pos)
        })
        .chain(std::iter::once((0, 0)))
        .collect::<HashSet<_>>()
        .len()
}

fn part_2(input: &[u8]) -> usize {
    let (santa, robot) =
        rayon::join(
            || {
                input
                    .iter()
                    .step_by(2)
                    .scan((0, 0), |pos, &c| {
                        let (x, y) = *pos;
                        *pos = match c {
                            b'>' => (x + 1, y),
                            b'<' => (x - 1, y),
                            b'^' => (x, y + 1),
                            b'v' => (x, y - 1),
                            _ => unreachable!("Invalid input"),
                        };
                        Some(*pos)
                    })
                    .chain(std::iter::once((0, 0)))
                    .collect::<HashSet<_>>()
            },
            || {
                input
                    .iter()
                    .skip(1)
                    .step_by(2)
                    .scan((0, 0), |pos, &c| {
                        let (x, y) = *pos;
                        *pos = match c {
                            b'>' => (x + 1, y),
                            b'<' => (x - 1, y),
                            b'^' => (x, y + 1),
                            b'v' => (x, y - 1),
                            _ => unreachable!("Invalid input"),
                        };
                        Some(*pos)
                    })
                    .collect::<HashSet<_>>()
            },
        );

    santa.union(&robot).count()
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
        assert_eq!(part_1(b">"), 2);
        assert_eq!(part_1(b"^>v<"), 4);
        assert_eq!(part_1(b"^v^v^v^v^v"), 2);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(b"^v"), 3);
        assert_eq!(part_2(b"^>v<"), 3);
        assert_eq!(part_2(b"^v^v^v^v^v"), 11);
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
