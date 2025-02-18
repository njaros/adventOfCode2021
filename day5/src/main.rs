use lib_aoc::input_lib;

struct Segment {
    value: [[u32; 2]; 2],
}

impl From<&str> for Segment {
    fn from(s: &str) -> Self {
        let mut seg = Segment{ value: [[0; 2]; 2]};
        s.split(" -> ").enumerate().for_each(|(idx1, pair)| {
            pair.split(',').enumerate().for_each(|(idx2, n_str)| {
                seg.value[idx1][idx2] = n_str.parse().unwrap();
            });
        });
        seg
    }
}



fn main() {
    let mut content = input_lib::get_input_as_string(file!(), true);
    let segments: Vec<Segment> = Vec::new();
    content.split('\n').for_each(|line| {
        let new_segment: Segment = line.into();
    });
}
