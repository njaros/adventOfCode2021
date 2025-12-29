use std::collections::HashMap;

use itertools::Itertools;
use lib_aoc::input_lib::{get_input_as_string, get_part};

type Sensus = HashMap<u8, u64>;

fn sensus_union(a: &mut Sensus, b: &Sensus) {
    b.iter().for_each(|(&k, &v)| {
        a.entry(k)
        .and_modify(|val| *val += v)
        .or_insert(v);
    })
}

fn dig(recipes: &HashMap<(u8, u8), u8>, cache: &mut HashMap<((u8, u8), u32), Sensus>, current: (u8, u8), goal: u32) -> Sensus {
    match cache.get(&(current, goal)) {
        Some(cached) => cached.clone(),
        _ => {
            let mut sensus = Sensus::new();
            match recipes.get(&current) {
                Some(&letter) => {
                    sensus.insert(letter, 1);
                    if goal > 0 {
                        sensus_union(&mut sensus, &dig(recipes, cache, (current.0, letter), goal - 1));
                        sensus_union(&mut sensus, &dig(recipes, cache, (letter, current.1), goal - 1));
                    }
                }
                None => {}
            }
            cache.insert((current, goal), sensus.clone());
            sensus
        }
    }
}

fn main() {

    let part = get_part();
    let input = get_input_as_string(file!(), false);

    let (current_raw, recipes_raw) = input.split_once("\n\n").unwrap();
    let current = current_raw.as_bytes();

    let recipes = recipes_raw.split('\n')
    .map(|line| {
        let (left_raw, right_raw) = line.split_once(" -> ").unwrap();
        let left = left_raw.as_bytes();
        ((left[0], left[1]), right_raw.as_bytes()[0])
    })
    .collect::<HashMap<(u8, u8), u8>>();

    let mut sensus = Sensus::new();
    let mut cache = HashMap::<((u8, u8), u32), Sensus>::new();

    for &letter in current {
        sensus.entry(letter).and_modify(|n| *n += 1).or_insert(1);
    }

    current.iter().tuple_windows()
    .for_each(|(&a, &b)| {
        sensus_union(&mut sensus, &dig(&recipes, &mut cache, (a, b), match part {1 => 9, _ => 39}));
    });

    println!("{}", sensus.values().max().unwrap() - sensus.values().min().unwrap());

}
