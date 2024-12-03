use std::mem;

fn main() {
    let result = include_str!("input.txt")
        .lines()
        .filter(|&report| is_safe(report))
        .count();

    println!("Safe reports: {result}");
}

fn changes(report: &str) -> impl Iterator<Item = isize> + use<'_> {
    report
        // Parse the levels from the text.
        .split_ascii_whitespace()
        .map(|level| level.parse::<isize>().unwrap())
        // Compute the change between each pair of levels.
        .scan(0, |prev, curr| {
            Some(curr - mem::replace(prev, curr))
        })
        .skip(1)
}

fn is_safe(report: &str) -> bool {
    let mut changes = changes(report)
        // Allow peeking at the first change, to determine the direction.
        .peekable();

    let Some(first) = changes.peek() else { return true; };
    let direction = first.signum();

    // Check that all changes match the direction and are within the safe range.
    changes.all(|change| {
        change.signum() == direction &&
        (1..=3).contains(&change.abs())
    })
}
