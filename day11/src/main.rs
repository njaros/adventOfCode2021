use itertools::Itertools;
use std::collections::VecDeque;
use lib_aoc::input_lib::{get_input_as_string, get_part};
use lib_aoc::math::{min, max};

fn proceed(map: &mut Vec<Vec<u32>>) -> u64 {

    let mut will_flash = VecDeque::<(usize, usize)>::new();
    
    map.iter_mut()
    .enumerate()
    .for_each(|(y, line)| line.iter_mut().enumerate().for_each(|(x, n)| {
        *n += 1;
        if *n == 10 {
            will_flash.push_back((x, y));
        }
    }));

    let mut flashed = 0u64;
    while !(will_flash.is_empty()) {
        flashed += 1;
        let (x, y) = will_flash.pop_front().unwrap();
        for _y in max(0, y as i64 - 1) as usize..=min(y + 1, map.len() - 1) {
            for _x in max(0, x as i64 - 1) as usize..=min(x + 1, map[0].len() - 1) {
                if _x != x || _y != y {
                    if map[_y][_x] == 9 {
                        will_flash.push_back((_x, _y));
                    }
                    map[_y][_x] += 1;
                }
            }
        }
    }

    map.iter_mut()
    .for_each(|line| line.iter_mut().for_each(|n| {
        if *n >= 10 {
            *n = 0;
        }
    }));

    flashed
}

fn main() {
    let part = get_part();
    let input = get_input_as_string(file!(), false);

    let mut map = input.split_ascii_whitespace()
    .map(|line| line.chars().map(|c| c.to_digit(10u32).unwrap()).collect_vec())
    .collect_vec();

    let mut res = 1u64;
    if part == 1 {
        res = (0..100)
        .fold(0u64, |acc, _| {
            acc + proceed(&mut map)
        });
    }
    else {
        while proceed(&mut map) != 100 {
            res += 1;
        }
    }

    println!("{res}")
}
