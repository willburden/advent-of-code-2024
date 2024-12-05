fn main() {
    let mut input = include_str!("input.txt");

    let mut total = 0;
    while !input.is_empty() {
        let prev_ptr = input.as_ptr();

        if let Some(mul) = parse_mul(&mut input) {
            total += mul.evaluate();
        } else if input.as_ptr() == prev_ptr {
            // If the parse attempt failed without consuming a character, we advance by one
            // character so that the parser has a different input next time.
            advance(&mut input);
        }
    }

    println!("Result: {total}");
}

fn parse_mul(src: &mut &str) -> Option<MulInstr> {
    parse_tag(src, "mul(")?;
    let left = parse_num(src)?;
    parse_char(src, ',')?;
    let right = parse_num(src)?;
    parse_char(src, ')')?;

    Some(MulInstr { left, right })
}

fn parse_tag(src: &mut &str, tag: &str) -> Option<()> {
    for exp in tag.chars() {
        parse_char(src, exp)?;
    }
    Some(())
}

fn parse_char(src: &mut &str, exp: char) -> Option<()> {
    match src.chars().next() {
        Some(c) if c == exp => { advance(src); Some(()) },
        _ => return None,
    }
}

fn parse_num(src: &mut &str) -> Option<u32> {
    let digits = parse_while(src, |c| c.is_ascii_digit());
    if (1..=3).contains(&digits.len()) {
        digits.parse().ok()
    } else {
        None
    }
}

fn parse_while<'a, P>(src: &mut &'a str, mut predicate: P) -> &'a str
where
    P: FnMut(char) -> bool,
{
    let split_index = src.char_indices()
        .find(|&(_, c)| !predicate(c))
        .map(|(i, _)| i)
        .unwrap_or(src.len());

    &std::mem::replace(src, &src[split_index..])[..split_index]
}

fn advance(src: &mut &str) -> Option<char> {
    src.chars().next()
        .inspect(|&c| *src = &src[c.len_utf8()..])
}

#[derive(Debug, Clone, Copy)]
struct MulInstr {
    left: u32,
    right: u32,
}

impl MulInstr {
    fn evaluate(&self) -> u32 {
        self.left * self.right
    }
}
