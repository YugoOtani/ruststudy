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
        let integer = one_of(b"0123456789").repeat(0..);
        //let brace = sym(b'(') * integer - sym(b')');
        tn(&integer);
        println!("{:?} {:?}", s.as_bytes(), integer.parse(&s.as_bytes()));
        stdout().flush().unwrap();
    }
}

fn tn<T>(_: T) {
    println!("{:?}", type_name::<T>())
}
