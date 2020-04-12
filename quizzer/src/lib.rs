use std::{error::Error, fs};

impl Quizzer {
  pub fn new(filename: String) -> Quizzer {
    Quizzer {
      filename
    }
  }

  pub fn parse_lines(self) -> Result<Vec<Problem>, Box<dyn Error>> {
    let content = fs::read_to_string(self.filename)?;
    let mut problems :Vec<Problem> = Vec::new();
    for line in content.lines() {
      let problem: Vec<&str> = line.split(",").collect();
      problems.push(Problem {
        question: String::from(problem[0]),
        answer: String::from(problem[1])
      })
    }
    Ok(problems)
  }
}

#[derive(Debug)]
pub struct Quizzer {
  filename: String
}

#[derive(Debug)]
pub struct Problem {
  pub question: String,
  pub answer: String
}
