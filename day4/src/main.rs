use lib_aoc::{input_lib, math};
use std::collections::HashMap;

type Line = [u32; 5];
type Col = Line;

struct Bingo {
    lines: [Line; 5],
    columns: [Col; 5],
    all_numbers: Vec<u32>,
}

fn main() {
    let mut content = input_lib::get_input_as_string(file!(), false);
    let bingos_string =
        content.split_off(content.find("\n").expect("unreachable"))[2..].to_string();
    let mut vec_nb: Vec<u32> = Vec::new();
    let mut map_nb: HashMap<u32, u32> = HashMap::new();
    content.split(',').for_each(|s| {
        let nb = s.parse::<u32>().expect("unreachable");
        map_nb.insert(nb, map_nb.len() as u32);
        vec_nb.push(s.parse::<u32>().expect("unreachable"));
    });
    let mut bingos: Vec<Bingo> = Vec::new();
    bingos_string.split("\n\n").for_each(|s: &str| {
        let mut new_grid = Bingo {
            lines: [[0, 0, 0, 0, 0]; 5],
            columns: [[0, 0, 0, 0, 0]; 5],
            all_numbers: Vec::new(),
        };
        s.split('\n').enumerate().for_each(|(idx_l, l)| {
            l.split_ascii_whitespace()
                .enumerate()
                .for_each(|(idx_c, elt)| {
                    let order_n = *map_nb.get(&elt.parse::<u32>().unwrap()).unwrap();
                    new_grid.lines[idx_l][idx_c] = order_n;
                    new_grid.columns[idx_c][idx_l] = order_n;
                    new_grid.all_numbers.push(order_n);
                })
        });
        bingos.push(new_grid);
    });
    let mut min_idx: usize = 0;
    let mut min_bingo: u32 = !0;
    let mut max_idx: usize = 0;
    let mut max_bingo: u32 = 0;
    bingos.iter().enumerate().for_each(|(idx, bingo)| {
        let min = math::min(
            bingo.lines.iter().fold(!0u32, |m, l| {
                math::min(m, l.iter().fold(0, |max, n| math::max(max, *n)))
            }),
            bingo.columns.iter().fold(!0u32, |m, l| {
                math::min(m, l.iter().fold(0, |max, n| math::max(max, *n)))
            }),
        );
        if min < min_bingo {
            min_bingo = min;
            min_idx = idx;
        }
        if min > max_bingo {
            max_bingo = min;
            max_idx = idx;
        }
    });
    let result_p1: u32 = (bingos[min_idx]
        .all_numbers
        .iter()
        .filter(|x| **x > min_bingo)
        .map(|x| vec_nb[*x as usize])
        .sum::<u32>())
        * vec_nb[min_bingo as usize];
    let result_p2: u32 = (bingos[max_idx]
        .all_numbers
        .iter()
        .filter(|x| **x > max_bingo)
        .map(|x| vec_nb[*x as usize])
        .sum::<u32>())
        * vec_nb[max_bingo as usize];
    println!("Result p1 is {result_p1}");
    println!("Result p2 is {result_p2}")
}
