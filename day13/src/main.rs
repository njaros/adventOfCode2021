use std::collections::HashSet;

use itertools::Itertools;
use lib_aoc::input_lib::{get_input_as_string, get_part};

fn do_fold(dots: &mut HashSet<(i32, i32)>, fold: (bool, i32)) {
    let mut to_fold = Vec::<(i32, i32)>::new();

    match fold.0 {
        true => {
            dots.iter().for_each(|&(x, y)| {
                if x >= fold.1 {
                    to_fold.push((x, y))
                }
            });
            to_fold.iter().for_each(|&(x, y)| {
                dots.remove(&(x, y));
                dots.insert((fold.1 - (x - fold.1), y));
            });
        },
        false => {
            dots.iter().for_each(|&(x, y)| {
                if y >= fold.1 {
                    to_fold.push((x, y))
                }
            });
            to_fold.iter().for_each(|&(x, y)| {
                dots.remove(&(x, y));
                dots.insert((x, fold.1 - (y - fold.1)));
            });
        }
    }
}

fn main() {

    let part = get_part();
    let input = get_input_as_string(file!(), false);

    let (dot_raw, folds_raw) = input.split_once("\n\n").unwrap();

    let mut dots = dot_raw.split_ascii_whitespace()
    .map(|pos_raw| {
        let (x_raw, y_raw) = pos_raw.split_once(',').unwrap();
        (x_raw.parse::<i32>().unwrap(), y_raw.parse::<i32>().unwrap())
    })
    .collect::<HashSet<(i32, i32)>>();

    let folds = folds_raw.split('\n')
    .map(|line| {
        let (_, fold_side_raw) = line.rsplit_once(' ').unwrap();
        let (side, pos_raw) = fold_side_raw.split_once('=').unwrap();
        (side == "x", pos_raw.parse::<i32>().unwrap())
    })
    .collect_vec();

    if part == 1 {
        do_fold(&mut dots, folds[0]);
        println!("{}", dots.len());
    }
    else {
        folds.iter().for_each(|&pouet| do_fold(&mut dots, pouet));
        let mut grid = vec![vec![' '; 50]; 10];
        dots.iter().for_each(|&(x, y)| grid[y as usize][x as usize] = '#');
        for line in grid {
            for saucisse in line {
                print!("{saucisse}");
            }
            println!();
        }
    }

}
