use std::fmt;

use super::Value;
use super::Expression;

use super::Operator;

#[derive(Debug, PartialEq, Clone)]
pub enum Statement
{
    Comment(String),
    If(Box<Expression>, Vec<Statement>, Option<Vec<Statement>>),
    Goto(Box<Expression>),
    Assignment(Value, Operator, Box<Expression>),
    Expression(Box<Expression>)
}

impl fmt::Display for Statement
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        let write_value: String = match self
        {
            Statement::Comment(string) => format!("//{}\n", string),
            Statement::If(cond, ref body, Some(ref else_body)) => format!("if {} then {} else {} end", cond, body.iter().fold(String::from(""), |a, e| a + e.to_string().as_str()), else_body.iter().fold(String::from(""), |a, e| a + e.to_string().as_str())),
            Statement::If(cond, body, None) => format!("if {} then {} end", cond, body.iter().fold(String::from(""), |a, e| a + e.to_string().as_str())),

            Statement::Goto(expr) => format!("goto {}", expr.as_ref()),
            Statement::Assignment(ident, op, value) => format!("{} {} {}", ident, op, value),

            Statement::Expression(expr) => format!("{}", expr.as_ref()),
        };

        write!(f, "{}", write_value)
    }
}