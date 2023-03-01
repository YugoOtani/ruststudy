use pom::parser::*;

//todo:https://en.wikipedia.org/wiki/Parsing_expression_grammar
const RESERVED_CHAR: &[u8; 11] = b";&|<>() \n\t\r";
#[derive(Debug)]
pub enum Ysh {
    YCommand(Command),
    YSeq(Command, Box<Ysh>),  // A; B
    YAnd(Command, Box<Ysh>),  // A && B
    YOr(Command, Box<Ysh>),   // A || B
    YPipe(Command, Box<Ysh>), // A | B
    YIn(Box<Ysh>, String),    // A < file
    YOut(Box<Ysh>, String),   // A > file
    YSub(Box<Ysh>),           // (A)
}
fn indent_n(n: usize) {
    for i in 0..n {
        print!("-");
    }
}
pub fn print_ysh(ysh: &Ysh) {
    fn print_com(c: &Command, n: usize) {
        indent_n(n);
        println!("Com[{},{:?}]", c.com, c.args);
    }
    fn go(ysh: &Ysh, indent: usize) {
        match ysh {
            Ysh::YCommand(com) => {
                print_com(com, indent);
            }
            Ysh::YSeq(com, ysh) => {
                indent_n(indent);
                println!("Seq");
                print_com(com, indent + 1);
                go(&ysh, indent + 1);
            }
            Ysh::YAnd(com, ysh) => {
                indent_n(indent);
                println!("And");
                print_com(com, indent + 1);
                go(ysh, indent + 1);
            }
            Ysh::YOr(com, ysh) => {
                indent_n(indent);
                println!("Or");
                print_com(com, indent + 1);
                go(ysh, indent + 1);
            }
            Ysh::YPipe(com, ysh) => {
                indent_n(indent);
                println!("Pipe");
                print_com(com, indent + 1);
                go(ysh, indent + 1);
            }
            Ysh::YIn(ysh, s) => {
                indent_n(indent);
                println!("In");
                go(ysh, indent + 1);
                indent_n(indent + 1);
                println!("{s}");
            }
            Ysh::YOut(ysh, s) => {
                indent_n(indent);
                println!("Out");
                go(&ysh, indent + 1);
                indent_n(indent + 1);
                println!("{s}");
            }
            Ysh::YSub(ysh) => {
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

fn com<'a>() -> Parser<'a, u8, Command> {
    (space() * list(id(), space()) - space()).convert(Command::new)
}
fn id<'a>() -> Parser<'a, u8, String> {
    (space() * none_of(RESERVED_CHAR).repeat(1..) - space()).convert(|u8s| String::from_utf8(u8s))
}
pub fn parser_ysh<'a>() -> Parser<'a, u8, Ysh> {
    (call(p_ysh2) - sym(b'<') + id()).map(|(c, fname)| Ysh::YIn(Box::new(c), fname))
        | (call(p_ysh2) - sym(b'>') + id()).map(|(c, fname)| Ysh::YOut(Box::new(c), fname))
        | (sym(b'(') * call(p_ysh2) - sym(b')')).map(|ysh| Ysh::YSub(Box::new(ysh)))
        | (com() - sym(b';') + call(p_ysh2)).map(|(s, ysh)| Ysh::YSeq(s, Box::new(ysh)))
        | (com() - seq(b"&&") + call(p_ysh2)).map(|(s, ysh)| Ysh::YAnd(s, Box::new(ysh)))
        | (com() - seq(b"||") + call(p_ysh2)).map(|(s, ysh)| Ysh::YOr(s, Box::new(ysh)))
        | (com() - sym(b'|') + call(p_ysh2)).map(|(s, ysh)| Ysh::YPipe(s, Box::new(ysh)))
        | call(p_ysh2)
}
fn p_ysh2<'a>() -> Parser<'a, u8, Ysh> {
    (sym(b'(') * call(p_ysh2) - sym(b')')).map(|ysh| Ysh::YSub(Box::new(ysh)))
        | (com() - end()).map(Ysh::YCommand)
        | (com() - sym(b';') + call(p_ysh2)).map(|(s, ysh)| Ysh::YSeq(s, Box::new(ysh)))
        | (com() - seq(b"&&") + call(p_ysh2)).map(|(s, ysh)| Ysh::YAnd(s, Box::new(ysh)))
        | (com() - seq(b"||") + call(p_ysh2)).map(|(s, ysh)| Ysh::YOr(s, Box::new(ysh)))
        | (com() - sym(b'|') + call(p_ysh2)).map(|(s, ysh)| Ysh::YPipe(s, Box::new(ysh)))
        | com().map(Ysh::YCommand)
}

fn space<'a>() -> Parser<'a, u8, ()> {
    one_of(b" \t\r\n").repeat(0..).discard()
}
