pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// module containing all helpers about the runtime inputs.
/// Inputs are either a file to read, or the user terminal input.
pub mod input_lib {
    use std::{fs::File, io::{stdin, stdout, Write}, path::Path};
    use crate::math::min;

    /// Wait for user input, then returns 2 if user types "2"
    /// or 1 if user types anything else.
    pub fn get_part() -> u32 {
        let mut input = String::new();
        print!("Type the part to execute : ");
        let _ = stdout().flush();
        match stdin().read_line(&mut input) {
            Ok(_) => {
                match input.parse::<u32>() {
                    Ok(n) => {
                        return min(n, 2);
                    }
                    Err(_) => {
                        return 1;
                    }
                }
            }
            Err(_) => {
                1
            }
        }
    }

    pub fn get_input_from_file_path<P: AsRef<Path>>(path: P) -> File {
        match path.as_ref().parent() {
            Some(folder) => {
                match File::open(folder.join("input")) {
                    Ok(file) => { return file }
                    Err(error) => { panic!("Unable to read file: {}", error) }
                }
            },
            None => { panic!("given path as no parent") }
        }
    }

}

pub mod math {

    /// Return the maximum between a Scalar A and a scalar B.
    pub fn max<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
        if a >= b {
            return a
        }
        return b
    }

    /// Return the minimum between a Scalar A and a scalar B.
    pub fn min<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
        if a <= b {
            return a
        }
        return b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
