use libaoc2021 as lib;

// WINDOW_SIZE = 1 for the first part of the puzzle, 3 for the second part
const WINDOW_SIZE: usize = 3;
const FILENAME: &str = "data/day1/input";

fn main() {
    // Get an iterator over the lines in the file, converting them to usize as we go.
    let mut iterator = lib::iter_lines(FILENAME)
        .expect("Cannot open file")
        .map(|i| i.expect("bad read").parse::<usize>().expect("not a number"));
    
    // Load up cache
    let mut cache = [0usize; WINDOW_SIZE];
    for i in 0..WINDOW_SIZE {
        cache[i] = iterator.next().expect("collection too small for cache");
    }

    // Compare entries
    let mut cache_index = 0;
    let mut count_greater = 0;
    for item in iterator {
        if item > cache[cache_index] {
            count_greater += 1;
        }
        cache[cache_index] = item;
        cache_index = (cache_index + 1) % WINDOW_SIZE;
    }

    println!("Increases: {}", count_greater);
}
