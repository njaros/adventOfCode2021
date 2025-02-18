use lib_aoc::input_lib::{get_input_as_string, get_part};

fn main() -> std::io::Result<()> {
    let part = get_part();
    let file_content = get_input_as_string(file!(), false);
    let mut depth = 0;
    let mut distance = 0;
    let mut aim = 0;
    let instructions = file_content.split("\n");
    for line in instructions.into_iter() {
        let mut args = line.split(" ");
        let dir = match args.next() {
            Some(s) => s,
            None => {
                panic!("string expected")
            }
        };
        let dist = match args.next() {
            Some(s) => match s.parse::<u32>() {
                Ok(n) => n,
                Err(_) => {
                    panic!("expect a number as string")
                }
            },
            None => {
                panic!("expected a second string")
            }
        };
        match part {
            1 => {
                match dir {
                    "forward" => distance += dist,
                    "down" => depth += dist,
                    "up" => depth -= dist,
                    _ => {
                        panic!("wrong input! Unexpected {dir} value.")
                    }
                };
            }
            _ => {
                match dir {
                    "forward" => {
                        distance += dist;
                        depth += aim * dist;
                    }
                    "down" => aim += dist,
                    "up" => aim -= dist,
                    _ => {
                        panic!("wrong input! Unexpected {dir} value.")
                    }
                };
            }
        }
    }

    println!("result is {}", depth * distance);

    Ok(())
}
