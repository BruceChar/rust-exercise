use std::collections::HashMap;

use once_cell::sync::Lazy;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

type Fop = fn(&mut Forth) -> Result;

const LOOKUP: [&str; 8] = ["+", "-", "*", "/", "dup", "swap", "over", "drop"];
static OPMAP: Lazy<HashMap<&str, Fop>> = Lazy::new(|| {
    let mut map: HashMap<&str, fn(&mut Forth) -> Result> = HashMap::new();
    LOOKUP.iter().for_each(|&op| {
        let call = match op {
            "+" => Forth::add,
            "-" => Forth::sub,
            "*" => Forth::mul,
            "/" => Forth::div,
            "dup" => Forth::dup,
            "swap" => Forth::swap,
            "over" => Forth::over,
            "drop" => Forth::drop,
            _ => panic!("invalid operation code"),
        };
        map.insert(op, call);
    });
    map
});

#[derive(Debug, Clone)]
enum Op {
    Call(Fop),
    Num(i32)
}

pub struct Forth {
    pub(crate) stack: Vec<Value>,
    ops: HashMap<String, Op>,
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
            ops: HashMap::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    fn pop(&mut self) -> std::result::Result<Value, Error> {
        self.stack.pop().ok_or(Error::StackUnderflow)
    }

    fn push(&mut self, val: Value) -> Result {
        self.stack.push(val);
        Ok(())
    }

    fn len(&self) -> usize {
        self.stack.len()
    }

    fn add(&mut self) -> Result {
        self.arth(|a, b| a + b)
    }
    fn sub(&mut self) -> Result {
        self.arth(|a, b| a - b)
    }
    fn mul(&mut self) -> Result {
        self.arth(|a, b| a * b)
    }
    fn div(&mut self) -> Result {
        self.arth(|a, b| a / b)
    }

    fn drop(&mut self) -> Result {
        self.stack.pop().ok_or(Error::StackUnderflow)?;
        Ok(())
    }

    fn dup(&mut self) -> Result {
        let v = self.stack.last().ok_or(Error::StackUnderflow)?;
        self.push(*v)
    }

    fn over(&mut self) -> Result {
        if self.len() < 2 {
            return Err(Error::StackUnderflow);
        }
        let v = self
            .stack
            .get(self.stack.len() - 2)
            .ok_or(Error::StackUnderflow)?;
        self.push(*v)
    }

    fn swap(&mut self) -> Result {
        let len = self.stack.len();
        if len < 2 {
            return Err(Error::StackUnderflow);
        }
        self.stack.swap(len - 2, len - 1);
        Ok(())
    }

    fn arth(&mut self, f: fn(i32, i32) -> i32) -> Result {
        let (v1, v2) = (self.pop()?, self.pop()?);
        if v1 == 0 && f(2, 2) == 1 {
            return Err(Error::DivisionByZero);
        }
        self.push(f(v2, v1))
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let mut iter = input.split_whitespace();
        while let Some(s) = iter.next() {
            let s = s.to_ascii_lowercase();
            let s = s.as_str();
            if let Some(f) = self.ops.get(s) {
                match f {
                    Op::Call(f) =>  f(self)?,
                    Op::Num(i) => self.push(*i)?,
                }
                continue;
            }
            // ops can be rewrite
            if let Some(f) = OPMAP.get(s) {
                f(self)?;
                continue;
            }
            if let Ok(num) = str::parse::<i32>(s) {
                self.push(num)?;
                continue;
            }
            if s == ":" {
                println!("{:?}", iter);
                let w = iter.next().ok_or(Error::StackUnderflow)?;
                if str::parse::<i32>(w).is_ok() {
                    return Err(Error::InvalidWord);
                }
                let op = iter.next().ok_or(Error::StackUnderflow)?;
                let c = iter.next().ok_or(Error::StackUnderflow)?;
                if c != ";" {
                    return Err(Error::InvalidWord);
                }
                
                let f: Op = match op {
                    "dup" => Op::Call(Self::dup),
                    "swap" => Op::Call(Self::swap),
                    "drop" => Op::Call(Self::drop),
                    "over" => Op::Call(Self::over),
                    _ => {
                        if let Ok(i) = str::parse::<i32>(op) {
                            Op::Num(i)
                        } else if let Some(f) = self.ops.get(op) {
                            f.clone()
                        } else {
                            return Err(Error::UnknownWord);
                        }
                    },
                };
                self.ops.insert(w.to_owned(), f);
            } else {
                return Err(Error::UnknownWord);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval() {
        let mut vm = Forth::new();
        vm.eval("1 2 +").unwrap();
        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack[0], 3);
    }

    #[test]
    fn test_define() {
        let mut forth = Forth::new();
        forth.eval(": foo drop ;").unwrap();
        forth.eval("1 foo").unwrap();
        assert_eq!(forth.stack.len(), 0);
    }
}
