use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    let (left, right) = input
        .lines()
        .map(parse_line)
        .fold(
            (Vec::new(), Vec::new()),
            |(mut left, mut right), (a, b)| {
                left.push(a);
                right.push(b);
                (left, right)
            }
        );
    
    let mut occs = left.into_iter()
        .map(|key| (key, 0_usize))
        .collect::<HashMap<_, _>>();

    for key in right {
        if let Some(product) = occs.get_mut(&key) {
            *product += key;
        }
    }

    let result: usize = occs.into_values().sum();

    println!("Total distance: {result}");
}

fn parse_line(line: &str) -> (usize, usize) {
    let mut words = line.split_ascii_whitespace();
    match (words.next(), words.next(), words.next()) {
        (Some(a), Some(b), None) => {
            let a = a.parse().unwrap();
            let b = b.parse().unwrap();

            (a, b)
        },
        _ => panic!(),
    }
}
