use lib_aoc::input_lib;
use std::{collections::LinkedList, io::prelude::*};

fn compare_end_tail_lst(lst: &LinkedList<u32>) -> bool {
    lst.iter().take(3).sum::<u32>() < lst.iter().rev().take(3).sum::<u32>()
}

fn main() -> std::io::Result<()> {
    let part = input_lib::get_part();
    let mut input = input_lib::get_input(file!(), false);
    let mut res = 0;
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;
    let mut iter = buf.split_ascii_whitespace();
    match part {
        1 => {
            let mut previous = iter.next();
            while previous.is_some() {
                let next = iter.next();
                if let Some(n) = next {
                    if let Some(p) = previous {
                        if n.parse::<i32>().unwrap() > p.parse::<i32>().expect("unreachable") {
                            res += 1;
                        }
                    }
                }
                previous = next;
            }
        }
        _ => {
            let mut lst: LinkedList<u32> = LinkedList::new();
            let mut val_read = iter.next();
            while val_read.is_some() {
                if lst.len() == 4 {
                    compare_end_tail_lst(&lst).then(|| {
                        res += 1;
                    });
                    lst.pop_front();
                }
                if let Some(v) = val_read {
                    lst.push_back(v.parse::<u32>().expect("unreachable"))
                }
                val_read = iter.next();
            }
            compare_end_tail_lst(&lst).then(|| {
                res += 1;
            });
        }
    }
    println!("{}", res);
    Ok(())
}
