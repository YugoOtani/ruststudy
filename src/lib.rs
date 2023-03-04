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
    const YSH_LIM: usize = 1000;
    const YSH_COMPLEXITY: usize = 2; //0 to 5
    struct Gen {
        cnt: usize,
    }
    impl Gen {
        fn new() -> Gen {
            Gen { cnt: 0 }
        }
        fn reset(&mut self) {
            self.cnt = 0;
        }
        fn gen_ysh(&mut self) -> Ysh {
            self.cnt += 1;
            if self.cnt > YSH_LIM {
                return Ysh::Command(Command::new(Self::gen_com()).unwrap());
            }
            let mut rng = rand::thread_rng();
            match rng.gen_range(0..=6 + 6 - YSH_COMPLEXITY) {
                0 => y_seq(self.gen_ysh(), self.gen_ysh()),
                1 => y_and(self.gen_ysh(), self.gen_ysh()),
                2 => y_or(self.gen_ysh(), self.gen_ysh()),
                3 => y_pipe(self.gen_ysh(), self.gen_ysh()),
                4 => y_in(self.gen_ysh(), Self::gen_fname()),
                5 => y_out(self.gen_ysh(), Self::gen_fname()),
                6 => y_sub(self.gen_ysh()),
                _ => Ysh::Command(Command::new(Self::gen_com()).unwrap()),
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
    }
    #[test]
    fn parse_test() {
        let mut g = Gen::new();
        for _ in 0..10 {
            g.reset();
            let s = g.gen_ysh().to_string();
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
