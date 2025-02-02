pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// module containing all helpers about the runtime inputs.
/// Inputs are either a file to read, or the user terminal input.
pub mod input_lib {
    use std::{fs::File, io::{stdin, stdout, Write}, path::Path};

    /// Wait for user input, then returns 2 if user input start with 2.
    /// else returns 1.
    pub fn get_part() -> u32 {
        let mut input = String::new();
        print!("Type the part to execute : ");
        let _ = stdout().flush();
        match stdin().read_line(&mut input) {
            Ok(_) => {
                match input.starts_with('2') {
                    true => {
                        return 2;
                    }
                    false => {
                        return 1;
                    }
                }
            }
            Err(_) => {
                return 1
            }
        }
    }

    /// if the parent of the parent of the path contain
    /// a file named "input", it return his File object.
    /// If a parent is None or the file doesn't exists, it panics !
    /// if is_example parameter is true, the file "input_example"
    /// will be opened.
    /// 
    /// # Example
    /// 
    /// ```
    /// let input = get_input(file!());
    /// ```
    pub fn get_input<P: AsRef<Path>>(path: P, is_example: bool) -> File {
        let file = match is_example {
            true => {"input_example"}
            false => {"input"}
        };
        match path.as_ref().parent() {
            Some(src_folder) => {
                match src_folder.parent() {
                    Some(day_folder) => {
                        match File::open(day_folder.join(file)) {
                            Ok(file) => { return file }
                            Err(error) => { panic!("Unable to read file: {}", error) }
                        }
                    }
                    None => { panic!("given path as no parent") }
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
