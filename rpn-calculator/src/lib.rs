#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();

    for i in inputs {
        match i {
            CalculatorInput::Add => {
                if stack.len() < 2 {
                    return None;
                } else {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(a+b);
                }
            },
            CalculatorInput::Subtract => {
                if stack.len() < 2 {
                    return None;
                } else {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(b-a);
                }
            },
            CalculatorInput::Multiply => {
                if stack.len() < 2 {
                    return None;
                } else {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(a*b);
                }
            },
            CalculatorInput::Divide => {
                if stack.len() < 2 {
                    return None;
                } else {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(b/a);
                }
            },
            CalculatorInput::Value(x) => {
                stack.push(x.clone());
            },
        }
    }
    if stack.len() == 1 {
        return stack.pop();
    } else {
        return None;
    }
}
