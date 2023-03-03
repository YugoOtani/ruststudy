
//todo:https://en.wikipedia.org/wiki/Parsing_expression_grammar
const SPACE_CHARS: &[u8; 4] = b" \n\t\r";
const RESERVED_CHARS: &[u8; 11] = b";&|<>() \n\t\r";
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
    fn new(v: Vec<String>) -> Result<Command, String> {
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
    p_a(s)
}
// S := a
// a := b;a  b&&a  b|a  b||a  b
// a>STR;a  a>STR&&a  a>STR|a  a>STR||a  a>STR
// b := (a) | C

// S := a
// a := b;ax  b&&ax  b|ax  b||ax  bx
// x := >STR;ax  >STR&&ax >STR|ax >STR||ax >STRx end
// b := (a) | C

enum X{
    Seq(Ysh),And(Ysh),Pipe(Ysh),Or(Ysh)
}
fn p_a(s: String) -> Result<Ysh, String> {
    let (b, s) = p_b(s);
    let s = del_space(s);
    match look_n(&s,2) {
        Some("&&") => p_a(del_n(s,2)).and_then(|c| p_x(c,))
            let x=p_x(s);
        ,
        Some("||") => {
            p_a(del_n(s,2));
        }
    }
    todo!()
}
fn p_x(ysh:Ysh,s: String) -> Result<Ysh,String> {
    todo!()
}
fn p_b(s: String) -> (Ysh, String) {
    todo!()
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
