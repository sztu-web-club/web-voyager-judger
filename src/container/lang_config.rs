use nix::libc::rlim_t;

static STD_MB: u64 = 0x100000;

pub trait ResourceLimit {
  fn max_cpu_time(&self) -> rlim_t;
  fn max_real_time(&self) -> rlim_t;
  fn max_memory(&self) -> rlim_t;
  fn max_stack(&self) -> rlim_t;
  fn max_process(&self) -> rlim_t;
  fn max_output(&self) -> rlim_t;
}

pub struct CorCPPResLimit {}

impl ResourceLimit for CorCPPResLimit {
  fn max_cpu_time(&self) -> rlim_t {
    return 1
  }

  fn max_real_time(&self) -> rlim_t {
    return 5
  }

  fn max_memory(&self) -> rlim_t {
    return STD_MB / 2 * 3
  }

  fn max_stack(&self) -> rlim_t {
    return STD_MB << 8
  }

  fn max_process(&self) -> rlim_t {
    return 5
  }

  fn max_output(&self) -> rlim_t {
    return 1000
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
