use itertools::Itertools;
use lib_aoc::input_lib::{get_input_as_string, get_part};
use lib_aoc::math::{max, min};
use std::collections::HashSet;
use priority_queue::{self, PriorityQueue};

fn is_min(map: &Vec<Vec<u8>>, x: usize, y: usize) -> bool {
    for y_ in max(0, y as i64 - 1) as usize..=min(map.len() - 1, y + 1) {
        for x_ in max(0, x as i64 - 1) as usize..=min(map[0].len() - 1, x + 1) {
            if map[y_][x_] < map[y][x] {
                return false
            }
        }
    }
    true
}

fn p1(map: &Vec<Vec<u8>>, x: usize, y: usize) -> u64 {
    if is_min(map, x, y) {
        return map[y][x] as u64 + 1
    }
    0
}

fn extend_until_nine(map: &Vec<Vec<u8>>, x: i64, y: i64, visited: &mut HashSet<(i64, i64)>) -> u64 {
    if visited.insert((x, y)) &&
    x >= 0 && (x as usize) < map[0].len() &&
    y >= 0 && (y as usize) < map.len() &&
    map[y as usize][x as usize] < 9 {
        return 1 +
        extend_until_nine(map, x - 1, y, visited) +
        extend_until_nine(map, x + 1, y, visited) +
        extend_until_nine(map, x, y - 1, visited) +
        extend_until_nine(map, x , y + 1, visited)
    }
    0
}

fn get_area_size(map: &Vec<Vec<u8>>, x: usize, y: usize, pq: &mut PriorityQueue<(usize, usize), u64>) {
    if is_min(map, x, y) {
        pq.push((x, y),extend_until_nine(map, x as i64, y as i64, &mut HashSet::<(i64, i64)>::new()));
    }
}

fn main() {
    let part = get_part();
    let input = get_input_as_string(file!(), false);

    let map = input.split('\n')
    .map(|line|
        line.as_bytes().iter()
        .map(|b| b - b'0')
        .collect_vec())
    .collect_vec();

    let mut result = if part == 1 { 0u64 } else { 1u64 };
    let mut pq = PriorityQueue::<(usize, usize), u64>::new();
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            match part {
                1 => result += p1(&map, x, y),
                _ => get_area_size(&map, x, y, &mut pq)
            }
        }
    }

    if part != 1 {
        println!("{}",
        (0..3)
        .fold(1u64, |acc, _| acc * pq.pop().unwrap().1))
    }
    else {
        println!("{result}");
    }

}
