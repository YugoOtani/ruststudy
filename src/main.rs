use nix::{
    sys::wait::{wait, WaitStatus},
    unistd::{dup2, fork, ForkResult},
};

use pipe::{PipeReader, PipeWriter};
use rust::yparser::*;
use std::fs::File;
use std::io::{stdin, stdout, Write};
use std::os::fd::{AsRawFd, RawFd};
use std::os::unix::process::CommandExt;
use std::process;

fn main() {
    for i in 1.. {
        print!("ysh[{i}] 🐈 > ");
        stdout().flush().unwrap();
        let mut buf = String::new();
        match stdin().read_line(&mut buf) {
            Ok(_) => match p_ysh().parse(buf.as_bytes()) {
                Ok(res) => {
                    println!("{:?}", res);
                    exec_proc(&res, Proc::Parent);
                }
                Err(e) => println!("{:?}", e),
            },
            Err(error) => println!("error: {error}"),
        }
    }
    //todo: ctrl-c and ctrl-d
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

//https://doc.rust-lang.org/std/os/unix/process/trait.CommandExt.html#tymethod.exec

fn exec_cmd(c: &Command) -> Status {
    process::Command::new(format!("/bin/{}", c.com))
        .args(&c.args)
        .exec();
    Status::Fail
}
fn exec_cmd_fork(c: &Command) -> Status {
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child: _ }) => match wait() {
            Ok(WaitStatus::Exited(_, 0)) => Status::Success,
            e => {
                println!("{:?}", e);
                Status::Fail
            }
        },
        Ok(ForkResult::Child) => exec_cmd(&c),
        Err(e) => {
            println!("Fork failed : {e}");
            Status::Fail
        }
    }
}
fn exec_proc(ysh: &Ysh, p: Proc) -> Status {
    let is_parent = match p {
        Proc::Child => false,
        Proc::Parent => true,
    };
    match ysh {
        Ysh::YCommand(_) => exec_fork(ysh, is_parent),
        Ysh::YPipe(_, _) => exec_fork(ysh, false),
        Ysh::YIn(_, _) => exec_fork(ysh, is_parent),
        Ysh::YOut(_, _) => exec_fork(ysh, is_parent),
        Ysh::YSeq(_, _) => exec_fork(ysh, false),
        Ysh::YAnd(_, _) => exec_fork(ysh, false),
        Ysh::YOr(_, _) => exec_fork(ysh, false),
        Ysh::YSub(_) => exec_fork(ysh, is_parent),
    }
}
fn exec_fork(ysh: &Ysh, fork_and_exec: bool) -> Status {
    if fork_and_exec {
        match unsafe { fork() } {
            Ok(ForkResult::Parent { child: _ }) => match wait() {
                Ok(WaitStatus::Exited(_, 0)) => Status::Success,
                e => {
                    println!("{:?}", e);
                    Status::Fail
                }
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
fn multi_pipe(fst: &Command, ysh: &Ysh, pipes: &Vec<(PipeReader, PipeWriter)>) -> Status {
    todo!()
}
fn exec_impl(ysh: &Ysh) -> Status {
    match ysh {
        Ysh::YCommand(com) => {
            stdout().flush().unwrap();
            exec_cmd(com)
        }
        Ysh::YPipe(com, ysh) => {
            let mut pipes = vec![];
            for _ in 0..MAX_PIPE {
                pipes.push(pipe::pipe())
            }
            multi_pipe(com, ysh, &pipes)
        }
        Ysh::YIn(ysh, fname) => {
            let fd_res = File::open(fname);
            match fd_res {
                Ok(_) => {
                    if let Ok(fd) = fd_res {
                        dup2(fd.as_raw_fd(), stdin().as_raw_fd()).expect("failed to duplicate");
                        //todo:change to pattern match
                    } //close file here
                    exec_proc(ysh, Proc::Child)
                }
                Err(e) => {
                    println!("failed to open {fname} : {e}");
                    Status::Fail
                }
            }
        }
        Ysh::YOut(ysh, fname) => {
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
        Ysh::YSeq(com, ysh) => {
            let l = exec_cmd_fork(com);
            let r = exec_proc(ysh, Proc::Parent);
            l.and(r)
        }
        Ysh::YAnd(com, ysh) => {
            if exec_cmd_fork(com) == Status::Success {
                exec_proc(ysh, Proc::Parent)
            } else {
                Status::Fail
            }
        }
        Ysh::YOr(com, ysh) => {
            if exec_cmd_fork(com) == Status::Fail {
                exec_proc(ysh, Proc::Parent)
            } else {
                Status::Success
            }
        }
        Ysh::YSub(ysh) => exec_proc(ysh, Proc::Parent),
    }
}
