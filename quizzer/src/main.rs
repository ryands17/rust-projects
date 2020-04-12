use std::{process,io};
use clap::{App, load_yaml};
use quizzer::{Quizzer};
use std::io::Write;

fn main() {
  let yml = load_yaml!("cli.yml");
  let inputs = App::from(yml).get_matches();
  let filename = inputs.value_of("csv").unwrap().to_string();

  let config = Quizzer::new(filename);

  let problems = config.parse_lines().unwrap_or_else(|e| {
    eprintln!("Cannot read file: {}", e.to_string());
    process::exit(1);
  });
  for problem in problems {
    println!("Question: {}", problem.question);
    let mut input = String::new();

    print!("Answer: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
  }
}
