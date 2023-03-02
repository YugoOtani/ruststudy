use pom::parser::*;
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
// S := a
// a := b;a | b
// b := (a) | C > STR | C

pub fn parser_ysh<'a>() -> Parser<'a, u8, Ysh> {
    fn p_a<'a>() -> Parser<'a, u8, Ysh> {
        (call(p_b) - sym(b';') + call(p_a)).map(|(b, a)| Ysh::Seq(Box::new(b), Box::new(a)))
            | (call(p_b) - seq(b"&&") + call(p_a)).map(|(b, a)| Ysh::And(Box::new(b), Box::new(a)))
            | (call(p_b) - seq(b"||") + call(p_a)).map(|(b, a)| Ysh::Or(Box::new(b), Box::new(a)))
            | (call(p_b) - sym(b'|') + call(p_a)).map(|(b, a)| Ysh::Pipe(Box::new(b), Box::new(a)))
            | call(p_b)
    }
    fn p_b<'a>() -> Parser<'a, u8, Ysh> {
        (space() * sym(b'(') * p_a() - sym(b')') - space())
            .map(Box::new)
            .map(Ysh::Sub)
            | (com() - sym(b'>') + id()).map(|(a, s)| Ysh::Out(Box::new(a), s))
            | (com() - sym(b'<') + id()).map(|(a, s)| Ysh::In(Box::new(a), s))
            | com()
    }
    p_a()
}

fn com<'a>() -> Parser<'a, u8, Ysh> {
    (space() * list(id(), space()) - space())
        .convert(Command::new)
        .map(Ysh::Command)
}
fn id<'a>() -> Parser<'a, u8, String> {
    (space() * none_of(RESERVED_CHARS).repeat(1..) - space()).convert(|u8s| String::from_utf8(u8s))
}
fn space<'a>() -> Parser<'a, u8, ()> {
    one_of(SPACE_CHARS).repeat(0..).discard()
}
/*
a -> C   b;a   (a)   c&&a   d|a   e>S
結合順位 () > &&,"<",";" |
 */
