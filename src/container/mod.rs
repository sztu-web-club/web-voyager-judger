mod seccomp;
mod lang_config;
mod child_process;
mod parent_process;
use lang_config::ResourceLimit;

use nix::unistd::{fork, ForkResult};

pub struct RunConfig {
  exec_path: String,
  exec_args: String,
  exec_envs: String,
  infile_path: String,
  outfile_path: String,
  uid: i32
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

pub enum JudgeError {
  RunConfigInvalid,
  ForkFailed,
  PthreadFailed,
  WaitFailed,
  Dup2Failed,
  SetrlimitFailed,
  SetuidFailed,
  SeccompFailed,
  ExecveFailed,
  RootRequired,
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

pub fn run(config: RunConfig, res_limit: Box<dyn ResourceLimit>) -> () {
  match unsafe{fork()} {
    Ok(ForkResult::Parent { child, .. }) => {
      unsafe {
        let _ = match parent_process::require_usage(
          res_limit.max_real_time().try_into().unwrap(),
          child
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        };
      };
    }
    Ok(ForkResult::Child) => {
      match child_process::run_child_process(config, res_limit) {
        Ok(_) => todo!(),
        Err(e) => {},
      };
    }
    Err(_) => {},
  }
}
