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
    Ops(Vec<Vec<Op>>),
    Key((String, usize)),
}

impl Op {
    fn call(&self, forth: &mut Forth, ind: usize) -> UnitResult {
        match self {
            Op::Call(call) => call(forth),
            Op::Num(num) => forth.push(*num),
            Op::Ops(ops) => {
                for op in &ops[ind] {
                    op.call(forth, ind)?;
                }
                Ok(())
            }
            Op::Key((k, i)) => {
                if let Some(op) = forth.ops.get(k).cloned() {
                    op.call(forth, *i)?;
                    Ok(())
                } else {
                    Err(Error::InvalidWord)
                }
            }
        }
    }
}

#[derive(Debug, Default)]
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
        Self::default()
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
            if let Some(op) = self.ops.get(s).cloned() {
                match &op {
                    Op::Ops(ops) => {
                        op.call(self, ops.len() - 1)?;
                    }
                    _ => op.call(self, 0)?,
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
                let (w, op) = self.define(&mut iter)?;
                self.ops.insert(w, op);
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

        for s in iter.by_ref() {
            let s = s.to_ascii_lowercase();
            let s = s.as_str();
            let mut vd = vec![];
            match s {
                // Assume the ";" is the last
                ";" => {
                    if let Some(Op::Ops(v)) = self.ops.get(w) {
                        vd.extend(v.clone());
                    }
                    vd.push(ops);
                    return Ok((w.to_ascii_lowercase(), Op::Ops(vd)));
                }
                _ => {
                    if let Some(f) = OPMAP.get(s) {
                        ops.push(Op::Call(*f));
                        continue;
                    }
                    if let Ok(num) = str::parse::<i32>(s) {
                        ops.push(Op::Num(num));
                        continue;
                    }
                    if let Some(Op::Ops(v)) = self.ops.get(s) {
                        if w == s {
                            ops.extend(v.last().unwrap().clone());

                            // ops.push(op.clone());
                        } else {
                            ops.push(Op::Key((s.to_string(), v.len() - 1)));
                        }
                    } else {
                        return Err(Error::InvalidWord);
                    }
                }
            }
        }
        Err(Error::InvalidWord)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval() {
        let mut f = Forth::new();
        f.eval("1 2 + dup").unwrap();
        println!("{f:?}");

        assert_eq!(f.stack.len(), 2);
        assert_eq!(f.stack[0], 3);
    }

    #[test]
    fn test_define() {
        let mut forth = Forth::new();
        forth.eval(": foo 5 ;").unwrap();
        forth.eval(": baz foo ;").unwrap();
        forth.eval(": foo 6 ;").unwrap();
        forth.eval(": fuz foo ;").unwrap();
        forth.eval(": foo 7 ;").unwrap();
        forth.eval("fuz").unwrap();
        forth.eval("baz").unwrap();
        forth.eval("foo").unwrap();

        println!("{forth:?}");
    }

    #[test]
    fn can_define_word_that_uses_word_with_the_same_name() {
        let mut f = Forth::new();
        assert!(f.eval(": foo 10 ;").is_ok());
        println!("{f:?}");

        assert!(f.eval(": foo foo 1 + ;").is_ok());
        println!("{f:?}");

        assert!(f.eval("foo").is_ok());
        println!("{f:?}");

        // assert_eq!(vec![11], f.stack());
    }
}
