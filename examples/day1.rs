use libaoc2021 as lib;

// WINDOW_SIZE = 1 for the first part of the puzzle, 3 for the second part
const WINDOW_SIZE: usize = 3;
const FILENAME: &str = "data/day1/input";

/// Count the increases over N sized window.
///
/// You don't have to do any adding:
///   (b + c + d) > (a + b + c) if (b + c + d) - (a + b + c) > 0
///   (b + c + d) - (a + b + c) = d - a
///   has_increased = d - a > 0
///
fn count_increases<const N: usize>(iterator: &mut dyn Iterator<Item = usize>) -> usize {
    // Preload N-sized buffer.
    let mut buffer = [0usize; WINDOW_SIZE];
    buffer
        .iter_mut()
        .map(|i| *i = iterator.next().expect("too few"))
        .nth(N);

    // Compare entries.
    let mut buffer_index = 0;
    let mut count_greater = 0;
    for item in iterator {
        if item > buffer[buffer_index] {
            count_greater += 1;
        }
        buffer[buffer_index] = item;
        buffer_index = (buffer_index + 1) % WINDOW_SIZE;
    }

    count_greater
}

fn main() {
    // Get an iterator over the lines in the file, converting them to usize as we go.
    let mut iterator = lib::iter_lines(FILENAME)
        .expect("Cannot open file")
        .map(|i| i.expect("bad read").parse::<usize>().expect("not a number"));

    let count_greater = count_increases::<WINDOW_SIZE>(&mut iterator);

    println!("Increases: {}", count_greater);
}
