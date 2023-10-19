use std::collections::HashMap;
mod lib1;

pub type Value = i32;
pub type Result<T> = std::result::Result<T, Error>;
type UnitResult = Result<()>;

type Fop = fn(&mut Forth) -> UnitResult;

const LOOKUP: [&str; 8] = ["+", "-", "*", "/", "dup", "swap", "over", "drop"];
static OPMAP: [Fop; 8] = [
    Forth::add,
    Forth::sub,
    Forth::mul,
    Forth::div,
    Forth::dup,
    Forth::swap,
    Forth::over,
    Forth::drop,
];

#[derive(Debug, Clone)]
enum Op {
    Call(Fop),
    Num(i32),
    Ops(Vec<Op>),
}

impl Op {
    fn call(&self, forth: &mut Forth) -> UnitResult {
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
    ops: Vec<(String, Op)>,
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
            ops: Vec::new(),
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
            if let Some((_, op)) = self.ops.iter().find(|(k, _)| *k == s) {
                op.clone().call(self)?;
                continue;
            }
            // if let Some(f) = self.ops.get(s) {
            //     match f.clone() {
            //         Op::Call(c) => c(self)?,
            //         Op::Num(i) => self.push(i)?,
            //         Op::Ops(ops) => {
            //             for op in ops {
            //                 op.call(self)?;
            //             }
            //         }
            //     }
            //     continue;
            // }
            if let Some(ind) = LOOKUP.iter().position(|op| *op == s) {
                OPMAP[ind](self)?;
                continue;
            }
            if let Ok(num) = str::parse::<i32>(s) {
                self.push(num)?;
                continue;
            }
            if s == ":" {
                let (w, op) = self.define(&mut iter)?;
                if let Some(ind) = self.ops.iter_mut().position(|(k, _)| *k == w) {
                    self.ops[ind] = (w, op);
                } else {
                    self.ops.push((w, op));
                }
            } else {
                return Err(Error::UnknownWord);
            }
        }
        Ok(())
    }

    fn define<'a>(&self, iter: &mut impl Iterator<Item = &'a str>) -> Result<(String, Op)> {
        let w = iter.next().ok_or(Error::InvalidWord)?;
        if str::parse::<i32>(w).is_ok() {
            return Err(Error::InvalidWord);
        }
        let mut ops = vec![];
        while let Some(s) = iter.next() {
            let s = s.to_ascii_lowercase();
            let s = s.as_str();
            match s {
                // Assume the ";" is the last
                ";" => return Ok((w.to_ascii_lowercase(), Op::Ops(ops))),
                _ => {
                    if let Some(ind) = LOOKUP.iter().position(|&op| op == s) {
                        ops.push(Op::Call(OPMAP[ind]));
                        continue;
                    }
                    if let Ok(num) = str::parse::<i32>(s) {
                        ops.push(Op::Num(num));
                        continue;
                    }
                    if let Some((_, op)) = self.ops.iter().find(|(k,_)| *k == s) {
                        ops.push(op.clone());
                    } else {
                        return Err(Error::InvalidWord);
                    }
                }
            }
        }
        return Err(Error::InvalidWord);
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
        forth.eval("1 1 : baz foo ;").unwrap();
        // assert_eq!(forth.stack.len(), 1);
    }
}
