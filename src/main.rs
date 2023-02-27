use pom::parser::is_a;
use pom::parser::*;
use std::any::type_name;
use std::io::{stdin, stdout, Write};

const RESERVED_CHAR: &[u8; 11] = b";&|<>() \n\t\r";
#[derive(Debug)]
enum Ysh {
    YCommand(String),
    YSeq(Box<Ysh>, Box<Ysh>),  // A; B
    YAnd(Box<Ysh>, Box<Ysh>),  // A && B
    YOr(Box<Ysh>, Box<Ysh>),   // A || B
    YPipe(Box<Ysh>, Box<Ysh>), // A | B
    YIn(Box<Ysh>, String),     // A < file
    YOut(Box<Ysh>, String),    // A > file
    YSub(Box<Ysh>),            // (A)
}
fn main() {
    let mut s = String::new();
    stdout().flush().unwrap();
    stdin().read_line(&mut s).unwrap();
    println!("{:?}", p_command().parse(s.as_bytes()));
}
fn p_command<'a>() -> Parser<'a, u8, Ysh> {
    let com = space() * none_of(RESERVED_CHAR).repeat(1..) - space();
    com.convert(|u8s| String::from_utf8(u8s)).map(Ysh::YCommand)
}
fn exp<'a>() -> Parser<'a, u8, Vec<u8>> {
    let integer = one_of(b"0123456789").repeat(0..);
    let space = sym(b' ').repeat(0..);
    end().map(|_| vec![])
        | (integer + space * call(exp)).map(|(mut v, mut e)| {
            v.append(&mut e);
            v
        })
}
fn space<'a>() -> Parser<'a, u8, ()> {
    one_of(b" \t\r\n").repeat(0..).discard()
}

fn tn<T>(_: &T) {
    println!("{:?}", type_name::<T>())
}
