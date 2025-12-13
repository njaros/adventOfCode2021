use std::collections::BTreeSet;

use lib_aoc::{input_lib, segment::{InterNoDiagonal, Segment}};

// A lot of useless implementations, that's to learn the language.

fn main() {
    let content = input_lib::get_input_as_string(file!(), false);
    let mut cross_set: BTreeSet<[i64; 2]> = BTreeSet::new();

    // Collect all segments in a vector.
    let mut segments: Vec<Segment> = Vec::new();
    content.split('\n').for_each(|line| {
        segments.push(line.into());
    });

    // Only retain horizontal or vertical segments.
    segments.retain(|s| s.value[0][0] == s.value[1][0] || s.value[0][1] == s.value[1][1]);

    // debug part
    for i in 0..segments.len() {
        for j in i + 1 .. segments.len() {
            for point in segments[i].inter_points_nd(&segments[j]) {
                cross_set.insert(point);
            }
        }
    }

    // for point in &cross_set {
    //     println!("{}, {}\n", point[0], point[1]);
    // }

    println!("Result is {}", cross_set.len())
}
