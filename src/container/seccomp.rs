use log::debug;
use nix::libc::{self, O_WRONLY, O_RDWR, O_CREAT};
use seccompiler::{
  BpfProgram, SeccompAction, SeccompCmpArgLen, SeccompCmpOp, SeccompCondition, SeccompFilter,
  SeccompRule, Error as SeccompError,
};
use std::convert::TryInto;

pub fn prepare_seccomp(exe_path_ptr: u64) -> Result<(), SeccompError> {
  let syscalls_whitelist = vec![
    libc::SYS_read,
    libc::SYS_fstat,
    libc::SYS_mmap,
    libc::SYS_mprotect,
    libc::SYS_munmap,
    libc::SYS_uname,
    libc::SYS_arch_prctl,
    libc::SYS_brk,
    libc::SYS_access,
    libc::SYS_exit_group,
    libc::SYS_close,
    libc::SYS_readlink,
    libc::SYS_sysinfo,
    libc::SYS_write,
    libc::SYS_writev,
    libc::SYS_lseek,
    libc::SYS_clock_gettime,
  ];
  let syscall_tuples: Vec<(i64, Vec<SeccompRule>)> = syscalls_whitelist
    .into_iter()
    .map(|syscall| (syscall, Vec::new()))
    .collect();
  let special_syscall_tuples: Vec<(i64, Vec<SeccompRule>)> = vec![
      (libc::SYS_execve,
        vec![
          SeccompRule::new(vec![
              SeccompCondition::new(
                0,
                SeccompCmpArgLen::Dword,
                SeccompCmpOp::Eq,
                exe_path_ptr
              )?
            ]
          )?
        ]
      )
    ];
  let ro_syscall_tuples: Vec<(i64, Vec<SeccompRule>)> = vec![
    (libc::SYS_open,
      vec![
        SeccompRule::new(vec![
            SeccompCondition::new(
              1,
              SeccompCmpArgLen::Dword,
              SeccompCmpOp::MaskedEq((O_WRONLY | O_RDWR | O_CREAT).try_into().unwrap()),
              0)?
          ]
        )?
      ]
    ),
    (libc::SYS_openat,
      vec![
        SeccompRule::new(vec![
            SeccompCondition::new(
              1,
              SeccompCmpArgLen::Dword,
              SeccompCmpOp::MaskedEq((O_WRONLY | O_RDWR | O_CREAT).try_into().unwrap()),
              0)?
          ]
        )?
      ]
    ),
  ];
  let filter: BpfProgram = SeccompFilter::new(
      // [
        // syscall_tuples,
        // special_syscall_tuples,
        // ro_syscall_tuples
      // ].concat()
      // .into_iter()
      // .collect(),
      vec![].into_iter().collect(),
      SeccompAction::Allow,
      SeccompAction::KillThread,
      std::env::consts::ARCH.try_into()?,
  )?.try_into()?;
  println!("seccomp filter generated");
  match seccompiler::apply_filter(&filter) {
    Ok(_) => {
      println!("seccomp filter applied");
    },
    Err(e) => {
      println!("seccomp filter failed");
      return Err(e)
    }
  }
  println!("seccomp filter applied");
  Ok(())
}
