#![deny(clippy::all)]

use core::panic;
use std::process::Stdio;
use std::io::{BufReader, BufRead, Write, BufWriter};
use execute::command;

#[macro_use]
extern crate napi_derive;
extern crate napi;
use napi::bindgen_prelude::*;

extern crate execute;

mod common;
use common::{Expect, Input};

#[derive(Debug)]
enum Step {
  Expect(Expect),
  Input(Input),
  // ExpectError(ExpectError)
}

#[napi]
#[derive(Clone, Copy, Debug)]
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
#[derive(Debug)]
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

  pub fn fun_be_because_no_run(&mut self) -> ClixResult {
    let mut child = command(&self.command)
      .stdout(Stdio::piped())
      .stdin(Stdio::piped())
      .spawn()
      .unwrap();

    let stdout = child.stdout.take().unwrap();
    let stdout_reader = BufReader::new(stdout);

    let mut lines = Vec::new();
    stdout_reader
      .lines()
      .filter_map(|line| { line.ok() })
      .for_each(|line| {
        lines.push(line);
      });


    if lines[0] != String::from("What is your name?") {
      panic!("boom");
    } else {
      panic!("pas boom");
    }

    // self.result
  }

  #[napi]
  pub fn run(&mut self) -> ClixResult {
    println!("Start");
    let mut child;
    let maybe_child = command(&self.command)
      .stdout(Stdio::piped())
      .stdin(Stdio::piped())
      .spawn();

    match maybe_child {
      Ok(c) => {
        child = c;
      },
      Err(err) => {
        println!("Clix error: run command {} failed", &self.command);
        println!("{}", err.to_string());
        self.result.ok = false;
        return self.result;
      }
    }
      
    let stdout = child.stdout.take().unwrap();
    let stdout_reader = BufReader::new(stdout);
    
    let stdin = child.stdin.take().unwrap();
    let mut stdin_writer = BufWriter::new(stdin);

    // TODO: should be in a function 
    let mut stdout_lines = std::collections::LinkedList::new();
    stdout_reader
      .lines()
      .for_each(|line| {
        stdout_lines.push_back(line.ok().unwrap());
      });

    child.try_wait().unwrap();

    let mut steps_it = self.steps.iter();
    while let Some(step) = steps_it.next() {
      println!("{:?}", step);
      match step {
        Step::Expect(e) => {
          match stdout_lines.pop_front() {
            Some(x) => {
              if e.line != x {
                self.result.ok = false; 
                println!("ERROR: {} != {}", e.line, x);
              } else {
                println!("PASS: {} == {}", e.line, x);
              }
            },
            None => {
              self.result.ok = false; 
              println!("ERROR: missing output for line {}", e.line);
              break;
            }
          }
        },
        Step::Input(input) => {
          stdin_writer.write(input.line.as_bytes()).unwrap();
          println!("INPUT: {}", input.line);
        }
      }
    }

    child.wait().unwrap();

    self.result
  }

  pub fn input(&mut self, line: String) -> &mut Clix {
    let input = Input::new(line);
    self.steps.push(Step::Input(input));

    self
  }
}

#[napi]
fn clix(cmd_str: String) -> Result<Clix> {
  if cmd_str.len() < 1 {
    return Err(
      Error::new(
        Status::InvalidArg,
        String::from("command argument is required")
      )
    );
  }

  Ok(Clix::new(cmd_str))
}

// #[test]
// fn given_an_empty_string_it_should_return_an_error() {
//     let clix_instance = clix(String::from("")).unwrap_err();
//     assert_eq!(clix_instance.reason, String::from("command argument is required"));
// }

// #[test]
// fn given_command_it_should_return_clix_struct() {
//   let valid_command = String::from("ls -la");

//   let clix_instance = clix(valid_command.clone()).unwrap();

//   assert_eq!(clix_instance.command, valid_command);
// }

// #[test]
// fn it_should_expose_expect_command() {
//   let valid_command = String::from("ls -la");
//   let mut scenario = clix(valid_command.clone())
//     .unwrap();

//   let stp = &scenario.expect(String::from("hey")).steps[0];

//   match stp {
//     Step::Expect(step) => assert_eq!(step.line, "hey"),
//     _ => {}
//   }  
// }

// #[test]
// fn it_should_expose_a_run_method() {
//   let mut scenario = clix(String::from("echo lol"))
//     .unwrap();
//   scenario.expect(String::from("lol"));
  
//   let res = &scenario.run();

//   assert!(res.ok);
// }

// #[test]
// fn it_should_return_fasly_ok_if_lines_not_match() {
//   let mut scenario = clix(String::from("echo lol"))
//     .unwrap();
//   scenario.expect(String::from("zoo"));
  
//   let res = &scenario.run();

//   assert_eq!(res.ok, false);
// }

#[test]
fn it_should_expose_an_input_method() {
  let mut scenario = clix(String::from("bash ./src/fixture/test.bash")).unwrap();
  scenario
    .expect(String::from("What is your name?"));
    // .input(String::from("tony"))
    // .expect(String::from("Hello tony!"));

  let res = &scenario.fun_be_because_no_run();

  assert_eq!(res.ok, true);
}

// IDEA:
// - SPAWN COMMAND
// - COLLECT OUTPUT
// - CHECK IT
// - APPLY INPUT
// - IF STEPS TO CHECK
  // - COLLECT OUTPUT
  // - CHECK IT
  // - APPLY INPUT
// - ELSE -> CLOSE