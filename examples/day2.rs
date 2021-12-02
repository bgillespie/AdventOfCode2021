use libaoc2021 as lib;

const FILENAME: &str = "data/day2/input";

/// Translate instructions of the form (direction, distance) to (x, y) moves.
///
fn translator(instruction: String) -> (isize, isize) {
    let space_index = instruction.find(' ').expect("No space found");

    // the numeric (second) bit, after the string
    let distance = instruction[space_index + 1..]
        .parse::<isize>()
        .expect("not a number");

    // use the direction (first) bit to create the vector
    match &instruction[..space_index] {
        "forward" => (distance, 0),
        "down" => (0, distance),
        "up" => (0, -distance),
        _ => panic!("unknown instruction"),
    }
}

/// Straightforward translations from the origin
fn part_1(source: &mut dyn Iterator<Item = (isize, isize)>) -> (isize, isize) {
    source.fold((0, 0), |acc, i| (acc.0 + i.0, acc.1 + i.1))
}

/// Moves using the "aim".
fn part_2(source: &mut dyn Iterator<Item = (isize, isize)>) -> (isize, isize) {
    // the 3-tuple below is (x, y, aim)
    let moves = source.fold((0, 0, 0), |acc, i| {
        (acc.0 + i.0, acc.1 + (acc.2 * i.0), acc.2 + i.1)
    });
    (moves.0, moves.1)
}

#[test]
fn test_part2() {
    //! Test using the example in the puzzle
    let moves = [(5isize, 0isize), (0, 5), (8, 0), (0, -3), (0, 8), (2, 0)];
    let iterator = &mut moves.into_iter();
    assert_eq!(part_2(iterator), (15, 60));
}

fn main() {
    let mut iterator = lib::iter_lines(FILENAME)
        .expect("Cannot open file")
        .map(|i| translator(i.expect("bad read")));

    let answer = part_2(&mut iterator);
    println!("Product of x and y: {}", answer.0 * answer.1);
}
