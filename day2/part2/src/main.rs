use std::{mem, ops::ControlFlow};

fn main() {
    let result = include_str!("input.txt")
        .lines()
        .filter(|&report| is_considered_safe(report))
        .count();

    println!("Safe reports: {result}");
}

fn levels(report: &str) -> impl Iterator<Item = isize> + use<'_> {
    report
        .split_ascii_whitespace()
        .map(|level| level.parse().unwrap())
}

fn changes<'a>(levels: impl IntoIterator<Item = &'a isize>) -> impl Iterator<Item = isize> {
    levels.into_iter().copied()
        // Compute the change between each pair of levels.
        .scan(0, |prev, curr| {
            Some(curr - mem::replace(prev, curr))
        })
        .skip(1)
}

fn is_considered_safe(report: &str) -> bool {
    let levels = levels(report).collect::<Vec<_>>();

    if let Some(index) = fault_index(&levels) {
        // The fault index can never be 0 because a fault can't occur until the level has changed.
        if
            fault_index(skip_nth(levels.iter(), index)).is_some() &&
            fault_index(skip_nth(levels.iter(), index - 1)).is_some() &&
            (
                index == 1 || fault_index(skip_nth(levels.iter(), index - 2)).is_some()
            )
        {
            return false;
        }
    }
    
    true
}

fn fault_index<'a>(levels: impl IntoIterator<Item = &'a isize>) -> Option<usize> {
    let outcome = changes(levels)
        .enumerate()
        .try_fold(0, |dir, (index, change)| {
            if (1..=3).contains(&change.abs()) && (
                dir == 0 || dir == change.signum()
            ) {
                ControlFlow::Continue(change.signum())
            } else {
                ControlFlow::Break(index)
            }
        });

    match outcome {
        ControlFlow::Break(index) => Some(index + 1),
        ControlFlow::Continue(_) => None,
    }
}

fn skip_nth<T>(iter: impl Iterator<Item = T>, n: usize) -> impl Iterator<Item = T> {
    iter.enumerate().filter(move |&(i, _)| i != n).map(|(_, t)| t)
}
