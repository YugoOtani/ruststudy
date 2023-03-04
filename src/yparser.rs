use crate::ysh::*;
pub fn parse_ysh(s: &str) -> Result<Ysh, String> {
    let (ysh, s) = p_a(s.trim_end())?;
    if s == format!("") {
        Ok(ysh)
    } else {
        Err(format!("{s} remains untaken"))
    }
}
// S := a
// a := b;a  b&&a  b|a  b||a  b
// a>STR;a  a>STR&&a  a>STR|a  a>STR||a  a>STR
// b := (a) | C

//delete left recursion
// S := a
// a := b;t  b&&t  b|t  b||t  bx
// x := >STR;t  >STR&&t >STR|t >STR||t >STRx ////end
// t := ax
// b := (a) | C

fn p_a(s: &str) -> Result<(Ysh, &str), String> {
    let (b, s) = p_b(del_space(s))?;
    let s = del_space(s);
    match next_n(&s, 2) {
        Some("&&") => {
            let (t, s) = p_t(&s[2..])?;
            Ok((y_and(b, t), s))
        }
        Some("||") => {
            let (t, s) = p_t(&s[2..])?;
            Ok((y_or(b, t), s))
        }
        _ => match next(&s) {
            Some(';') => {
                let (t, s) = p_t(&s[1..])?;
                Ok((y_seq(b, t), s))
            }
            Some('|') => {
                let (t, s) = p_t(&s[1..])?;
                Ok((y_pipe(b, t), s))
            }
            Some('<') | Some('>') | Some(')') => p_x(b, s),
            Some(c) => Err(format!("unexpected '{c}' in p_a")),
            None => Ok((b, "")),
        },
    }
}
fn p_x(ysh: Ysh, s: &str) -> Result<(Ysh, &str), String> {
    let s = del_space(s);
    let (r, s) = match next(&s) {
        Some('>') => {
            let (fname, s) = take_string(del_space(&s[1..]))?;
            (y_out(ysh, fname), s)
        }
        Some('<') => {
            let (fname, s) = take_string(del_space(&s[1..]))?;
            (y_in(ysh, fname), s)
        }
        None | Some(')') => return Ok((ysh, s)),
        Some(s) => return Err(format!("unexpected {s} in p_x")),
    };
    let s = del_space(s);
    match next_n(&s, 2) {
        Some("&&") => {
            let (t, s) = p_t(&s[2..])?;
            Ok((y_and(r, t), s))
        }
        Some("||") => {
            let (t, s) = p_t(&s[2..])?;
            Ok((y_or(r, t), s))
        }
        _ => match next(&s) {
            Some('|') => {
                let (t, s) = p_t(del_space(&s[1..]))?;
                Ok((y_pipe(r, t), s))
            }
            Some(';') => {
                let (t, s) = p_t(del_space(&s[1..]))?;
                Ok((y_seq(r, t), s))
            }
            Some('<') | Some('>') => p_x(r, s),
            None | Some(')') => Ok((r, s)),
            Some(c) => Err(format!("unexpected {c} in p_x")),
        },
    }
}
fn p_b(s: &str) -> Result<(Ysh, &str), String> {
    let s = del_space(s);
    match next(&s) {
        Some('(') => {
            let (a, s) = p_a(&s[1..])?;
            if let Some(')') = next(&s) {
                Ok((y_sub(a), &s[1..]))
            } else {
                Err(format!("unexpected {:?} in p_b", next(&s)))
            }
        }
        _ => take_com(s),
    }
}
fn p_t(s: &str) -> Result<(Ysh, &str), String> {
    let s = del_space(s);
    let (a, s) = p_a(s)?;
    p_x(a, s)
}
fn next(s: &str) -> Option<char> {
    s.chars().nth(0)
}
fn next_n(s: &str, n: usize) -> Option<&str> {
    if s.len() < n {
        None
    } else {
        Some(&s[..n])
    }
}

fn del_space(s: &str) -> &str {
    match s.find(|c: char| !c.is_whitespace()) {
        Some(0) => s,
        Some(i) => &s[i..],
        None => "",
    }
}
fn take_string(s: &str) -> Result<(&str, &str), String> {
    let s = del_space(s);
    let f = |c: char| (RESERVED_CHARS.contains(|c2| c2 == c) || c.is_whitespace());
    match s.find(f) {
        Some(0) => Err(format!("can't take string from {s}")),
        Some(i) => Ok((&s[..i], &s[i..])),
        None => Ok((s, "")),
    }
}
fn take_com(s: &str) -> Result<(Ysh, &str), String> {
    let s = del_space(s);
    let f = |c: char| RESERVED_CHARS.contains(|c2| c2 == c);
    let splw = |s: &str| {
        s.split_whitespace()
            .map(String::from)
            .collect::<Vec<String>>()
    };
    match s.find(f) {
        Some(0) => Err(format!("can't take command from {s}")),
        Some(i) => match &splw(&s[..i])[..] {
            [head, tail @ ..] => Ok((y_com(head, &tail.to_vec())?, &s[i..])),
            _ => panic!(),
        },
        None => match &splw(s)[..] {
            [head, tail @ ..] => Ok((y_com(head, &tail.to_vec())?, "")),
            _ => panic!(),
        },
    }
}
