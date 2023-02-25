use std::any::type_name;
use std::io::{stdin, stdout, Write};
enum Ysh {
    YCommand(String),
    YSeq(Box<Ysh>, Box<Ysh>),  // A; B
    YAnd(Box<Ysh>, Box<Ysh>),  // A && B
    YOr(Box<Ysh>, Box<Ysh>),   // A || B
    YPipe(Box<Ysh>, Box<Ysh>), // A | B
    YIn(Box<Ysh>, String),     // A < file
    YOut(Box<Ysh>, String),    // A > file
}
#[derive(Debug)]
enum YshKw {
    YSeq,
    YAnd,
    YOr,
    YPipe,
    YIn,
    YOut,
    YToken(String),
}
fn parse_words(s: &str) -> Vec<YshKw> {
    s.split(' ').map(|w| conv(w)).collect()
}
fn conv(s: &str) -> YshKw {
    match s {
        ";" => YshKw::YSeq,
        "&&" => YshKw::YAnd,
        "||" => YshKw::YOr,
        "|" => YshKw::YPipe,
        "<" => YshKw::YIn,
        ">" => YshKw::YOut,
        s => YshKw::YToken(s.to_string()),
    }
}

fn main() {
    loop {
        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();
        println!("{:?}", parse_words(&s[..]));
        stdout().flush().unwrap();
    }
}

fn tn<T>(_: T) {
    println!("{:?}", type_name::<T>())
}
