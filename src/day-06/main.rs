#![feature(test)]
extern crate test;

use bstr::ByteSlice;
use itertools::Itertools;

const INPUT: &[u8] = include_bytes!("input.txt");

fn parse_coord(coord: &[u8]) -> [usize; 2] {
    coord
        .split_str(",")
        .map(|x| unsafe { x.to_str_unchecked() }.parse::<usize>().unwrap())
        .collect_vec()
        .try_into()
        .unwrap()
}

fn part_1(input: &[u8]) -> usize {
    input
        .lines()
        .fold(vec![vec![false; 1000]; 1000], |mut lights, line| {
            let words = line.split_str(" ").collect_vec();
            let [from_x, from_y] = parse_coord(words[words.len() - 3]);
            let [to_x, to_y] = parse_coord(words[words.len() - 1]);

            match words[0] {
                b"turn" if words[1] == b"on" => {
                    lights.iter_mut().take(to_x + 1).skip(from_x).for_each(|row| {
                        row.iter_mut().take(to_y + 1).skip(from_y).for_each(|light| {
                            *light = true;
                        })
                    })
                }
                b"turn" if words[1] == b"off" => {
                    lights.iter_mut().take(to_x + 1).skip(from_x).for_each(|row| {
                        row.iter_mut().take(to_y + 1).skip(from_y).for_each(|light| {
                            *light = false;
                        })
                    })
                }
                b"toggle" => {
                    lights.iter_mut().take(to_x + 1).skip(from_x).for_each(|row| {
                        row.iter_mut().take(to_y + 1).skip(from_y).for_each(|light| {
                            *light = !*light;
                        })
                    })
                }
                _ => unreachable!(),
            }
            lights
        })
        .into_iter()
        .flatten()
        .filter(|&light| light)
        .count()
}

fn part_2(input: &[u8]) -> u32 {
    input
        .lines()
        .fold(vec![vec![0_u32; 1000]; 1000], |mut lights, line| {
            let words = line.split_str(" ").collect_vec();
            let [from_x, from_y] = parse_coord(words[words.len() - 3]);
            let [to_x, to_y] = parse_coord(words[words.len() - 1]);

            match words[0] {
                b"turn" if words[1] == b"on" => {
                    lights.iter_mut().take(to_x + 1).skip(from_x).for_each(|row| {
                        row.iter_mut().take(to_y + 1).skip(from_y).for_each(|light| {
                            *light += 1;
                        })
                    })
                }
                b"turn" if words[1] == b"off" => {
                    lights.iter_mut().take(to_x + 1).skip(from_x).for_each(|row| {
                        row.iter_mut().take(to_y + 1).skip(from_y).for_each(|light| {
                            *light = light.saturating_sub(1);
                        })
                    })
                }
                b"toggle" => {
                    lights.iter_mut().take(to_x + 1).skip(from_x).for_each(|row| {
                        row.iter_mut().take(to_y + 1).skip(from_y).for_each(|light| {
                            *light += 2;
                        })
                    })
                }
                _ => unreachable!(),
            }
            lights
        })
        .into_iter()
        .flatten()
        .sum()
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
