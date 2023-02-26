use pom::parser::*;
use pom::Parser;
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
    YOpen,
    YClose,
    YToken(String),
}

fn main() {
    loop {
        let mut s = String::new();
        print!("input words to parse > ");
        stdout().flush().unwrap();
        stdin().read_line(&mut s).unwrap();
        println!("{:?}", exp().parse(&mut s.as_bytes()));
        stdout().flush().unwrap();
    }
}
fn exp() -> Parser<u8, Vec<u8>> {
    let integer = one_of(b"0123456789").repeat(0..);
    let space = sym(b' ').repeat(0..);
    end().map(|_| vec![])
        | (integer + space * call(exp)).map(|(mut v, mut e)| {
            v.append(&mut e);
            v
        })
}
fn space() -> Parser<u8, ()> {
    one_of(b" \t\r\n").repeat(0..).discard()
}

fn tn<T>(_: &T) {
    println!("{:?}", type_name::<*const T>())
}
