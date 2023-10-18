use std::collections::HashMap;

use once_cell::sync::Lazy;

pub type Value = i32;
pub type Result<T> = std::result::Result<T, Error>;
type UnitResult = Result<()>;

type Fop = fn(&mut Forth) -> UnitResult;

const LOOKUP: [&str; 8] = ["+", "-", "*", "/", "dup", "swap", "over", "drop"];
static OPMAP: Lazy<HashMap<&str, Fop>> = Lazy::new(|| {
    let mut map: HashMap<&str, Fop> = HashMap::with_capacity(LOOKUP.len());
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
    Num(i32),
    Ops(Vec<Op>),
}

impl Op {
    fn call(&mut self, forth: &mut Forth) -> UnitResult {
        match self {
            Op::Call(call) => call(forth),
            Op::Num(num) => forth.push(*num),
            Op::Ops(ops) => {
                for op in ops {
                    op.call(forth)?;
                }
                Ok(())
            }
        }
    }
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

    fn pop(&mut self) -> Result<Value> {
        self.stack.pop().ok_or(Error::StackUnderflow)
    }

    fn push(&mut self, val: Value) -> UnitResult {
        self.stack.push(val);
        Ok(())
    }

    fn len(&self) -> usize {
        self.stack.len()
    }

    fn add(&mut self) -> UnitResult {
        self.arth(|a, b| a + b)
    }
    fn sub(&mut self) -> UnitResult {
        self.arth(|a, b| a - b)
    }
    fn mul(&mut self) -> UnitResult {
        self.arth(|a, b| a * b)
    }
    fn div(&mut self) -> UnitResult {
        self.arth(|a, b| a / b)
    }

    fn drop(&mut self) -> UnitResult {
        self.stack.pop().ok_or(Error::StackUnderflow)?;
        Ok(())
    }

    fn dup(&mut self) -> UnitResult {
        let v = self.stack.last().ok_or(Error::StackUnderflow)?;
        self.push(*v)
    }

    fn over(&mut self) -> UnitResult {
        if self.len() < 2 {
            return Err(Error::StackUnderflow);
        }
        let v = self
            .stack
            .get(self.stack.len() - 2)
            .ok_or(Error::StackUnderflow)?;
        self.push(*v)
    }

    fn swap(&mut self) -> UnitResult {
        let len = self.stack.len();
        if len < 2 {
            return Err(Error::StackUnderflow);
        }
        self.stack.swap(len - 2, len - 1);
        Ok(())
    }

    fn arth(&mut self, f: fn(i32, i32) -> i32) -> UnitResult {
        let (v1, v2) = (self.pop()?, self.pop()?);
        if v1 == 0 && f(2, 2) == 1 {
            return Err(Error::DivisionByZero);
        }
        self.push(f(v2, v1))
    }

    pub fn eval(&mut self, input: &str) -> UnitResult {
        let mut iter = input.split_whitespace();
        while let Some(s) = iter.next() {
            let s = s.to_ascii_lowercase();
            let s = s.as_str();
            if let Some(f) = self.ops.get(s) {
                match f {
                    Op::Call(f) => f(self)?,
                    Op::Num(i) => self.push(*i)?,
                    Op::Ops(ops) => {
                        for op in ops {
                            f.call(*self)?;
                        }
                    },
                }
                continue;
            }
            // ops can be rewrite, must be after self.ops
            if let Some(f) = OPMAP.get(s) {
                f(self)?;
                continue;
            }
            if let Ok(num) = str::parse::<i32>(s) {
                self.push(num)?;
                continue;
            }
            if s == ":" {
                let (w, op) = self.define(&mut iter, &self.ops)?;
                self.ops.insert(w, op);
            } else {
                return Err(Error::UnknownWord);
            }
        }
        Ok(())
    }

    fn define<'a>(&self,
        iter: &mut impl Iterator<Item = &'a str>,
        ops: &HashMap<String, Op>,
    ) -> Result<(String, Op)> {
        let w = iter.next().ok_or(Error::StackUnderflow)?;
        if str::parse::<i32>(w).is_ok() {
            return Err(Error::InvalidWord);
        }
        let mut ops = vec![];
        while let Some(s) = iter.next() {
            match s {
                // Assume the ";" is the last
                ";" => return Ok((w.to_owned(), Op::Ops(ops))),
                _ => {
                    if let Some(f) = OPMAP.get(s) {
                        ops.push(Op::Call(*f));
                        continue;
                    }
                    if let Ok(num) = str::parse::<i32>(s) {
                        ops.push(Op::Num(num));
                        continue;
                    } 
                    if let Some(f) = self.ops.get(s) {
                        ops.push(f.clone());
                    } else {
                        return Err(Error::UnknownWord);
                    }
                }
            }
        }
        return Err(Error::StackUnderflow);
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
