use std::env::args;
use std::fs::File;
use std::io::prelude::*;
const MAX_HISTORY: usize = 100;
const HISTORY_PATH: &str = "./command_history.txt";
//must be same as ysh.rs::HISTORY_PATH
//maybe it's better to exec history in main.rs ?
struct Opt {
    nlines: usize,
}
impl Opt {
    fn default() -> Opt {
        Opt { nlines: 5 }
    }
    fn parse(opt: &Vec<String>) -> Opt {
        let mut ret = Opt::default();
        //first argument is file path -> ignore
        let mut i = 1;
        while i < opt.len() {
            match &opt[i].replace(" ", "")[..] {
                "-n" => {
                    if i + 1 >= opt.len() {
                        println!("args for option -n is not given");
                    } else {
                        match opt[i + 1].parse() {
                            Ok(n) => {
                                ret.nlines = n;
                                i += 1;
                            }
                            Err(_) => println!("args for option -n must be an integer"),
                        }
                    }
                }
                s => println!("option {s} is not supported"),
            }
            i += 1;
        }
        ret
    }
}
fn main() {
    let opt = Opt::parse(&args().collect());
    let mut file = File::open(HISTORY_PATH).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    let com: Vec<String> = buf.lines().map(String::from).collect();

    // print
    let st = if com.len() > opt.nlines {
        com.len() - opt.nlines
    } else {
        0
    };
    for i in st..com.len() {
        println!("{}", com[i]);
    }
    // rewrite file
    let mut file = File::options()
        .write(true)
        .truncate(true)
        .open(HISTORY_PATH)
        .unwrap();
    let st = if com.len() < MAX_HISTORY {
        0
    } else {
        MAX_HISTORY - com.len()
    };
    for i in st..com.len() {
        file.write(com[i].as_bytes()).unwrap();
        file.write(b"\n").unwrap();
    }
}
//TODO:check file line length
