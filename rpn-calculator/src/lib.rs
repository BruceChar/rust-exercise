#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}
use CalculatorInput::*;

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = vec![];
    let mut inputs = inputs.iter();
    while let Some(i) = inputs.next() {
        match i {
            Value(v) => stack.push(*v),
            others => {
                if stack.len() < 2 {
                    return None;
                }    
                let second = stack.pop().unwrap();
                let first = stack.pop().unwrap(); 
                let val = match others {
                    Add => first + second,
                    Subtract => first - second,
                    Multiply => first * second,
                    Divide => first / second,
                    _ => return None,
                };
                stack.push(val);
            }
        }
    }
    stack.pop()
}
