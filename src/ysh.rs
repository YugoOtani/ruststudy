use std::os::unix::process::CommandExt;
use std::process;
pub const RESERVED_CHARS: &str = ";&|<>()";
pub const HISTORY_PATH: &str = "./command_history.txt"; //must be same as src/command/history.rs
pub const VALID_COMMAND: [&str; 13] = [
    "mv", "expr", "date", "ls", "cat", "pwd", "ps", "echo", "cp", "mkdir", "sleep", "sample",
    "history",
];
pub enum Ysh {
    Command(Box<dyn Command>),
    Seq(Box<Ysh>, Box<Ysh>),  // A; B
    And(Box<Ysh>, Box<Ysh>),  // A && B
    Or(Box<Ysh>, Box<Ysh>),   // A || B
    Pipe(Box<Ysh>, Box<Ysh>), // A | B
    In(Box<Ysh>, String),     // A < file
    Out(Box<Ysh>, String),    // A > file
    Sub(Box<Ysh>),            // (A)
}
impl Ysh {
    pub fn debug(&self) {
        fn go(ysh: &Ysh, indent: usize) {
            match ysh {
                Ysh::Command(com) => {
                    indent_n(indent);
                    println!("{}", com.debug());
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
        go(self, 0)
    }
    pub fn to_string(&self) -> String {
        match self {
            Ysh::Command(c) => c.to_string(),
            Ysh::Seq(l, r) => l.to_string() + " ; " + &r.to_string(),
            Ysh::And(l, r) => l.to_string() + " && " + &r.to_string(),
            Ysh::Or(l, r) => l.to_string() + " || " + &r.to_string(),
            Ysh::Pipe(l, r) => l.to_string() + " | " + &r.to_string(),
            Ysh::In(l, r) => l.to_string() + " < " + &r,
            Ysh::Out(l, r) => l.to_string() + " > " + &r,
            Ysh::Sub(y) => format!(" (") + &y.to_string() + ") ",
        }
    }
}

fn indent_n(n: usize) {
    print!("{}", "--".repeat(n));
}
pub trait Command {
    fn debug(&self) -> String;
    fn to_string(&self) -> String;
    fn exec(&self);
}
struct MyCommand {
    com: String,
    args: Vec<String>,
}
impl MyCommand {
    fn new(com: String, args: Vec<String>) -> Result<MyCommand, String> {
        if VALID_COMMAND.contains(&&com[..]) {
            Ok(MyCommand { com, args })
        } else {
            Err(format!(
                "{}(arg:{:?}) this is not a built in command",
                com, args
            ))
        }
    }
}

impl Command for MyCommand {
    fn debug(&self) -> String {
        format!("MyCom[{},{:?}]", self.com, self.args)
    }
    fn to_string(&self) -> String {
        self.com.to_string()
    }
    fn exec(&self) {
        process::Command::new(format!("./src/bin/{}", self.com))
            .args(&self.args)
            .exec();
    }
}

pub fn y_com(com: &str, args: &Vec<String>) -> Result<Ysh, String> {
    if let Ok(y) = MyCommand::new(com.to_string(), args.to_vec()) {
        Ok(Ysh::Command(Box::new(y)))
    } else {
        Err(format!("{}(args:{:?}) is not a valid command", com, args))
    }
}
pub fn y_seq(l: Ysh, r: Ysh) -> Ysh {
    Ysh::Seq(Box::new(l), Box::new(r))
}
pub fn y_and(l: Ysh, r: Ysh) -> Ysh {
    Ysh::And(Box::new(l), Box::new(r))
}
pub fn y_or(l: Ysh, r: Ysh) -> Ysh {
    Ysh::Or(Box::new(l), Box::new(r))
}
pub fn y_pipe(l: Ysh, r: Ysh) -> Ysh {
    Ysh::Pipe(Box::new(l), Box::new(r))
}
pub fn y_in(l: Ysh, s: &str) -> Ysh {
    Ysh::In(Box::new(l), s.to_string())
}
pub fn y_out(l: Ysh, s: &str) -> Ysh {
    Ysh::Out(Box::new(l), s.to_string())
}
pub fn y_sub(y: Ysh) -> Ysh {
    Ysh::Sub(Box::new(y))
}
