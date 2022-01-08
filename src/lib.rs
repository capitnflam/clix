#![deny(clippy::all)]

extern crate execute;

use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead};
use execute::{Execute, command};

#[macro_use]
extern crate napi_derive;
extern crate napi;

use napi::bindgen_prelude::*;

#[napi]
pub struct Clix {
  command: String,
  steps: Vec<String>,
}

#[napi]
impl Clix {
  #[napi(constructor)]
  pub fn new(cmd_str: String) -> Self {
    Clix { command: cmd_str, steps: Vec::new() }
  }

  #[napi]
  pub fn expect(&mut self, line: String) -> &Self {
    self.steps.push(line);
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
    let res = clix(String::from(""));
    assert!(res.is_err());
}

#[test]
fn given_command_it_should_return_clix_struct() {
  let valid_command = String::from("ls -la");
  let res = clix(valid_command.clone()).unwrap();

  assert_eq!(res.command, valid_command);
}

#[test]
fn it_should_expose_expect_command() {
  let valid_command = String::from("ls -la");
  let mut res = clix(valid_command.clone()).unwrap();

  let slf = res.expect(String::from("hey"));

  assert_eq!(slf.steps[0], "hey");
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
