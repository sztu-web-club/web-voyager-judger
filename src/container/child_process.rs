use std::{fs::OpenOptions, os::fd::{AsRawFd, RawFd}, io::{stdin, stdout, stderr}, ffi::CString};
use nix::{unistd::{dup2, execve}, sys::resource::{setrlimit, Resource}};

use super::{lang_config::ResourceLimit, RunConfig, JudgeResult, JudgeError};


pub fn run_child_process(config: RunConfig, res_limit: Box<dyn ResourceLimit>) -> Result<JudgeResult, JudgeError> {
  // setup resource limit
  return match setrlimit(Resource::RLIMIT_AS, res_limit.max_memory(), res_limit.max_memory()).and_then(|()| {
    setrlimit(Resource::RLIMIT_STACK, res_limit.max_stack(), res_limit.max_stack()).and_then(|()| {
      setrlimit(Resource::RLIMIT_CPU, res_limit.max_cpu_time(), res_limit.max_cpu_time()).and_then(|()| {
        setrlimit(Resource::RLIMIT_NPROC, res_limit.max_process(), res_limit.max_process()).and_then(|()| {
          setrlimit(Resource::RLIMIT_FSIZE, res_limit.max_output(), res_limit.max_output())
        })
      })
    })
  }) {
    Ok(_) => {
      let mut infile_fd: RawFd = 0;
      let mut outfile_fd: RawFd = 0;
      match OpenOptions::new().read(true).open(config.infile_path).and_then(|infile| Ok({
        OpenOptions::new().write(true).open(config.outfile_path).and_then(|outfile| Ok({
          infile_fd = infile.as_raw_fd();
          outfile_fd = outfile.as_raw_fd();
        }))
      })) {
        Ok(_) => {
          if infile_fd == 0 || outfile_fd == 0 {
            return Err(JudgeError::Dup2Failed);
          }
          match dup2(infile_fd, stdin().as_raw_fd()).and_then(|_| {
            dup2(outfile_fd, stdout().as_raw_fd()).and_then(|_| {
              dup2(outfile_fd, stderr().as_raw_fd())
            })
          }) {
            Ok(_) => {
              // TODO: set gid and uid to nobody
              match Ok::<(), JudgeError>(()) {
                Ok(_) => {
                  // TODO: set seccomp
                  match Ok::<(), JudgeError>(()) {
                    Ok(_) => {
                      let exec_path = CString::new(config.exec_path);
                      let exec_args = CString::new(config.exec_args);
                      let exec_envs = CString::new(config.exec_envs);
                      // process image replaced so no more code below executed expect execve failed
                      let _ = execve(&exec_path.unwrap(), &[exec_args.unwrap()], &[exec_envs.unwrap()]);
                      return Err(JudgeError::ExecveFailed)
                    },
                    Err(_) => Err(JudgeError::SeccompFailed),
                  }
                },
                Err(_) => Err(JudgeError::NobodyRequired),
              }
            },
            Err(_) => Err(JudgeError::Dup2Failed),
          }
        },
        Err(_) => Err(JudgeError::Dup2Failed),
      }
    },
    Err(_) => Err(JudgeError::SetrlimitFailed),
  };
}
