mod preloaded;
use preloaded::RegExp;


fn main() {
    // Here I was debugging code
}

pub fn str_to_regex(input: &str) -> Option<RegExp> {
    let chars: Vec<char> = input.chars().collect();
    let (re, pos) = parse_or(&chars, 0)?;
    if pos == chars.len() {
        Some(re)
    } else {
        None
    }
}


fn parse_or(chars: &[char], pos: usize) -> Option<(RegExp, usize)> {
    let (mut left, mut pos) = parse_seq(chars, pos)?;
    if pos < chars.len() && chars[pos] == '|' {
        pos += 1;
        let (right, new_pos) = parse_or(chars, pos)?;
        left = RegExp::Or(Box::new(left), Box::new(right));
        pos = new_pos;
    }
    Some((left, pos))
}

fn parse_seq(chars: &[char], mut pos: usize) -> Option<(RegExp, usize)> {
    let mut parts = Vec::new();
    while let Some((node, next_pos)) = parse_star(chars, pos) {
        parts.push(node);
        pos = next_pos;
        if pos < chars.len() && (chars[pos] == '|' || chars[pos] == ')') {
            break;
        }
    }
    if parts.is_empty() {
        None
    } else if parts.len() == 1 {
        Some((parts.remove(0), pos))
    } else {
        Some((RegExp::Str(parts), pos))
    }
}

fn parse_star(chars: &[char], pos: usize) -> Option<(RegExp, usize)> {
    let (mut node, mut pos) = parse_atom(chars, pos)?;
    if pos < chars.len() && chars[pos] == '*' {
        node = RegExp::ZeroOrMore(Box::new(node));
        pos += 1;
        if pos < chars.len() && chars[pos] == '*' {
            return None;
        }
    }
    Some((node, pos))
}

fn parse_atom(chars: &[char], pos: usize) -> Option<(RegExp, usize)> {
    if pos >= chars.len() {
        return None;
    }
    let c = chars[pos];
    match c {
        '.' => Some((RegExp::Any, pos + 1)),
        '(' => {
            let (re, next_pos) = parse_or(chars, pos + 1)?;
            if next_pos < chars.len() && chars[next_pos] == ')' {
                Some((re, next_pos + 1))
            } else {
                None
            }
        }
        ')' | '|' | '*' => None,
        _ => Some((RegExp::Normal(c), pos + 1)),
    }
}