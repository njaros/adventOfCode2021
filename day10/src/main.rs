use std::ops::ControlFlow;

use lib_aoc::input_lib::{get_input_as_string};

fn get_illegal_value(c: char) -> u64 {
    match c {
        ']' => 57,
        '>' => 25137,
        '}' => 1197,
        ')' => 3,
        _ => unreachable!()
    }
}

fn not_good_closure(open: char, close: char) -> bool {
    match open {
        '[' => close != ']',
        '(' => close != ')',
        '{' => close != '}',
        '<' => close != '>',
        _ => unreachable!()
    }
}

fn main() {
    let input = get_input_as_string(file!(), false);

    let mut incompletes = Vec::<Vec<char>>::new();

    println!("part 1: {}",
        input.split_whitespace()
        .fold(0u64, |acc, line| {
            let mut stack = Vec::<char>::new();
            let illegal = line.chars()
            .try_for_each(|c| match c {
                '[' | '{' | '<' | '(' => {
                    stack.push(c);
                    ControlFlow::Continue(())
                },
                close => match stack.pop() {
                    None => return ControlFlow::Break(get_illegal_value(close)),
                    Some(open) => if not_good_closure(open, close) {
                        ControlFlow::Break(get_illegal_value(close))
                    } else {
                        ControlFlow::Continue(())
                    }
                }
            });

            match illegal {
                ControlFlow::Break(n) => acc + n,
                _ => {
                    incompletes.push(stack.clone());
                    acc
    }}})); //Part1 + incompletes built

    let mut scores = incompletes.iter_mut()
    .fold(Vec::<u64>::new(), |mut acc, s| {
        let mut score = 0u64;
        while !s.is_empty() {
            score *= 5;
            score += match s.pop().unwrap() {
                '[' => 2,
                '(' => 1,
                '{' => 3,
                '<' => 4,
                _ => unreachable!()
            };
        }
        acc.push(score);
        acc
    });
    scores.sort();
    println!("part 2: {}", scores[scores.len() / 2])
}
