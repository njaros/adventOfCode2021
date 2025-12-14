use lib_aoc::input_lib;

fn sleep_1_day(poissons: &mut[u64; 9]) {
    let new_poissons = poissons[0];

    for idx in 0..8 {
        poissons[idx] = poissons[idx + 1];
    }
    poissons[6] += new_poissons;
    poissons[8] = new_poissons;
}

fn main() {
    let part = input_lib::get_part();
    let input = input_lib::get_input_as_string(file!(), false);

    let mut poissons =
    input
    .split(',')
    .fold([0u64; 9], |mut acc, poisson_raw| {
        acc[poisson_raw.parse::<usize>().unwrap()] += 1;
        acc
    });

    println!("{}",{
        (0..match part {1 => 80, _ => 256}).for_each(|_| sleep_1_day(&mut poissons));
        poissons.into_iter().reduce(|a, b| a + b).unwrap()
    })
}
