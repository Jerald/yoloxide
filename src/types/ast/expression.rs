use std::fmt;

use std::convert::{TryFrom, TryInto};

use yolol_number::YololNumber;

use crate::types::ast::{
    operators::Operator,
    value::Value,

    cylon_ast::{
        Expression as CylonExpr
    }
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

impl TryFrom<CylonExpr> for Expression
{
    type Error = String;
    fn try_from(expr: CylonExpr) -> Result<Self, Self::Error>
    {
        match expr
        {
            CylonExpr::Group { group } => {
                let expr = (*group).try_into()?;
                let value = Value::Group(Box::new(expr));

                Ok(Expression::Value(value))
            },
            CylonExpr::BinaryOp { operator, left, right } => {
                let op = match operator.as_str()
                {
                    "<" => Operator::Lesser,
                    ">" => Operator::Greater,
                    "<=" => Operator::LesserEq,
                    ">=" => Operator::GreaterEq,
                    "==" => Operator::Equal,
                    "!=" => Operator::NotEqual,
                    "and" => Operator::And,
                    "or" => Operator::Or,

                    "+" => Operator::Add,
                    "-" => Operator::Sub,
                    "*" => Operator::Mul,
                    "/" => Operator::Div,
                    "%" => Operator::Mod,
                    "^" => Operator::Pow,

                    bad_op => return Err(format!("[Expression::TryFrom<CylonExpr>] Unable to convert to binary op from string! Found {}", bad_op))
                };

                let left = Box::new((*left).try_into()?);
                let right = Box::new((*right).try_into()?);

                Ok(Expression::BinaryOp(op, left, right))
            },
            CylonExpr::UnaryOp { operator, operand } => {
                let op = match operator.as_str()
                {
                    "-" => Operator::Negate,
                    "++a" => Operator::PreInc,
                    "a++" => Operator::PostInc,
                    "--a" => Operator::PreDec,
                    "a--" => Operator::PostDec,
                    "!" => Operator::Fact,

                    "abs" => Operator::Abs,
                    "sqrt" => Operator::Sqrt,
                    "sin" => Operator::Sin,
                    "cos" => Operator::Cos,
                    "tan" => Operator::Tan,
                    "arcsin" => Operator::Arcsin,
                    "arccos" => Operator::Arccos,
                    "arctan" => Operator::Arctan,
                    "not" => Operator::Not,

                    bad_op => return Err(format!("[Expression::TryFrom<CylonExpr>] Unable to convert to unary op from string! Found {}", bad_op))
                };

                let operand = Box::new((*operand).try_into()?);

                Ok(Expression::UnaryOp(op, operand))
            },
            CylonExpr::Number { num } => {
                let yolol_num = num.parse::<YololNumber>()?;
                let value = Value::NumberVal(yolol_num);

                Ok(Expression::Value(value))
            },
            CylonExpr::String { str } => {
                let value = Value::StringVal(str);
                Ok(Expression::Value(value))
            },
            CylonExpr::Identifier { name } => {
                let value = if name.starts_with(':')
                {
                    Value::DataField(name)
                }
                else
                {
                    Value::LocalVar(name)
                };

                Ok(Expression::Value(value))
            }
        }
    }
}