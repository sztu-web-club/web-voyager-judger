mod seccomp;
mod lang_config;
mod child_process;
mod parent_process;

use lang_config::ResourceLimit;
pub use lang_config::DefaultResLimit;

use log::debug;
use nix::unistd::{fork, ForkResult};

pub struct RunConfig {
  pub exec_path: String,
  pub exec_args: String,
  pub exec_envs: String,
  pub infile_path: String,
  pub outfile_path: String,
  pub uid: i32
}


pub struct RunResult {
  cpu_time: u32,
  real_time: u32,
  memory: u64,
  signal: i32,
  exit_code: i32,
  error: JudgeError,
  result: JudgeResult
}

#[derive(Debug)]
pub enum JudgeError {
  RunConfigInvalid,
  ForkFailed,
  PthreadFailed,
  WaitFailed,
  OpenFileFailed,
  Dup2Failed,
  SetrlimitFailed,
  SetuidFailed,
  SeccompFailed,
  ExecveFailed,
  NobodyRequired
}

pub enum JudgeResult {
  Waiting,
  Compiling,
  Running,
  Accepted,
  PresentationError,
  WrongAnswer,
  TimeLimitExceed,
  MemoryLimitExceed,
  OutputLimitExceed,
  RuntimeError,
  ComplieError,
  SystemError
}

pub fn run(config: RunConfig, res_limit: Box<dyn ResourceLimit>) -> Result<(), JudgeError> {
  match unsafe{fork()} {
    Ok(ForkResult::Parent { child, .. }) => {
      unsafe {
        println!("parent started");
        match parent_process::require_usage(
          res_limit.max_real_time().try_into().unwrap(),
          child
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
      }
    }
    Ok(ForkResult::Child) => {
      println!("child started");
      match child_process::run_child_process(config, res_limit) {
        Ok(_) => todo!(),
        Err(e) => Err(e),
      }
    }
    Err(_) => Err(JudgeError::ForkFailed),
  }
}
