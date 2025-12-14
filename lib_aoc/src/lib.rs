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
            Ok(_) => {
                buf.retain(|c| c != '\r');
                buf
            },
            Err(error) => {
                panic!("can't convert input to string: {error}")
            }
        }
    }
}

/**
 * Some algebric tools
 */
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

    /// Reorder two comparable values
    pub fn reorder_pair<T: std::cmp::PartialOrd>(a: T, b: T) -> (T, T) {
        if a <= b {
            return (a, b);
        }
        (b, a)
    }

    pub fn sub_abs<T: std::cmp::PartialOrd + std::ops::Sub + std::ops::Sub<Output = T>>(a: T, b: T) -> T {        
        if a <= b {
            return b - a;
        }
        a - b
    }

    pub fn u64_pow(a: u64, mut b: u64) -> u64 {
        let mut res: u64 = 1;
        while b != 0 {
            res *= a;
            b -= 1;
        }
        res
    }
    
    pub mod distance {
        use std::iter::IntoIterator;

        //doesn't work atm for unknown reason
        pub fn manhattan<'a, T, I1, I2>(p1: I1, p2: I2) -> T
        where 
            I1: IntoIterator<Item = &'a T>,
            I2: IntoIterator<Item = &'a T>,
            T: 'a + std::iter::IntoIterator + Copy + std::iter::Sum + std::cmp::PartialOrd + std::ops::Sub + std::ops::Sub<Output = T>
        {
            p1.into_iter()
            .zip(p2)
            .map(|(a, b)| super::sub_abs(*a, *b))
            .sum()
        }
    }
}

/**
 * Tools for lines in usual vectorial spaces
 */
pub mod lines {
    use crate::math::reorder_pair;


    // This variable may need to be higher, for now I don't know.
    pub const ESPILON: f64 = f64::EPSILON;

    #[derive(Debug, Clone, Copy)]
    pub struct Point2d {
        pub x: f64,
        pub y: f64
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Segment2d {
        pub p1: Point2d,
        pub p2: Point2d,
        pub a: f64,
        pub b: f64
    }

    // Usefull vectorial tools

    type Vector2d = Point2d;

    pub fn do_vector(a: Point2d, b: Point2d) -> Vector2d {
        Vector2d{x: b.x - a.x, y: b.y - a.y}
    }

    pub fn dot_product(v1: Vector2d, v2: Vector2d) -> f64 {
        v1.x * v2.y - v1.y * v2.x
    }

    // Traits/Impl for Point

    // Traits/Impl for Vector

    impl std::ops::Add for Vector2d {

        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Vector2d{x: self.x + rhs.x, y: self.y + rhs.y}
        }
    }

    impl std::ops::AddAssign for Vector2d {
        fn add_assign(&mut self, rhs: Self) {
            self.x += rhs.x;
            self.y += rhs.y;
        }
    }

    impl std::ops::Sub for Vector2d {

        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Vector2d{x: self.x - rhs.x, y: self.y - rhs.y}
        }
    }

    impl std::ops::SubAssign for Vector2d {
        fn sub_assign(&mut self, rhs: Self) {
            self.x -= rhs.x;
            self.y -= rhs.y;
        }
    }

    impl std::ops::Mul<f64> for Vector2d {

        type Output = Self;

        fn mul(self, rhs: f64) -> Self::Output {
            Vector2d{x: self.x * rhs, y: self.y * rhs}
        }
    }

    impl std::ops::MulAssign<f64> for Vector2d {
        fn mul_assign(&mut self, rhs: f64) {
            self.x *= rhs;
            self.y *= rhs;
        }
    }

    impl std::ops::Div<f64> for Vector2d {

        type Output = Self;

        fn div(self, rhs: f64) -> Self::Output {
            Vector2d{x: self.x / rhs, y: self.y / rhs}
        }
    }

    impl std::ops::DivAssign<f64> for Vector2d {
        fn div_assign(&mut self, rhs: f64) {
            self.x /= rhs;
            self.y /= rhs;
        }
    }

    impl std::cmp::PartialEq for Vector2d {
        fn eq(&self, other: &Self) -> bool {
            self.x == other.x && self.y == other.y
        }

        fn ne(&self, other: &Self) -> bool {
            !(self == other)
        }
    }

    // Traits/impl for Segment

    trait LineDir {

        fn get_line_coefficients(p1: Point2d, p2: Point2d) -> (f64, f64);
        fn get_leading_coefficient(p1: Point2d, p2: Point2d) -> f64;
        fn get_y_intercept(a: f64, p: Point2d) -> f64;
        
    }

    pub trait GetPoint {

        fn get_y_from_x(&self, x: f64) -> f64;
        fn get_x_from_y(&self, y: f64) -> f64;
        fn get_point_from_x(&self, x: f64) -> Point2d;
        fn get_point_from_y(&self, y: f64) -> Point2d;
        fn contain_point(&self, p: Point2d) -> bool;

    }

    pub trait Intercept<L, P> {

        /**
         * Get the interception point if exists of the infinite
           lines where the segments belong to.
         */
        fn intercept_point_lines(&self, other: &L) -> Option<P>;

        /**
         * Get the interception point of 2 segments if exists.
         */
        fn intercept_point(&self, other: &L) -> Option<P>;

        /**
         * Return true if the segment intercepts with the other.
         */
        fn intercept(&self, other: &L) -> bool;

    }

    impl LineDir for Segment2d {

        fn get_leading_coefficient(p1: Point2d, p2: Point2d) -> f64 {
            (p1.y - p2.y) / (p1.x - p2.x)
        }

        fn get_y_intercept(a: f64, p: Point2d) -> f64 {
            p.y - a * p.x
        }

        fn get_line_coefficients(p1: Point2d, p2: Point2d) -> (f64, f64) {
            let a = Self::get_leading_coefficient(p1, p2);
            (a, Self::get_y_intercept(a, p1))
        }

    }

    impl GetPoint for Segment2d {

        fn get_y_from_x(&self, x: f64) -> f64 {
            self.a * x + self.b
        }

        fn get_x_from_y(&self, y: f64) -> f64 {
            (y - self.b) / self.a
        }

        fn get_point_from_x(&self, x: f64) -> Point2d {
            Point2d { x, y: self.get_y_from_x(x) }
        }

        fn get_point_from_y(&self, y: f64) -> Point2d {
            Point2d { x: self.get_x_from_y(y), y }
        }

        fn contain_point(&self, p: Point2d) -> bool {
            let (min_x, max_x) = reorder_pair(self.p1.x, self.p2.x);
            let (min_y, max_y) = reorder_pair(self.p1.y, self.p2.y);
            match (p.x < min_x) || (p.x > max_x) || (p.y < min_y) || (p.y > max_y) {
                true => false,
                _ => {
                    super::math::sub_abs(self.get_y_from_x(p.x), p.y) < ESPILON
                }
            }
        }

    }

    impl Intercept<Segment2d, Point2d> for Segment2d {

        fn intercept_point_lines(&self, other: &Segment2d) -> Option<Point2d> {
            match self.a == other.a {
                // parallel
                true => None,
                // not parallel
                _ => Some(self.get_point_from_x((other.b - self.b) / (self.a - other.a)))
            }
        }

        fn intercept_point(&self, other: &Segment2d) -> Option<Point2d> {
            let (min_x, max_x) = reorder_pair(self.p1.x, self.p2.x);
            let (min_y, max_y) = reorder_pair(self.p1.y, self.p2.y);
            let (min_x_other, max_x_other) = reorder_pair(other.p1.x, other.p2.x);
            let (min_y_other, max_y_other) = reorder_pair(other.p1.y, other.p2.y);

            match (min_x_other > max_x) || (max_x_other < min_x) || (min_y_other > max_y) || (max_y_other < min_y) {
                true => None,
                _ => {
                    match self.intercept_point_lines(other) {
                        None => None,
                        Some(p) => {
                            match p.x < min_x || p.x > max_x || p.y < min_y || p.y > max_y {
                                true => None,
                                _ => Some(p)
                            }
                        }
                    }
                }
            }
        }

        fn intercept(&self, other: &Segment2d) -> bool {
            self.intercept_point(other).is_some()
        }

    }

    impl Segment2d {
        pub fn new(pt1: Point2d, pt2: Point2d) -> Self {
            let (a_, b_) = Self::get_line_coefficients(pt1, pt2);
            Self {p1: pt1, p2: pt2, a: a_, b: b_}
        }
    }

}

#[cfg(test)]
mod tests {
    use super::lines::Point2d;

    #[test]
    fn add_point2d() {
        let mut p = Point2d{x: 2., y: 3.2};
        assert_eq!(Point2d{x: 2.1, y: -1.}, p + Point2d{x: 0.1, y: -4.2});
        p += Point2d{x: 0.1, y: -4.2};
        assert_eq!(Point2d{x: 2.1, y: -1.}, p)
    }

}
