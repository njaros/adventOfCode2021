use lib_aoc::input_lib;
use itertools::Itertools;
use std::collections::HashMap;

enum DisplayPart {
    Top,
    Mid,
    Bot,
    TopR,
    TopL,
    BotR,
    BotL
}

type Display = HashMap<u8, DisplayPart>;

impl Display {

}

fn main() {
    let part = input_lib::get_part();
    let input = input_lib::get_input_as_string(file!(), false);

    if part == 1 {
        let res = input
        .split('\n')
        .fold(0u64, |acc, line| {
            acc + line.split_once(" | ").unwrap().1.split(" ").fold(0u64, |acc, output| {
                match output.len() {
                    2 | 3 | 4 | 7 => acc + 1,
                    _ => acc
                }
            })
        });
        
        println!("{res}");
    }
    else {

    }

}
