use std::{fs, error::Error, env};

impl Config {
  pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
    args.next();

    let query = match args.next() {
      Some(arg) => arg,
      None => return Err("Invalid text"),
    };

    let filename = match args.next() {
      Some(arg) => arg,
      None => return Err("Invalid filename"),
    };

    Ok(Config {
      query,
      filename,
    })
  }

  pub fn read_config(&self) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&self.filename)?;
    for line in self.search(&content) {
      println!("{}", line);
    }
    Ok(())
  }

  pub fn search<'a>(&self, content: &'a str) -> Vec<&'a str> {
    content.lines()
      .filter(|line| line.contains(&self.query))
      .collect()
  }
}

pub struct Config {
  pub query: String,
  pub filename: String,
}
