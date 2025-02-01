use std::io::prelude::*;
use lib_aoc::input_lib;

fn main() -> std::io::Result<()> {
    let part = input_lib::get_part();
    let mut input = input_lib::get_input_from_file_path(file!());
    let mut res = 0;
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;
    let mut iter = buf.split_ascii_whitespace();
    let mut previous = iter.next();
    while previous.is_some() {
        let next = iter.next();
        match next {
            Some(n) => {
                match previous {
                    Some(p) => {
                        if n.parse::<i32>().unwrap() > p.parse::<i32>().unwrap() {
                            res  = res + 1;
                        }
                    },
                    None => {},
                }
            },
            None => {},
        }
        previous = next;
    }
    println!("{}", res.to_string());
    Ok(())
}
