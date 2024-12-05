fn main() {
    let input = include_str!("input.txt");

    let total = parse_all_instrs(input)
        .scan(true, |enabled, instr| {
            match (*enabled, instr) {
                (_, Instr::Do) => { *enabled = true; },
                (_, Instr::Dont) => { *enabled = false; },
                (true, Instr::Mul(a, b)) => return Some(Some((a, b))),
                (false, Instr::Mul(_, _)) => {},
            }
            Some(None)
        })
        .flatten()
        .map(|(a, b)| a * b)
        .sum::<u32>();

    println!("Result: {total}");
}

fn parse_all_instrs(mut src: &str) -> impl Iterator<Item = Instr> + use<'_> {
    std::iter::from_fn(move || {
        while !src.is_empty() {
            if let Some(instr) = parse_instr(&mut src) {
                return Some(instr);
            } else {
                advance(&mut src);
            }
        }
        None
    })
}

fn parse_instr(src: &mut &str) -> Option<Instr> {
    for parser in [parse_mul, parse_dont, parse_do] {
        let mut temp_src = *src;
        let parsed = parser(&mut temp_src);
        if parsed.is_some() {
            *src = temp_src;
            return parsed;
        }
    }
    None
}

fn parse_mul(src: &mut &str) -> Option<Instr> {
    parse_tag(src, "mul(")?;
    let left = parse_num(src)?;
    parse_char(src, ',')?;
    let right = parse_num(src)?;
    parse_char(src, ')')?;

    Some(Instr::Mul(left, right))
}

fn parse_dont(src: &mut &str) -> Option<Instr> {
    parse_tag(src, "don't()").map(|_| Instr::Dont)
}

fn parse_do(src: &mut &str) -> Option<Instr> {
    parse_tag(src, "do()").map(|_| Instr::Do)
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
enum Instr {
    Do,
    Dont,
    Mul(u32, u32),
}
