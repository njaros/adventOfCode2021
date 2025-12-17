use lib_aoc::input_lib;
use itertools::Itertools;

fn pow_u8(a: u8, b: u8) -> u8 {
    (0..b).fold(1u8, |acc, _| acc * a)
}

fn str_to_u8(s: &str) -> u8 {
    s.as_bytes()
    .iter()
    .fold(0u8, |acc, &c| acc | pow_u8(2, c - b'a'))
}

fn u8_len(mut a: u8) -> usize {
    let mut res = 0usize;
    let mut endian = 128u8;

    while endian != 0 {
        if a >= endian {
            a -= endian;
            res += 1;
        }
        endian /= 2;
    }
    res
}

fn map_str_to_number(s: &str, digits: &[u8; 10]) -> u64 {
    for i in 0..=9 as usize {
        if str_to_u8(s) == digits[i] {
            return i as u64
        }
    }
    unreachable!("{s}")
}

fn my_take_if(nbrs: &mut Vec<u8>, predicate: impl Fn(u8) -> bool) -> u8 {
    let mut to_remove = 0usize;
    let mut res = 0u8;
    for i in 0..nbrs.len() {
        if predicate(nbrs[i]) {
            res = nbrs[i];
            to_remove = i;
            break
        }
    }
    nbrs.remove(to_remove);
    res
}

fn build_digits(line: &str) -> [u8; 10] {
    let mut digits = [0u8; 10];
    let mut to_u8 = line.split(' ').map(|s| str_to_u8(s)).collect_vec();

    digits[1] = my_take_if(&mut to_u8, |n| u8_len(n) == 2);
    digits[4] = my_take_if(&mut to_u8, |n| u8_len(n) == 4);
    digits[7] = my_take_if(&mut to_u8, |n| u8_len(n) == 3);
    digits[8] = my_take_if(&mut to_u8, |n| u8_len(n) == 7);
    digits[3] = my_take_if(&mut to_u8, |n| u8_len(n) == 5 && (digits[1] & n == digits[1]));
    digits[6] = my_take_if(&mut to_u8, |n| u8_len(n) == 6 && u8_len(digits[1] & n) == 1);
    digits[9] = my_take_if(&mut to_u8, |n| u8_len(n) == 6 && (digits[4] & n == digits[4]));
    digits[0] = my_take_if(&mut to_u8, |n| u8_len(n) == 6);
    digits[5] = my_take_if(&mut to_u8, |n| u8_len(digits[9] ^ n) == 1);
    digits[2] = to_u8.pop().unwrap();

    digits
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
        let res = input.split('\n')
        .fold(0u64, |acc, line| {
            let (first, output) = line.split_once(" | ").unwrap();
            let digits = build_digits(first);
            acc + output.split(' ').fold(0u64, |acc, s| acc * 10 + map_str_to_number(s, &digits))
        });

        println!("{res}");
    }

}
