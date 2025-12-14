use std::collections::HashSet;
use itertools::Itertools;
use lib_aoc::{input_lib};

fn lerp(s: ((i32, i32), (i32, i32))) -> Vec<(i32, i32)> {
    let mut points = Vec::<(i32, i32)>::new();
    let mut p = s.0;
    let step_x;
    let step_y;

    if s.0.0 < s.1.0 {
        step_x = 1;
    }
    else if s.0.0 > s.1.0 {
        step_x = -1;
    }
    else {
        step_x = 0;
    }

    if s.0.1 < s.1.1 {
        step_y = 1;
    }
    else if s.0.1 > s.1.1 {
        step_y = -1;
    }
    else {
        step_y = 0;
    }

    while p != s.1 {
        points.push(p);
        p.0 += step_x;
        p.1 += step_y;
    }
    points.push(p);
    points
}

fn main() {
    let part = input_lib::get_part();
    let input = input_lib::get_input_as_string(file!(), false);
    
    let segments = input
    .split("\n")
    .map(|line| {
        let (p1_raw, p2_raw) = line.split_once(" -> ").unwrap();
        let (x1_raw, y1_raw) = p1_raw.split_once(',').unwrap();
        let (x2_raw, y2_raw) = p2_raw.split_once(',').unwrap();
        ((x1_raw.parse::<i32>().unwrap(), y1_raw.parse::<i32>().unwrap()), ((x2_raw.parse::<i32>().unwrap()), (y2_raw.parse::<i32>().unwrap())))
    })
    .collect_vec();

    println!("Result is {}",
        segments
        .iter()
        .filter(|segment| match part {
            1 => segment.0.0 == segment.1.0 || segment.0.1 == segment.1.1,
            _ => true
        })
        .fold((HashSet::<(i32, i32)>::new(), HashSet::<(i32, i32)>::new()), |mut acc, &segment| {
            lerp(segment)
            .iter()
            .for_each(|&p| {
                if !acc.0.insert(p) {
                    acc.1.insert(p);
                }
            });
            acc
        })
        .1.len()
    )
}
