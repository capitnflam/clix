pub type Line = String;
//pub type Timeout = Option<u32>;

#[derive(Debug)]
pub struct Expect {
  pub line: Line,
  // timeout: Timeout,
}

impl Expect {
  pub fn new(line: String) -> Self {
    Self { line }
  }
}

#[derive(Debug)]
pub struct Input {
  pub line: Line,
}

impl Input {
  pub fn new(line: String) -> Self {
    Self { line }
  }
}

// struct ExpectError {
//   line: Line,
//   timeout: Timeout,
//   code: Option<u8>,
// }