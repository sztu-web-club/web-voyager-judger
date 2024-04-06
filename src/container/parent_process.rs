use nix::{errno::Errno, libc::{self, rusage}, sys::{signal::{kill, Signal}, wait::{waitpid, WaitPidFlag}}, unistd::{sleep, Pid}};

use super::JudgeError;

struct ThreadData {
  child_pid: Pid,
  timeout: u32,
}

fn kill_process(child_pid: Pid) -> Result<(), Errno> {
  println!("kill timeout executed");
  kill(child_pid, Signal::SIGKILL)
}

pub unsafe fn require_usage(time_limit: u32, child_pid: Pid) -> Result<(), JudgeError> {
  // since Rust has no POSIX thread support, we must do it in unsafe
  unsafe {
    let mut handle: libc::pthread_t = std::mem::zeroed();
    let attr = core::ptr::null_mut();

    if libc::pthread_create(
        &mut handle,
        attr,
        kill_timeout,
        &ThreadData {
          child_pid,
          timeout: time_limit,
        } as *const ThreadData as *mut libc::c_void,
    ) != 0
    {
      println!("pthread_create failed");
      let _ = kill_process(child_pid);
      return Err(JudgeError::PthreadFailed);
    }
    println!("pthread_create success");
    let status: libc::c_int = 0;
    let resource_usage: rusage = std::mem::zeroed();
    if waitpid(child_pid, Some(WaitPidFlag::WNOHANG)).is_err() {
      println!("waitpid failed");
      let _ = kill_process(child_pid);
      return Err(JudgeError::WaitFailed);
    }
    println!("waitpid success");
    libc::pthread_cancel(handle);
    Ok(())
  }
}

extern "C" fn kill_timeout(arg: *mut libc::c_void) -> *mut libc::c_void {
  unsafe {
    println!("kill_timeout started");
    let thread_data: &ThreadData = &*(arg as *const ThreadData);
    // failed to set curr thread unjoinable
    if libc::pthread_detach(libc::pthread_self()) != 0 {
      println!("pthread_detach failed");
      let _ = kill_process(thread_data.child_pid);
    }
    println!("pthread_detach success");
    if sleep(thread_data.timeout) != 0 {
      println!("sleep failed");
      let _ = kill_process(thread_data.child_pid);
    }
    println!("sleep success");
    let _ = kill_process(thread_data.child_pid);
    core::ptr::null_mut()
  }
}
