/// module containing all helpers about the runtime inputs.
/// Inputs are either a file to read, or the user terminal input.
pub mod input_lib {
    use std::{
        fs::File,
        io::{stdin, stdout, Read, Write},
        path::Path,
    };

    /// Wait for user input, then returns 2 if user input start with 2.
    /// else returns 1.
    pub fn get_part() -> u32 {
        let mut input = String::new();
        print!("Type the part to execute : ");
        let _ = stdout().flush();
        match stdin().read_line(&mut input) {
            Ok(_) => match input.starts_with('2') {
                true => 2,
                false => 1,
            },
            Err(_) => 1,
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
    /// let mut input = get_input(file!(), true);
    /// let mut buffer = String::new();
    /// input.read_to_string(&mut buffer)?;
    /// println!(buffer);
    /// ```
    pub fn get_input<P: AsRef<Path>>(path: P, is_example: bool) -> File {
        let file = match is_example {
            true => "input_example",
            false => "input",
        };
        match path.as_ref().parent() {
            Some(src_folder) => match src_folder.parent() {
                Some(day_folder) => match File::open(day_folder.join(file)) {
                    Ok(file) => file,
                    Err(error) => {
                        panic!("Unable to read file: {}", error)
                    }
                },
                None => {
                    panic!("given path has no parent")
                }
            },
            None => {
                panic!("given path has no parent")
            }
        }
    }

    pub fn get_input_as_string<P: AsRef<Path>>(path: P, is_example: bool) -> String {
        let mut buf = String::new();
        let mut input = get_input(path, is_example);
        match input.read_to_string(&mut buf) {
            Ok(_) => buf,
            Err(error) => {
                panic!("can't convert input to string: {error}")
            }
        }
    }
}

pub mod math {

    /// Return the maximum between a Scalar A and a scalar B.
    pub fn max<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
        if a >= b {
            return a;
        }
        b
    }

    /// Return the minimum between a Scalar A and a scalar B.
    pub fn min<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
        if a <= b {
            return a;
        }
        b
    }
}

pub mod segment {
    use std::collections::BTreeSet;

    use crate::math;

    pub enum SegDir {
        HOR,
        VER,
        FIX,
        DIA,
        ANY,
    }
    pub struct Segment {
        pub value: [[i64; 2]; 2],
    }
    
    impl From<&str> for Segment {
        fn from(s: &str) -> Self {
            let mut seg = Segment { value: [[0; 2]; 2] };
            s.split(" -> ").enumerate().for_each(|(idx1, pair)| {
                pair.split(',').enumerate().for_each(|(idx2, n_str)| {
                    seg.value[idx1][idx2] = n_str.parse().unwrap();
                });
            });
            seg
        }
    }
    
    impl std::fmt::Display for Segment {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "({}, {}) - ({}, {})",
                self.value[0][0], self.value[0][1], self.value[1][0], self.value[1][1]
            )
        }
    }

    trait PrivateDirection {
        fn _fixed(&self) -> bool;
        fn _vertical(&self) -> bool;
        fn _horizontal(&self) -> bool;
        fn _diagonal(&self) -> bool;
    }
    
    trait PrivateUtils {
        fn _create_vec(a: [i64; 2], b: [i64; 2]) -> [i64; 2];
        fn _create_vec_3points(a: [i64; 2], b: [i64; 2], c: [i64; 2]) -> [i64; 2];
        fn _colinear_vecs(v1: [i64; 2], v2: [i64; 2]) -> bool;
    }

    pub trait Direction {
        fn start(&self) -> [i64; 2];
        fn start_x(&self) -> i64;
        fn start_y(&self) -> i64;
        fn end(&self) -> [i64; 2];
        fn end_x(&self) -> i64;
        fn end_y(&self) -> i64;
        fn fixed(&self) -> bool;
        fn vertical(&self) -> bool;
        fn horizontal(&self) -> bool;
        fn diagonal(&self) -> bool;
        fn any(&self) -> bool;
        fn get_dir(&self) -> SegDir;
        fn get_vector(&self) -> [i64; 2];
    }

    pub trait InterDiagonal {
        fn inter_d(&self, rhs: &Segment) -> bool;
        fn inter_points_d(&self, rhs: &Segment) -> BTreeSet<[i64; 2]>;
    }

    pub trait InterNoDiagonal {
        // Apply for segment which is vertical or horizontal only.
        fn colinear_nd(&self, rhs: &Segment) -> bool;
        // Apply for segment which is vertical or horizontal only.
        fn inter_nd(&self, rhs: &Segment) -> bool;
        // Apply for segment which is vertical or horizontal only.
        fn inter_points_nd(&self, rhs: &Segment) -> BTreeSet<[i64; 2]>;
    }

    impl PrivateDirection for Segment {
            
        fn _fixed(&self) -> bool {
            self.start() == self.end()
        }
    
        fn _vertical(&self) -> bool {
            self.start()[0] == self.end()[0]
        }
    
        fn _horizontal(&self) -> bool {
            self.start()[1] == self.end()[1]
        }

        fn _diagonal(&self) -> bool {
            self.end_x() - self.start_x() == self.end_y() - self.start_y()
        }
    }
    
    impl Direction for Segment {
        fn start(&self) -> [i64; 2] {
            self.value[0]
        }

        fn start_x(&self) -> i64 {
            self.start()[0]
        }

        fn start_y(&self) -> i64 {
            self.start()[1]
        }
    
        fn end(&self) -> [i64; 2] {
            self.value[1]
        }

        fn end_x(&self) -> i64 {
            self.end()[0]
        }

        fn end_y(&self) -> i64 {
            self.end()[1]
        }
    
        fn get_dir(&self) -> SegDir {
            if self._fixed() {
                return SegDir::FIX;
            }
            if self._vertical() {
                return SegDir::VER;
            }
            if self._horizontal() {
                return SegDir::HOR;
            }
            if self._diagonal() {
                return SegDir::DIA;
            }
            SegDir::ANY
        }
    
        fn fixed(&self) -> bool {
            match self.get_dir() {
                SegDir::FIX => true,
                _ => false,
            }
        }
    
        fn vertical(&self) -> bool {
            match self.get_dir() {
                SegDir::VER => true,
                _ => false,
            }
        }
    
        fn horizontal(&self) -> bool {
            match self.get_dir() {
                SegDir::HOR => true,
                _ => false,
            }
        }
    
        fn diagonal(&self) -> bool {
            match self.get_dir() {
                SegDir::DIA => true,
                _ => false,
            }
        }

        fn any(&self) -> bool {
            match self.get_dir() {
                SegDir::ANY => true,
                _ => false
            }
        }

        fn get_vector(&self) -> [i64; 2] {
            [self.end()[0] - self.start()[0], self.end()[1] - self.start()[1]]
        }

    }

    impl InterNoDiagonal for Segment {

        fn colinear_nd(&self, rhs: &Segment) -> bool {
            let v1 = self.get_vector();
            let v2 = rhs.get_vector();
            v1[0] * v2[1] - v1[1] * v2[0] == 0
        }

        fn inter_points_nd(&self, rhs: &Segment) -> BTreeSet<[i64; 2]> {
            let mut points_set: BTreeSet<[i64; 2]> = BTreeSet::new();
            if self.any() || rhs.any() { panic!("not implemented for any segment (must be vertical, horizontal or diagonal).") }
            if self.fixed() || rhs.fixed() { return points_set }
            match self.colinear_nd(rhs) {
                true => {
                    let a_x1;
                    let a_x2;
                    let b_x1;
                    let b_x2;
                    if self.horizontal() {
                        if self.start_y() != rhs.start_y() { return points_set }
                        a_x1 = math::min(self.start_x(), self.end_x());
                        a_x2 = math::max(self.start_x(), self.end_x());
                        b_x1 = math::min(rhs.start_x(), rhs.end_x());
                        b_x2 = math::max(rhs.start_x(), rhs.end_x());
                    }
                    else if self.vertical() {
                        if self.start_x() != rhs.start_x() { return points_set }
                        a_x1 = math::min(self.start_y(), self.end_y());
                        a_x2 = math::max(self.start_y(), self.end_y());
                        b_x1 = math::min(rhs.start_y(), rhs.end_y());
                        b_x2 = math::max(rhs.start_y(), rhs.end_y());
                    }
                    else {
                        
                    }
                    if !(a_x1 > b_x2) && !(a_x2 < b_x1) {
                        if a_x1 >= b_x1 {
                            for i in a_x1..(math::min(b_x2, a_x2) + 1) {
                                if self.horizontal() {
                                    points_set.insert([i, self.start_y()]);
                                }
                                else {
                                    points_set.insert([self.start_x(), i]);
                                }
                            }
                        }
                        else {
                            for i in b_x1..(math::min(b_x2, a_x2) + 1) {
                                if self.horizontal() {
                                    points_set.insert([i, self.start_y()]);
                                }
                                else {
                                    points_set.insert([self.start_x(), i]);
                                }
                            }
                        }
                    }
                    points_set
                },
                false => {
                    let v_x;
                    let h_y;
                    let x1;
                    let x2;
                    let y1;
                    let y2;
                    if self.horizontal() {
                        v_x = rhs.start_x();
                        h_y = self.start_y();
                        x1 = math::min(self.start_x(), self.end_x());
                        x2 = math::max(self.start_x(), self.end_x());
                        y1 = math::min(rhs.start_y(), rhs.end_y());
                        y2 = math::max(rhs.start_y(), rhs.end_y());
                    }
                    else {
                        v_x = self.start_x();
                        h_y = rhs.start_y();
                        x1 = math::min(rhs.start_x(), rhs.end_x());
                        x2 = math::max(rhs.start_x(), rhs.end_x());
                        y1 = math::min(self.start_y(), self.end_y());
                        y2 = math::max(self.start_y(), self.end_y());
                    }
                    if v_x >= x1 && v_x <= x2 && h_y >= y1 && h_y <= y2 {
                        points_set.insert([v_x, h_y]);
                    }
                    points_set
                }
            }
        }

        fn inter_nd(&self, rhs: &Segment) -> bool {
            self.inter_points_nd(rhs).len() != 0
        }

    }
}

pub fn add(a: i64, b: i64) -> i64 {
    a + b
}

#[cfg(test)]
mod tests {
    use crate::segment::{Direction, InterNoDiagonal, Segment};

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn segment_test() {
        let s1: Segment = Segment { value: [[9, 4], [3, 4]] };
        let s2: Segment = Segment { value: [[7, 0], [7, 4]] };
        assert_eq!(s1.start(), [9, 4]);
        assert_eq!(s1.inter_points_nd(&s2).iter().cloned().collect::<Vec<[i64; 2]>>(), vec![[7, 4]]);
    }
}
