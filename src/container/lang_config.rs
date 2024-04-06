use nix::libc::rlim_t;

pub trait ResourceLimit {
  /// max time cpu being used in seconds
  fn max_cpu_time(&self) -> rlim_t;
  /// max time process running in seconds
  fn max_real_time(&self) -> rlim_t;
  /// max virtual memory in bytes
  fn max_memory(&self) -> rlim_t;
  /// max stack size in bytes
  fn max_stack(&self) -> rlim_t;
  /// max process number
  fn max_process(&self) -> rlim_t;
  /// max output(stdout stderr) size in bytes
  fn max_output(&self) -> rlim_t;
}

pub struct DefaultResLimit {}

impl ResourceLimit for DefaultResLimit {
  fn max_cpu_time(&self) -> rlim_t {
    return 50
  }

  fn max_real_time(&self) -> rlim_t {
    return 50
  }

  fn max_memory(&self) -> rlim_t {
    return 0x8000000
  }

  fn max_stack(&self) -> rlim_t {
    return 0x8000000
  }

  fn max_process(&self) -> rlim_t {
    return 10
  }

  fn max_output(&self) -> rlim_t {
    return 0x8000000
  }
}

pub struct CustomResLimit {
  max_cpu_time: rlim_t,
  max_real_time: rlim_t,
  max_memory: rlim_t,
  max_stack: rlim_t,
  max_process: rlim_t,
  max_output: rlim_t
}

impl CustomResLimit {
  fn new(
    max_cpu_time: rlim_t,
    max_real_time: rlim_t,
    max_memory: rlim_t,
    max_stack: rlim_t,
    max_process: rlim_t,
    max_output: rlim_t
  ) -> CustomResLimit {
    CustomResLimit {
      max_cpu_time,
      max_real_time,
      max_memory,
      max_stack,
      max_process,
      max_output
    }
  }
}

impl ResourceLimit for CustomResLimit {
  fn max_cpu_time(&self) -> rlim_t {
    return self.max_cpu_time
  }

  fn max_real_time(&self) -> rlim_t {
    return self.max_real_time
  }

  fn max_memory(&self) -> rlim_t {
    return self.max_memory
  }

  fn max_stack(&self) -> rlim_t {
    return self.max_stack
  }

  fn max_process(&self) -> rlim_t {
    return self.max_process
  }

  fn max_output(&self) -> rlim_t {
    return self.max_output
  }
}
