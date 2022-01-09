#![deny(clippy::all)]

extern crate execute;

use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead};
use execute::{Execute, command};

#[macro_use]
extern crate napi_derive;
extern crate napi;

use napi::bindgen_prelude::*;

type Line = String;
type Timeout = Option<u32>;

struct Expect {
  line: Line,
  timeout: Timeout,
}

impl Expect {
  pub fn new(line: String) -> Self {
    Self { line, timeout: None }
  }
}

struct ExpectError {
  line: Line,
  timeout: Timeout,
  code: Option<u8>,
}

enum Step {
  Expect(Expect),
  ExpectError(ExpectError)
}

#[napi]
pub struct Clix {
  command: String,
  steps: Vec<Step>,
}

#[napi]
impl Clix {
  #[napi(constructor)]
  pub fn new(cmd_str: String) -> Self {
    Clix { command: cmd_str, steps: Vec::new() }
  }

  #[napi]
  pub fn expect(&mut self, line: String) -> &Self {
    self.steps.push(Step::Expect(Expect::new(line)));
    self
  }
}

#[napi]
fn clix(cmd_str: String) -> Result<Clix> {
  if cmd_str.len() < 1 {
    return Err(
      Error::new(
        Status::InvalidArg,
        "command argument is required".to_owned()
      )
    );
  }

  Ok(Clix::new(cmd_str))
}

#[test]
fn given_an_empty_string_it_should_return_an_error() {
    let clix_instance = clix(String::from(""));
    assert!(clix_instance.is_err());
}

#[test]
fn given_command_it_should_return_clix_struct() {
  let valid_command = String::from("ls -la");

  let clix_instance = clix(valid_command.clone()).unwrap();

  assert_eq!(clix_instance.command, valid_command);
}

#[test]
fn it_should_expose_expect_command() {
  let valid_command = String::from("ls -la");
  let mut scenario = clix(valid_command.clone())
    .unwrap();

  let stp = &scenario.expect(String::from("hey")).steps[0];

  match stp {
    Step::Expect(step) => assert_eq!(step.line, "hey"),
    _ => assert!(false),
  }

  
}


// DRAFT for spawn command
// fn clix(cmd_str: String) -> String {
//   let stdout = command(cmd_str)
//     .stdout(Stdio::piped())
//     .spawn()
//     .expect("Spawn command failed")
//     .stdout
//     .expect("Wrap stdout failed");

//   let reader = BufReader::new(stdout);

//   reader
//     .lines()
//     .filter_map(|line| line.ok())
//     .for_each(|line| println!("{}", line));

//   String::from("Go!")
// }
