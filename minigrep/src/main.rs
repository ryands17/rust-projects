use std::{env, process};
use minigrep::{Config};

fn main() {
  let config = Config::new(env::args()).unwrap_or_else(|err| {
    eprintln!("Problem parsing: {}", err);
    process::exit(1);
  });

  if let Err(_e) = config.read_config() {
    eprintln!("Cannot read file: {}", config.filename);
    process::exit(1);
  }
}
