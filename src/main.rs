use nix::{
    sys::wait::{wait, WaitStatus},
    unistd::{dup2, fork, ForkResult},
};
use pipe::{PipeReader, PipeWriter};
use rust::{yparser::*, ysh::*};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{stdin, stdout, Write};
use std::os::fd::AsRawFd;
use std::process;
fn main() -> Result<(), String> {
    for i in 1.. {
        print!("ysh[{i}] 🐈 > ");
        stdout().flush().unwrap();
        let mut buf = String::new();
        stdin().read_line(&mut buf).map_err(|e| e.to_string())?;
        match parse_ysh(&buf[..]) {
            Ok(res) => {
                res.debug();
                exec_proc(&res, Proc::Parent);
            }
            Err(err) => {
                println!("{err}");
            }
        }
        save_history(&buf[..])?;
    }
    Ok(())
    //todo: ctrl-c and ctrl-d, up key
}

const MAX_PIPE: usize = 16;

#[derive(Debug)]

enum Proc {
    Child,
    Parent,
}
#[derive(Debug, PartialEq)]
enum Status {
    Success,
    Fail,
}
impl Status {
    fn and(&self, s2: Status) -> Status {
        match (self, s2) {
            (Status::Success, Status::Success) => Status::Success,
            _ => Status::Fail,
        }
    }
}
fn save_history(s: &str) -> Result<(), String> {
    let mut f = OpenOptions::new()
        .append(true)
        .create(true)
        .open(HISTORY_PATH)
        .map_err(|e| e.to_string())?;
    f.write(s.as_bytes()).map_err(|e| e.to_string())?;
    Ok(())
}
fn exec_proc(ysh: &Ysh, p: Proc) -> Status {
    let is_parent = match p {
        Proc::Child => false,
        Proc::Parent => true,
    };
    match ysh {
        Ysh::Command(_) | Ysh::In(_, _) | Ysh::Out(_, _) | Ysh::Sub(_) => exec_fork(ysh, is_parent),
        Ysh::Pipe(_, _) | Ysh::Seq(_, _) | Ysh::And(_, _) | Ysh::Or(_, _) => exec_fork(ysh, false),
    }
}
fn exec_fork(ysh: &Ysh, fork_and_exec: bool) -> Status {
    if fork_and_exec {
        match unsafe { fork() } {
            Ok(ForkResult::Parent { child: _ }) => match wait() {
                Ok(WaitStatus::Exited(_, 0)) => Status::Success,
                _ => Status::Fail,
            },
            Ok(ForkResult::Child) => exec_impl(ysh),
            Err(e) => {
                println!("Fork failed : {e}");
                Status::Fail
            }
        }
    } else {
        exec_impl(ysh)
    }
}
fn multi_pipe(_: &Ysh, _: &Ysh, _: &Vec<(PipeReader, PipeWriter)>) -> Status {
    todo!()
}
fn exec_impl(y: &Ysh) -> Status {
    match y {
        Ysh::Command(com) => {
            stdout().flush().unwrap();
            com.exec();
            Status::Fail
        }
        Ysh::Pipe(ysh1, ysh2) => {
            let mut pipes = vec![];
            for _ in 0..MAX_PIPE {
                pipes.push(pipe::pipe())
            }
            multi_pipe(ysh1, ysh2, &pipes)
        }
        Ysh::In(ysh, fname) => {
            let fd_res = File::open(fname);
            match fd_res {
                Ok(_) => {
                    if let Ok(fd) = fd_res {
                        dup2(fd.as_raw_fd(), stdin().as_raw_fd()).expect("failed to duplicate");
                    } //close file here
                    exec_proc(ysh, Proc::Child)
                }
                Err(e) => {
                    println!("failed to open {fname} : {e}");
                    Status::Fail
                }
            }
        }
        Ysh::Out(ysh, fname) => {
            let fd_res = File::create(fname);
            match fd_res {
                Ok(_) => {
                    if let Ok(fd) = fd_res {
                        dup2(fd.as_raw_fd(), stdout().as_raw_fd()).expect("failed to duplicate");
                        //todo:change to pattern match
                    } //close file here
                    exec_proc(ysh, Proc::Child)
                }
                Err(e) => {
                    println!("failed to create {fname} : {e}");
                    Status::Fail
                }
            }
        }
        Ysh::Seq(ysh1, ysh2) => {
            let l = exec_proc(ysh1, Proc::Parent);
            let r = exec_proc(ysh2, Proc::Parent);
            l.and(r)
        }
        Ysh::And(ysh1, ysh2) => {
            if exec_proc(ysh1, Proc::Parent) == Status::Success {
                exec_proc(ysh2, Proc::Parent)
            } else {
                Status::Fail
            }
        }
        Ysh::Or(ysh1, ysh2) => {
            if exec_proc(ysh1, Proc::Parent) == Status::Fail {
                exec_proc(ysh2, Proc::Parent)
            } else {
                Status::Success
            }
        }
        Ysh::Sub(ysh) => {
            let status = exec_proc(ysh, Proc::Parent);
            process::exit(if status == Status::Success { 0 } else { 1 })
        }
    }
}
