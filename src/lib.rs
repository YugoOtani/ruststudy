pub mod yparser;
pub mod ysh;
#[cfg(test)]
mod parse_test {
    use crate::yparser::*;
    use crate::ysh::*;
    use rand::Rng;

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
                let (c, arg) = Self::gen_com();
                return y_com(c, arg)
                    .expect("gen_com must generate valid command : {},{:?} was given");
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
                _ => {
                    let (c, arg) = Self::gen_com();
                    y_com(c, arg).expect("gen_com mut generate valid command")
                }
            }
        }
        fn gen_fname() -> String {
            "file.txt".to_string()
        }
        fn gen_com() -> (String, Vec<String>) {
            let mut rng = rand::thread_rng();
            let l = VALID_COMMAND.len();
            (VALID_COMMAND[rng.gen_range(0..l)].to_string(), vec![])
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
