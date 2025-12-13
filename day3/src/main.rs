use lib_aoc::input_lib::{get_input_as_string, get_part};

fn parse_c(c: char) -> u32 {
    match c {
        '1' => 1,
        _ => 0,
    }
}

fn found_last_line_rec(v: &mut Vec<String>, idx: usize, more: bool) -> u32 {
    match v.as_slice() {
        [] => panic!("vector can't be empty."),
        [s] => {
            let mut res = 0u32;
            s.chars()
                .rev()
                .enumerate()
                .for_each(|(pow, c)| res += parse_c(c) * 2u32.pow(pow as u32));
            res
        }
        _ => {
            let bit_to_be: char;
            let count = v.iter().fold(0, |res, s| {
                if s.chars().nth(idx).unwrap() == '1' {
                    res + 1
                } else {
                    res
                }
            });
            match more {
                true => {
                    if 2 * count >= v.len() {
                        bit_to_be = '1'
                    } else {
                        bit_to_be = '0'
                    }
                }
                false => {
                    if 2 * count >= v.len() {
                        bit_to_be = '0'
                    } else {
                        bit_to_be = '1'
                    }
                }
            }
            v.retain(|s| s.chars().nth(idx).unwrap() == bit_to_be);
            found_last_line_rec(v, idx + 1, more)
        }
    }
}

fn main() {
    let content = get_input_as_string(file!(), false);
    let part = get_part();
    let lines = content.split_whitespace();
    let mut counter: Vec<usize> = Vec::new();
    let mut epsilon: usize = 0;
    let mut gamma: usize = 0;
    let mut len: usize = 0;
    match part {
        1 => {
            for (idx, line) in lines.enumerate() {
                len += 1;
                match idx {
                    0 => line.chars().for_each(|c| counter.push(parse_c(c) as usize)),
                    _ => line
                        .chars()
                        .enumerate()
                        .for_each(|(idx_char, c)| counter[idx_char] += parse_c(c) as usize),
                }
            }
            counter.iter().rev().enumerate().for_each(|(idx, n)| {
                if *n > len / 2 {
                    epsilon += 2usize.pow(idx as u32)
                } else {
                    gamma += 2usize.pow(idx as u32)
                }
            });
            let result = gamma * epsilon;
            println!("{result}")
        }
        _ => {
            let mut oxygen: Vec<String> = Vec::new();
            let mut co2: Vec<String> = Vec::new();
            lines.for_each(|line| {
                oxygen.push(String::from(line));
                co2.push(String::from(line));
            });
            let oxygen_rate = found_last_line_rec(&mut oxygen, 0, true);
            let co2_rate = found_last_line_rec(&mut co2, 0, false);
            let result = oxygen_rate * co2_rate;
            println!("{result}")
        }
    }
}
