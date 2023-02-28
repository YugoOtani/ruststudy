use pom::parser::*;
use std::io::{stderr, stdin, stdout, Write};
use std::process;

fn main() {
    for i in 1.. {
        print!("ysh[{i}] 🐈 > ");
        stdout().flush().unwrap();
        let mut buf = String::new();
        match stdin().read_line(&mut buf) {
            Ok(_) => match p_ysh().parse(buf.as_bytes()) {
                Ok(res) => exec_node(&res),
                Err(e) => println!("{:?}", e),
            },
            Err(error) => println!("error: {error}"),
        }
    }
    //todo: ctrl-c and ctrl-d
}
//todo:https://en.wikipedia.org/wiki/Parsing_expression_grammar
const RESERVED_CHAR: &[u8; 11] = b";&|<>() \n\t\r";
#[derive(Debug)]
enum Ysh {
    YCommand(Command),
    YSeq(Command, Box<Ysh>),  // A; B
    YAnd(Command, Box<Ysh>),  // A && B
    YOr(Command, Box<Ysh>),   // A || B
    YPipe(Command, Box<Ysh>), // A | B
    YIn(Box<Ysh>, String),    // A < file
    YOut(Box<Ysh>, String),   // A > file
    YSub(Box<Ysh>),           // (A)
}

#[derive(Debug)]
struct Command {
    com: String,
    args: Vec<String>,
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
    none_of(RESERVED_CHAR)
        .repeat(1..)
        .convert(|u8s| String::from_utf8(u8s))
}
fn p_ysh<'a>() -> Parser<'a, u8, Ysh> {
    (call(p_ysh2) - sym(b'>') + id()).map(|(c, fname)| Ysh::YIn(Box::new(c), fname))
        | (call(p_ysh2) - sym(b'<') + id()).map(|(c, fname)| Ysh::YOut(Box::new(c), fname))
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

fn exec_node(ysh: &Ysh) {
    match ysh {
        //https://doc.rust-lang.org/std/os/unix/process/trait.CommandExt.html#tymethod.exec
        Ysh::YCommand(Command { com, args }) => {
            match process::Command::new(format!("/bin/{com}"))
                .args(args)
                .output()
            {
                Ok(msg) => {
                    stdout().write_all(&msg.stdout).unwrap();
                    stderr().write_all(&msg.stderr).unwrap();
                }
                Err(msg) => println!("err : {msg}"), //todo:change error message
            }
        }
        _ => todo!("other command"),
    }
}
fn space<'a>() -> Parser<'a, u8, ()> {
    one_of(b" \t\r\n").repeat(0..).discard()
}
