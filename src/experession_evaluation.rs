use std::ops::{Add, DivAssign, Mul, Sub};

pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide
}

enum Expression {
    Value(i64),
    Operation(Box<Expression>, Operation, Box<Expression>)
}

fn evaluate(e: Expression) -> Result<i64, String> {
    match e {
        Expression::Value(v) => Ok(v),
        Expression::Operation(left, op, right) => {
            let left = evaluate(*left)?;
            let right = evaluate(*right)?;

            match op {
                Operation::Add => Ok(left + right),
                Operation::Subtract => Ok(left - right),
                Operation::Multiply => Ok(left * right),
                Operation::Divide => Ok(left / right)
            }
        }
    }
}

#[test]
#[ignore]
fn test_value() {
    assert_eq!(evaluate(Expression::Value(19)), Ok(19));
}

#[test]
#[ignore]
fn test_sum() {
    assert_eq!(
        evaluate(Expression::Operation(
            Box::new(Expression::Value(10)),
            Operation::Add,
            Box::new(Expression::Value(20))
        )),
        Ok((30))
    );
}