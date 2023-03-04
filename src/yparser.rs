//todo:https://en.wikipedia.org/wiki/Parsing_expression_grammar
const RESERVED_CHARS: &str = ";&|<>()";
pub const VALID_COMMAND: [&str; 9] = [
    "ls", "cat", "pwd", "ps", "echo", "cp", "kill", "mkdir", "sleep",
];
#[derive(Debug)]
pub enum Ysh {
    Command(Command),
    Seq(Box<Ysh>, Box<Ysh>),  // A; B
    And(Box<Ysh>, Box<Ysh>),  // A && B
    Or(Box<Ysh>, Box<Ysh>),   // A || B
    Pipe(Box<Ysh>, Box<Ysh>), // A | B
    In(Box<Ysh>, String),     // A < file
    Out(Box<Ysh>, String),    // A > file
    Sub(Box<Ysh>),            // (A)
}
fn indent_n(n: usize) {
    print!("{}", "--".repeat(n));
}
pub fn print_ysh(ysh: &Ysh) {
    fn print_com(c: &Command, n: usize) {
        indent_n(n);
        println!("Com[{},{:?}]", c.com, c.args);
    }
    fn go(ysh: &Ysh, indent: usize) {
        match ysh {
            Ysh::Command(com) => {
                print_com(com, indent);
            }
            Ysh::Seq(ysh1, ysh2) => {
                indent_n(indent);
                println!("Seq");
                go(&ysh1, indent + 1);
                go(&ysh2, indent + 1);
            }
            Ysh::And(ysh1, ysh2) => {
                indent_n(indent);
                println!("And");
                go(&ysh1, indent + 1);
                go(&ysh2, indent + 1);
            }
            Ysh::Or(ysh1, ysh2) => {
                indent_n(indent);
                println!("Or");
                go(&ysh1, indent + 1);
                go(&ysh2, indent + 1);
            }
            Ysh::Pipe(ysh1, ysh2) => {
                indent_n(indent);
                println!("Pipe");
                go(&ysh1, indent + 1);
                go(&ysh2, indent + 1);
            }
            Ysh::In(ysh, s) => {
                indent_n(indent);
                println!("In");
                go(ysh, indent + 1);
                indent_n(indent + 1);
                println!("{s}");
            }
            Ysh::Out(ysh, s) => {
                indent_n(indent);
                println!("Out");
                go(&ysh, indent + 1);
                indent_n(indent + 1);
                println!("{s}");
            }
            Ysh::Sub(ysh) => {
                indent_n(indent);
                println!("Sub");
                go(&ysh, indent + 1);
            }
        }
    }
    go(ysh, 0)
}

#[derive(Debug)]
pub struct Command {
    pub com: String,
    pub args: Vec<String>,
}

impl Command {
    pub fn new(v: Vec<String>) -> Result<Command, String> {
        if v.len() == 0 {
            Err(String::from("empty command"))
        } else {
            Ok(Command {
                com: v[0].clone(),
                args: v[1..].to_vec(),
            })
        }
    }
}

pub fn parse_ysh(s: String) -> Result<Ysh, String> {
    let (ysh, s) = p_a(s.lines().collect())?;
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

// S := a
// a := b;t  b&&t  b|t  b||t  bx
// x := >STR;t  >STR&&t >STR|t >STR||t >STRx ////end
// t := ax
// b := (a) | C

fn p_a(s: String) -> Result<(Ysh, String), String> {
    let (b, s) = p_b(del_space(s))?;
    let s = del_space(s);
    match look_n(&s, 2) {
        Some("&&") => {
            let (t, s) = p_t(del_n(s, 2))?;
            Ok((y_and(b, t), s))
        }
        Some("||") => {
            let (t, s) = p_t(del_n(s, 2))?;
            Ok((y_or(b, t), s))
        }
        _ => match look_1(&s) {
            Some(';') => {
                let (t, s) = p_t(del_1(s))?;
                Ok((y_seq(b, t), s))
            }
            Some('|') => {
                let (t, s) = p_t(del_1(s))?;
                Ok((y_pipe(b, t), s))
            }
            Some('<') | Some('>') | Some(')') => p_x(b, s),
            Some(c) => Err(format!("unexpected '{c}' in p_a")),
            None => Ok((b, format!(""))),
        },
    }
}
fn p_x(ysh: Ysh, s: String) -> Result<(Ysh, String), String> {
    let s = del_space(s);
    let (r, s) = match look_1(&s) {
        Some('>') => {
            let (fname, s) = take_string(del_space(del_1(s)))?;
            (y_out(ysh, fname), s)
        }
        Some('<') => {
            let (fname, s) = take_string(del_space(del_1(s)))?;
            (y_in(ysh, fname), s)
        }
        None | Some(')') => return Ok((ysh, s)),
        Some(s) => return Err(format!("unexpected {s} in p_x")),
    };
    let s = del_space(s);
    match look_n(&s, 2) {
        Some("&&") => {
            let (t, s) = p_t(del_n(s, 2))?;
            Ok((y_and(r, t), s))
        }
        Some("||") => {
            let (t, s) = p_t(del_n(s, 2))?;
            Ok((y_or(r, t), s))
        }
        _ => match look_1(&s) {
            Some('|') => {
                let (t, s) = p_t(del_space(del_1(s)))?;
                Ok((y_pipe(r, t), s))
            }
            Some(';') => {
                let (t, s) = p_t(del_space(del_1(s)))?;
                Ok((y_seq(r, t), s))
            }
            Some('<') | Some('>') => p_x(r, s),
            None | Some(')') => Ok((r, s)),
            Some(c) => Err(format!("unexpected {c} in p_x")),
        },
    }
}
fn p_b(s: String) -> Result<(Ysh, String), String> {
    let s = del_space(s);
    match look_1(&s) {
        Some('(') => {
            let (a, s) = p_a(del_1(s))?;
            if let Some(')') = look_1(&s) {
                Ok((y_sub(a), del_1(s)))
            } else {
                Err(format!("unexpected {:?} in p_b", look_1(&s)))
            }
        }
        _ => take_com(s),
    }
}
fn p_t(s: String) -> Result<(Ysh, String), String> {
    let s = del_space(s);
    let (a, s) = p_a(s)?;
    p_x(a, s)
}
fn look_1(s: &String) -> Option<char> {
    s.chars().nth(0)
}
fn look_n(s: &String, n: usize) -> Option<&str> {
    if s.len() < n {
        None
    } else {
        Some(&s[..n])
    }
}
fn del_1(s: String) -> String {
    String::from(&s[1..])
}
fn del_n(s: String, n: usize) -> String {
    String::from(&s[n..])
}
fn del_space(s: String) -> String {
    s.chars().skip_while(|c| c.is_whitespace()).collect()
}
fn take_string(s: String) -> Result<(String, String), String> {
    let s = del_space(s);
    let f = |c: char| (RESERVED_CHARS.contains(|c2| c2 == c) || c.is_whitespace());
    match s.find(f) {
        Some(0) => Err(format!("can't take string from {s}")),
        Some(i) => {
            let mut lft = s.to_string();
            let ret = lft.drain(..i).collect();
            Ok((ret, lft))
        }
        None => Ok((s, "".to_string())),
    }
}
fn take_com(s: String) -> Result<(Ysh, String), String> {
    let s = del_space(s);
    let f = |c: char| RESERVED_CHARS.contains(|c2| c2 == c);
    let splw = |s: String| s.split_whitespace().map(String::from).collect();
    match s.find(f) {
        Some(0) => Err(format!("can't take command from {s}")),
        Some(i) => {
            let mut lft = s.to_string();
            let rcom = Command::new(splw(lft.drain(..i).collect()));
            rcom.map(move |com| (Ysh::Command(com), lft))
        }
        None => Command::new(splw(s)).map(move |com| (Ysh::Command(com), "".to_string())),
    }
}

pub fn y_seq(l: Ysh, r: Ysh) -> Ysh {
    Ysh::Seq(Box::new(l), Box::new(r))
}
pub fn y_and(l: Ysh, r: Ysh) -> Ysh {
    Ysh::And(Box::new(l), Box::new(r))
}
pub fn y_or(l: Ysh, r: Ysh) -> Ysh {
    Ysh::Or(Box::new(l), Box::new(r))
}
pub fn y_pipe(l: Ysh, r: Ysh) -> Ysh {
    Ysh::Pipe(Box::new(l), Box::new(r))
}
pub fn y_in(l: Ysh, s: String) -> Ysh {
    Ysh::In(Box::new(l), s)
}
pub fn y_out(l: Ysh, s: String) -> Ysh {
    Ysh::Out(Box::new(l), s)
}
pub fn y_sub(y: Ysh) -> Ysh {
    Ysh::Sub(Box::new(y))
}
