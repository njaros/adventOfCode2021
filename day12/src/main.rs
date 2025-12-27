use std::{cell::RefCell, collections::{HashMap, HashSet}};

use itertools::Itertools;
use lib_aoc::input_lib::{get_input_as_string, get_part};

fn is_small_cave(cave: &str) -> bool {
    let char = cave.as_bytes()[0];
    char >= b'a' && char <= b'z'
}

fn dfs(graph: &HashMap<&str, RefCell<HashSet<&str>>>, current: &str, visited: &mut HashSet<String>, part: u32, twice: &mut bool) -> u64 {
    let mut twiced = false;
    if current == "end" {
        return 1
    }
    else if is_small_cave(current) {
        if !visited.insert(String::from(current)) {
            if part == 1 || *twice || current == "start" {
                return 0
            }
            if part != 1 {
                twiced = true;
                *twice = true;
            }
        }
    }

    let res = graph.get(current)
    .unwrap()
    .borrow()
    .iter()
    .fold(0u64, |acc, &cave| acc + dfs(graph, cave, visited, part, twice));
    if part == 1 || !twiced {
        visited.remove(current);
    }
    else if twiced {
        *twice = false;
    }
    res
}

fn main() {
    let part = get_part();
    let input = get_input_as_string(file!(), false);

    let graph = input
    .split_ascii_whitespace()
    .fold( HashMap::<&str, RefCell<HashSet<&str>>>::new(), |mut acc, line| {
        line.split('-')
        .permutations(2)
        .for_each(|a| {
            acc.entry(a[0])
            .and_modify(|set| {set.borrow_mut().insert(a[1]);})
            .or_insert(RefCell::<HashSet::<&str>>::new(HashSet::<&str>::from([a[1]])));
        });
        acc
    });

    println!("{}", {
        dfs(&graph, "start", &mut HashSet::<String>::new(), part, &mut false)
    })
}
