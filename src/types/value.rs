use std::fmt;

use super::Token;
use super::YololNumber;
use super::Expression;

mod literal_value;
pub use literal_value::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Value
{
    Group(Box<Expression>),
    LocalVar(String),
    DataField(String),
    NumberVal(YololNumber),
    StringVal(String)
}

impl fmt::Display for Value
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self
        {
            Value::Group(expr) => write!(f, "({})", *expr),
            Value::LocalVar(string) => write!(f, "{}", string),
            Value::DataField(string) => write!(f, "{}", string),
            Value::NumberVal(num) => write!(f, "{}", num),
            Value::StringVal(string) => write!(f, "\"{}\"", string),
        }
    }
}

impl From<Token> for Value
{
    fn from(input: Token) -> Value
    {
        match input
        {
            Token::Identifier(ident) => {
                if let Some(':') = ident.chars().next()
                {
                    Value::DataField(ident)
                }
                else
                {
                    Value::LocalVar(ident)
                }
            },
            Token::StringToken(string) => Value::StringVal(string),
            Token::YololNum(num) => Value::NumberVal(num),

            _ => panic!("Invalid conversion from Token to Value!")
        }
    }
}