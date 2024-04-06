mod config;
mod container;
mod compiler;

use std::{fs::File, os::fd::AsRawFd};

use log::{error, debug};
use nix::{unistd::Uid, libc};
use tide::Request;

use container::{run, RunConfig};
use config::{init_env, get_env};

use crate::compiler::{compile, LangType};

#[async_std::main]
async fn main() -> tide::Result<()> {
  env_logger::init();
  #[cfg(not(target_os = "linux"))] {
    println!("You must run on linux");
    panic!("You must run on linux")
  }
  if !Uid::effective().is_root() {
    println!("You must run as root");
    panic!("You must run as root")
  }
  init_env();
  let mut app = tide::new();
  app.at("/orders/shoes").post(order_shoes);
  println!("Listening on port {}", get_env("PORT"));
  app.listen(format!("127.0.0.1:{}", get_env("PORT"))).await?;
  Ok(())
}

async fn order_shoes(mut req: Request<()>) -> tide::Result {
  println!("order_shoes");
  let artifacts = compile(LangType::Cpp, "#include <iostream>\nint main() { std::cout << \"Hello, World!\"; return 0; }");
  let run_res = run(RunConfig {
    exec_path: artifacts.unwrap(),
    exec_args: "".to_owned(),
    exec_envs: "".to_owned(),
    infile_path: "input.txt".to_owned(),
    outfile_path: "output.txt".to_owned(),
    uid: 65536,
  }, Box::new(container::DefaultResLimit {}));
  if run_res.is_err() {
    println!("{:?}", run_res.err().unwrap());
    return Ok(format!("error").into())
  }
  Ok(format!("runed").into())
}
