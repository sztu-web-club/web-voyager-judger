use std::{fs::{OpenOptions, File}, os::fd::{AsRawFd, RawFd}, io::{stdin, stdout, stderr}, ffi::CString};
use log::{error, debug};
use nix::{unistd::{dup2, execve}, sys::resource::{setrlimit, Resource}, libc, errno::Errno};

use super::{lang_config::ResourceLimit, RunConfig, JudgeResult, JudgeError, seccomp::prepare_seccomp};

fn setup_res_limit(res_limit: Box<dyn ResourceLimit>) -> Result<(), Errno> {
  // setup resource limit
  return setrlimit(Resource::RLIMIT_AS, res_limit.max_memory(), res_limit.max_memory()).and_then(|()| {
    setrlimit(Resource::RLIMIT_STACK, res_limit.max_stack(), res_limit.max_stack()).and_then(|()| {
      setrlimit(Resource::RLIMIT_CPU, res_limit.max_cpu_time(), res_limit.max_cpu_time()).and_then(|()| {
        setrlimit(Resource::RLIMIT_NPROC, res_limit.max_process(), res_limit.max_process()).and_then(|()| {
          setrlimit(Resource::RLIMIT_FSIZE, res_limit.max_output(), res_limit.max_output())
        })
      })
    })
  });
}

fn open_file(infile_path: &String, outfile_path: &String) -> Result<(RawFd, RawFd), std::io::Error> {
  let infile = File::create(infile_path);
  if infile.is_err() {
    return Err(infile.unwrap_err());
  }
  let outfile = File::create(outfile_path);
  if outfile.is_err() {
    return Err(outfile.unwrap_err());
  }
  return Ok((infile.unwrap().as_raw_fd(), outfile.unwrap().as_raw_fd()));
}

fn redirect_file(infile_fd: RawFd, outfile_fd: RawFd) -> Result<(), JudgeError> {
  unsafe {
    if libc::dup2(outfile_fd, stdout().as_raw_fd()) == -1 {
      return Err(JudgeError::Dup2Failed);
    }
    if libc::dup2(outfile_fd, stderr().as_raw_fd()) == -1 {
      return Err(JudgeError::Dup2Failed);
    }
    if libc::dup2(infile_fd, stdin().as_raw_fd()) == -1 {
      return Err(JudgeError::Dup2Failed);
    }
  }
  return Ok(());
}

pub fn run_child_process(config: RunConfig, res_limit: Box<dyn ResourceLimit>) -> Result<JudgeResult, JudgeError> {
  // convert to CString in advance to avoid execve call with different pointer
  let exec_path = CString::new(config.exec_path).unwrap();
  let exec_args = CString::new(config.exec_args).unwrap();
  let exec_envs = CString::new(config.exec_envs).unwrap();

  println!("prepare_seccomp success");
  // setup resource limit
  let setup_res_limit_res = setup_res_limit(res_limit);
  if setup_res_limit_res.is_err() {
    println!("setup_res_limit failed");
    return Err(JudgeError::SetrlimitFailed)
  }
  println!("setup_res_limit success");

  let open_file_res = open_file(&config.infile_path, &config.outfile_path);
  if open_file_res.is_err() {
    println!("open_file failed");
    return Err(JudgeError::OpenFileFailed)
  }
  println!("open_file success");

  let in_out_fd = open_file_res.unwrap();
  println!("{} {}", in_out_fd.0, in_out_fd.1);
  let seccomp_res = prepare_seccomp(exec_path.as_ptr() as u64);
  if seccomp_res.is_err() {
    println!("prepare_seccomp failed");
    return Err(JudgeError::SeccompFailed)
  }
  println!("prepare_seccomp success");

  // TODO: unshare namespace

  // TODO: set cgroup

  // TODO: set chroot

  // TODO: set gid and uid to nobody

  let redirect_file_res = redirect_file(in_out_fd.1, in_out_fd.0);
  if redirect_file_res.is_err() {
    println!("redirect_file failed");
    return Err(JudgeError::Dup2Failed)
  }
  println!("redirect_file success");

  // process image replaced so no more code below executed expect execve failed
  let _ = execve(&exec_path, &[exec_args], &[exec_envs]);
  return Err(JudgeError::ExecveFailed);
}
