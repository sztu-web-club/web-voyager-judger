use std::env;

pub fn init_env() {
  if read_env().is_err() {
    panic!("failed to load .env file")
  }
}

fn read_env() -> Result<std::path::PathBuf, dotenvy::Error> {
  #[cfg(debug_assertions)] {
    dotenvy::from_filename_override(".env.development")
  }
  #[cfg(not(debug_assertions))] {
    dotenvy::from_filename_override(".env.production")
  }
}

pub fn get_env(key: &str) -> String {
  match env::var(key) {
    Ok(value) => {
      value
    },
    Err(_) => {
      panic!("{} not found in .env file", key)
    },
  }
}
