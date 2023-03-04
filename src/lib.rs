pub mod yparser;
#[cfg(test)]
mod parse_test {
    use crate::yparser::*;
    use rand::Rng;
    impl Ysh {
        fn to_string(&self) -> String {
            match self {
                Ysh::Command(c) => {
                    format!("{}", c.com)
                }
                Ysh::Seq(l, r) => l.to_string() + " ; " + &r.to_string(),
                Ysh::And(l, r) => l.to_string() + " && " + &r.to_string(),
                Ysh::Or(l, r) => l.to_string() + " ||  " + &r.to_string(),
                Ysh::Pipe(l, r) => l.to_string() + "  | " + &r.to_string(),
                Ysh::In(l, r) => l.to_string() + " < " + &r,
                Ysh::Out(l, r) => l.to_string() + "> " + &r,
                Ysh::Sub(y) => format!("(") + &y.to_string() + ") ",
            }
        }
    }
    fn gen_ysh() -> Ysh {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=10) {
            0 => y_seq(gen_ysh(), gen_ysh()),
            1 => y_and(gen_ysh(), gen_ysh()),
            2 => y_or(gen_ysh(), gen_ysh()),
            3 => y_pipe(gen_ysh(), gen_ysh()),
            4 => y_in(gen_ysh(), gen_fname()),
            5 => y_out(gen_ysh(), gen_fname()),
            6 => y_sub(gen_ysh()),
            _ => Ysh::Command(Command::new(gen_com()).unwrap()),
        }
    }
    fn gen_fname() -> String {
        "file.txt".to_string()
    }
    fn gen_com() -> Vec<String> {
        let mut rng = rand::thread_rng();
        let l = VALID_COMMAND.len();
        vec![VALID_COMMAND[rng.gen_range(0..l)].to_string()]
    }
    #[test]
    fn ptest() {
        for _ in 0..10 {
            let s = gen_ysh().to_string();
            println!("{s}");
            if let Err(e) = parse_ysh(s) {
                println!("{e}");
                assert!(false);
            } else {
                println!(" ===> success!");
            }
        }
    }
}
