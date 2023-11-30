use nix::{libc, unistd::{Pid, sleep}, sys::{signal::{kill, Signal}, wait::{waitpid, WaitPidFlag}}, errno::Errno};

use super::JudgeError;

struct ThreadData {
  child_pid: Pid,
  timeout: u32,
}

fn kill_process(child_pid: Pid) -> Result<(), Errno> {
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
      let _ = kill_process(child_pid);
      return Err(JudgeError::PthreadFailed);
    }

    match waitpid(child_pid, Some(WaitPidFlag::WSTOPPED)) {
        Ok(_) => {},
        Err(_) => {
          let _ = kill_process(child_pid);
          return Err(JudgeError::WaitFailed);
        },
    }

    libc::pthread_cancel(handle);
    Ok(())
  }
}

extern "C" fn kill_timeout(arg: *mut libc::c_void) -> *mut libc::c_void {
  unsafe {
    let thread_data: &ThreadData = &*(arg as *const ThreadData);
    // failed to set curr thread unjoinable
    if libc::pthread_detach(libc::pthread_self()) != 0 {
      let _ = kill_process(thread_data.child_pid);
    }
    if sleep(thread_data.timeout) != 0 {
      let _ = kill_process(thread_data.child_pid);
    }
    let _ = kill_process(thread_data.child_pid);
    libc::pthread_exit(core::ptr::null_mut());
  };
}
