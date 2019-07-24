use std::fmt;
use std::convert::{TryFrom, TryInto};

use crate::types::ast::{
    expression::Expression,
    operators::Operator,
    value::Value,

    cylon_ast:: {
        Statement as CylonStat,
    }
};

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
            Statement::Comment(string) => format!("//{}", string),
            Statement::If(cond, ref body, Some(ref else_body)) => format!("if {} then {} else {} end", cond, body.iter().fold(String::from(""), |a, e| a + e.to_string().as_str()), else_body.iter().fold(String::from(""), |a, e| a + e.to_string().as_str())),
            Statement::If(cond, body, None) => format!("if {} then {} end", cond, body.iter().fold(String::from(""), |a, e| a + e.to_string().as_str())),

            Statement::Goto(expr) => format!("goto {}", expr.as_ref()),
            Statement::Assignment(ident, op, value) => format!("{} {} {}", ident, op, value),

            Statement::Expression(expr) => format!("{}", expr.as_ref()),
        };

        write!(f, "{}", write_value)
    }
}

impl TryFrom<CylonStat> for Statement
{
    type Error = String;
    fn try_from(stat: CylonStat) -> Result<Self, Self::Error>
    {
        match stat
        {
            CylonStat::Goto { expression } => {
                let expr = Box::new(expression.try_into()?);
                Ok(Statement::Goto(expr))
            },
            CylonStat::If { condition, body, else_body } => {
                let cond = Box::new(condition.try_into()?);

                let mut ast_body = vec![];
                for stat in body
                {
                    ast_body.push(stat.try_into()?)
                }

                let ast_else_body = if else_body.is_empty()
                {
                    None
                }
                else
                {
                    let mut ast_else_body = vec![];
                    for stat in else_body
                    {
                        ast_else_body.push(stat.try_into()?)
                    }

                    Some(ast_else_body)
                };

                Ok(Statement::If(cond, ast_body, ast_else_body))
            },
            CylonStat::Assignment { identifier, operator, value } => {
                let ident = if identifier.starts_with(':')
                {
                    Value::DataField(identifier)
                }
                else
                {
                    Value::LocalVar(identifier)
                };

                let op = match operator.as_str()
                {
                    "=" => Operator::Assign,
                    "+=" => Operator::AddAssign,
                    "-=" => Operator::SubAssign,
                    "*=" => Operator::MulAssign,
                    "/=" => Operator::DivAssign,
                    "%=" => Operator::ModAssign,

                    bad_op => return Err(format!("[Statement::TryFrom<CylonStat>] Unable to convert to assignment op from string! Found{}", bad_op))
                };

                let value = Box::new(value.try_into()?);

                Ok(Statement::Assignment(ident, op, value))
            },
            CylonStat::Expression { expression } => {
                let expr = Box::new(expression.try_into()?);
                Ok(Statement::Expression(expr))
            }
        }
    }
}