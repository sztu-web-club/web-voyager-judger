use std::{ffi::CString, fs::{File, OpenOptions}, io::{stderr, stdin, stdout, Read}, os::fd::AsRawFd};
use nix::{unistd::execve, sys::resource::{setrlimit, Resource}, libc, errno::Errno};

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

fn open_file(infile_path: &String, outfile_path: &String) -> Result<(File, File), std::io::Error> {
  let infile = OpenOptions::new().read(true).write(true).open(infile_path);
  if infile.is_err() {
    return Err(infile.unwrap_err());
  }
  let mut buf = String::new();
  infile.as_ref().unwrap().read_to_string(&mut buf)?;
  let outfile = OpenOptions::new().read(true).write(true).open(outfile_path);
  if outfile.is_err() {
    return Err(outfile.unwrap_err());
  }
  let mut buf2 = String::new();
  outfile.as_ref().unwrap().read_to_string(&mut buf2)?;
  return Ok((infile.unwrap(), outfile.unwrap()));
}

fn redirect_file(infile: &File, outfile: &File) -> Result<(), JudgeError> {
  unsafe {
    if libc::dup2(outfile.as_raw_fd(), stdout().as_raw_fd()) == -1 {
      return Err(JudgeError::Dup2Failed);
    }
    if libc::dup2(outfile.as_raw_fd(), stderr().as_raw_fd()) == -1 {
      return Err(JudgeError::Dup2Failed);
    }
    if libc::dup2(infile.as_raw_fd(), stdin().as_raw_fd()) == -1 {
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

  // setup resource limit
  let setup_res_limit_res = setup_res_limit(res_limit);
  if setup_res_limit_res.is_err() {
    return Err(JudgeError::SetrlimitFailed)
  }

  let open_file_res = open_file(&config.infile_path, &config.outfile_path);
  if open_file_res.is_err() {
    return Err(JudgeError::OpenFileFailed)
  }

  let in_out_fd = open_file_res.unwrap();
  let seccomp_res = prepare_seccomp(exec_path.as_ptr() as u64);
  if seccomp_res.is_err() {
    return Err(JudgeError::SeccompFailed)
  }

  // TODO: unshare namespace

  // TODO: set cgroup

  // TODO: set chroot

  // TODO: set gid and uid to nobody

  let redirect_file_res = redirect_file(&in_out_fd.0, &in_out_fd.1);
  if redirect_file_res.is_err() {
    return Err(JudgeError::Dup2Failed)
  }

  // process image replaced so no more code below executed expect execve failed
  let _ = execve(&exec_path, &[exec_args], &[exec_envs]);
  return Err(JudgeError::ExecveFailed);
}
