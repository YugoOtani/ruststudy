pub const RESERVED_CHARS: &str = ";&|<>()";
pub const VALID_COMMAND: [&str; 9] = [
    "ls", "cat", "pwd", "ps", "echo", "cp", "kill", "mkdir", "sleep",
];
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
impl Ysh {
    pub fn debug(&self) {
        fn go(ysh: &Ysh, indent: usize) {
            match ysh {
                Ysh::Command(com) => {
                    indent_n(indent);
                    com.debug();
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
            Ysh::Command(c) => {
                format!("{}", c.com)
            }
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

#[derive(Debug)]
pub struct Command {
    pub com: String,
    pub args: Vec<String>,
}

impl Command {
    fn new(com: String, args: Vec<String>) -> Result<Command, String> {
        Ok(Command { com, args })
    }
    fn debug(&self) {
        println!("Com[{},{:?}]", self.com, self.args);
    }
}
pub fn y_com(com: String, args: Vec<String>) -> Result<Ysh, String> {
    Ok(Ysh::Command(Command::new(com.to_string(), args.to_vec())?))
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
pub fn y_in(l: Ysh, s: String) -> Ysh {
    Ysh::In(Box::new(l), s)
}
pub fn y_out(l: Ysh, s: String) -> Ysh {
    Ysh::Out(Box::new(l), s)
}
pub fn y_sub(y: Ysh) -> Ysh {
    Ysh::Sub(Box::new(y))
}
