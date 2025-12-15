use lib_aoc::input_lib;
use itertools::Itertools;
use lib_aoc::math::sub_abs;

fn main() {
    let part = input_lib::get_part();
    let input = input_lib::get_input_as_string(file!(), false);

    let crabs = input.split(',')
    .map(|pos_raw| pos_raw.parse::<u64>().unwrap())
    .sorted()
    .collect_vec();

    if part == 1 {
        let dest = crabs[crabs.len() / 2];
        println!("{}", 
        crabs
        .iter()
        .fold(0u64, |acc, &n| acc + sub_abs(n, dest))
    )}
    else {
        println!("{}", 
            (*crabs.first().unwrap()..=*crabs.last().unwrap())
            .map(|n| crabs.iter().fold(0u64, |acc, &pos| {
                let dist = sub_abs(pos, n);
                acc + (dist * (dist + 1) / 2)
            }))
            .min().unwrap()
        )
    }

}
