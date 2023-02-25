use std::any::type_name;
use std::io;
use std::io::Write;
use std::{thread, time};
fn interpret_line(input: String) {
    if &input[..] == "exit" {
        std::process::exit(0);
    }
    println!("{:?}", input);
}
fn main() {
    for i in 1.. {
        print!("ysh[{i}] 🐈 > ");
        io::stdout().flush().unwrap();
        let mut buf = String::new();
        match io::stdin().read_line(&mut buf) {
            Ok(_) => interpret_line(buf.lines().collect()),
            Err(error) => println!("error: {error}"),
        }
    }
    //todo: ctrl-c and ctrl-d
}

fn tn<T>(_: T) {
    println!("{:?}", type_name::<T>())
}
