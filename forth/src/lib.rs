pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    pub(crate) stack: Vec<Value>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Self {
            stack: Vec::<Value>::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }
    fn pop_two(&mut self) -> std::result::Result<(Value, Value), Error> {
        let v1 = self.stack.pop().ok_or(Error::StackUnderflow)?;
        let v2 = self.stack.pop().ok_or(Error::StackUnderflow)?;
        Ok((v1, v2))
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let words = std::collections::HashMap::<&str, &str>::new();
        for s in input.split_whitespace() {
            match s.to_ascii_lowercase().as_str() {
                "+" => {
                    let (v1, v2) = self.pop_two()?;
                    self.stack.push(v1 + v2);
                }
                "-" => {
                    let (v1, v2) = self.pop_two()?;
                    self.stack.push(v2 - v1);
                }
                "*" => {
                    let (v1, v2) = self.pop_two()?;
                    self.stack.push(v2 * v1);
                }
                "/" => {
                    let (v1, v2) = self.pop_two()?;
                    if v1 == 0 {
                        return Err(Error::DivisionByZero);
                    }
                    self.stack.push(v2 / v1);
                }
                "dup" => {
                    let v = self.stack.pop().ok_or(Error::StackUnderflow)?;
                    self.stack.push(v);
                    self.stack.push(v);
                }
                "drop" => {
                    self.stack.pop().ok_or(Error::StackUnderflow)?;
                }
                "swap" => {
                    let (v1, v2) = self.pop_two()?;
                    self.stack.push(v1);
                    self.stack.push(v2);
                }
                "over" => {
                    let (v1, v2) = self.pop_two()?;
                    self.stack.push(v2);
                    self.stack.push(v1);
                    self.stack.push(v2);
                }

                _ => match s.parse::<i32>() {
                    Ok(v) => {
                        self.stack.push(v);
                    }
                    Err(_) => return Err(Error::UnknownWord),
                },
            }
        }
        Ok(())
    }
}
