fn main() {
    let input = include_str!("input.txt");

    let (mut left, mut right) = input
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
    
    left.sort_unstable();
    right.sort_unstable();

    let result = left.into_iter().zip(right.into_iter())
        .map(|(a, b)| if a > b {
            a - b
        } else {
            b - a
        })
        .sum::<usize>();

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
