use std::{collections::LinkedList, io::prelude::*};
use lib_aoc::input_lib;

fn compare_end_tail_lst(lst: &LinkedList<u32>) -> bool {
    return lst.iter().take(3).sum::<u32>() < lst.iter().rev().take(3).sum::<u32>();
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
        }
        _ => {
            let mut lst: LinkedList<u32> = LinkedList::new();
            let mut val_read = iter.next();
            while val_read.is_some() {
                if lst.len() == 4 {
                    compare_end_tail_lst(&lst).then(|| {res += 1;});
                    lst.pop_front();
                }
                match val_read {
                    Some(s) => {
                        match s.parse::<u32>() {
                            Ok(n) => {lst.push_back(n);},
                            Err(_) => {},
                        }
                    },
                    None => {},
                }
                val_read = iter.next();
            }
            compare_end_tail_lst(&lst).then(|| {res += 1;});
        }
    }
    println!("{}", res.to_string());
    Ok(())
}
