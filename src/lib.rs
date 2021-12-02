use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;

/// Get an iterator over the lines of a file.
///
/// If the given file path can't be opened, return the IO error.
///
pub fn iter_lines<P: AsRef<Path>>(
    filename: P,
) -> io::Result<Box<dyn Iterator<Item = io::Result<String>>>> {
    let f = File::open(filename)?;
    let br = BufReader::new(f);
    let mut lines_iter = br.lines();
    Ok(Box::new(std::iter::from_fn(move || lines_iter.next())))
}
