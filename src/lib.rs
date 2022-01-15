#![deny(clippy::all)]

extern crate execute;

use std::process::Stdio;
use std::io::{BufReader, BufRead};
use execute::command;

#[macro_use]
extern crate napi_derive;
extern crate napi;

use napi::bindgen_prelude::*;

mod common;

use common::{Expect};

enum Step {
  Expect(Expect),
  // ExpectError(ExpectError)
}

#[napi]
#[derive(Clone, Copy)]
pub struct ClixResult {
  ok: bool,
  // output: Vec<String>, // TODO: use ErrOutput and Output type
}

#[napi]
impl ClixResult {
  pub fn new(ok: bool) -> Self {
    Self { ok }
  }
}

#[napi]
pub struct Clix {
  command: String,
  steps: Vec<Step>,
  result: ClixResult, // TODO: should exist in the run only
}

#[napi]
impl Clix {
  #[napi(constructor)]
  pub fn new(cmd_str: String) -> Self {
    let res = ClixResult::new(true);
    Clix { command: cmd_str, steps: Vec::new(), result: res }
  }

  #[napi]
  pub fn expect(&mut self, line: String) -> &mut Clix {
    self.steps.push(Step::Expect(Expect::new(line)));
    self
  }

  #[napi]
  pub fn run(&mut self) -> ClixResult {
    let mut child = command(&self.command)
      .stdout(Stdio::piped())
      .spawn()
      .unwrap();

    let stdout = child.stdout.take().unwrap();
    let stdout_reader = BufReader::new(stdout);

    let mut stdout_lines = std::collections::LinkedList::new();
    stdout_reader
      .lines()
      .for_each(|line| {
        stdout_lines.push_back(line.ok().unwrap());
      });

    let mut steps_it = self.steps.iter();
    while let Some(step) = steps_it.next() {
      match step {
        Step::Expect(e) => {
          match stdout_lines.pop_front() {
            Some(x) => {
              if e.line != x {
                self.result.ok = false; 
                println!("ERROR: {} != {}", e.line, x);
              }
            },
            None => {
              self.result.ok = false; 
              println!("ERROR: missing output for line {}", e.line);
              break;
            }
          }
        }
      }
    }

    child.wait().unwrap();

    self.result
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

// TODO: check error string
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
    Step::Expect(step) => assert_eq!(step.line, "hey")
  }  
}

#[test]
fn it_should_expose_a_run_method() {
  let mut scenario = clix(String::from("echo lol"))
    .unwrap();
    
  scenario.expect(String::from("lol"));
  
  let res = &scenario.run();

  assert!(res.ok);
}

#[test]
fn it_should_return_fasly_ok_if_lines_not_match() {
  let mut scenario = clix(String::from("echo lol"))
    .unwrap();
    
  scenario.expect(String::from("zoo"));
  
  let res = &scenario.run();

  assert_eq!(res.ok, false);
}