use crate::CalculatorInput::*;

#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Substract,
    Multiply,
    Divide,
    Value(i32),
}

fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    if inputs.len() == 1 {
        return None;
    }
    let mut stack: Vec<i32> = Vec::new();
    for item in inputs {
        match item {
            Value(v) => stack.push(*v),
            _ => {
                if stack.len() < 2 {
                    return None;
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();

                match item {
                    Add => stack.push(a + b),
                    Substract => stack.push(a - b),
                    Divide => stack.push(a / b),
                    Multiply => stack.push(a * b),
                    _ => (),
                }
            }
        };
    }
    if stack.len() != 1 {
        return None;
    }
    stack.pop()
}

fn main() {
    let cal_input = [
        Value(4),
        Value(8),
        Add,
        Value(7),
        Value(5),
        Substract,
        Divide,
    ];
    let result = evaluate(&cal_input);
    println!("{result:?}");
}
