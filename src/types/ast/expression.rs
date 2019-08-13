use std::fmt;

use crate::types::ast::{
    operators::Operator,
    value::Value,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Expression
{
    BinaryOp(Operator, Box<Expression>, Box<Expression>),
    UnaryOp(Operator, Box<Expression>),
    Value(Value)
}

impl fmt::Display for Expression
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self
        {
            Expression::BinaryOp(op, left, right) => write!(f, "{} {} {}", left, op, right),
            
            Expression::UnaryOp(op @ Operator::Negate, value) |
            Expression::UnaryOp(op @ Operator::PreInc, value) |
            Expression::UnaryOp(op @ Operator::PreDec, value)  => write!(f, "{}{}", op, value),

            Expression::UnaryOp(op @ Operator::PostInc, value) |
            Expression::UnaryOp(op @ Operator::PostDec, value) |
            Expression::UnaryOp(op @ Operator::Fact, value) => write!(f, "{}{}", value, op),

            Expression::UnaryOp(op @ Operator::Abs, value) |
            Expression::UnaryOp(op @ Operator::Sqrt, value) |
            Expression::UnaryOp(op @ Operator::Sin, value) |
            Expression::UnaryOp(op @ Operator::Cos, value) |
            Expression::UnaryOp(op @ Operator::Tan, value) |
            Expression::UnaryOp(op @ Operator::Arcsin, value) |
            Expression::UnaryOp(op @ Operator::Arccos, value) |
            Expression::UnaryOp(op @ Operator::Arctan, value) |
            Expression::UnaryOp(op @ Operator::Not, value) => write!(f, "{} {}", op, value),

            Expression::Value(value) => write!(f, "{}", value),

            _ => panic!("Attempting to display bad expression!")
        }
    }
}

