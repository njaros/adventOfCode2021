use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut input = File::open("input")?;
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;
    println!("{}", buf);
    Ok(())
}
