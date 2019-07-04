use std::fmt;
use std::error;
use std::convert::TryInto;

use super::YololNumber;
use super::Operator;
use super::Value;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token
{
    Comment(String),
    Identifier(String),
    DataField(String),
    StringToken(String),
    YololNum(YololNumber),

    Newline,

    Equal,
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    LAngleBrak,
    RAngleBrak,
    Exclam,
    Caret,
    Percent,
}

impl fmt::Display for Token
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        let write_value: String = match self
        {
            Token::Comment(string) => format!("// {}\n", string),
            Token::Identifier(string) => string.clone(),
            Token::DataField(string) => format!(":{}", string),
            Token::StringToken(string) => format!("\"{}\"", string),
            Token::YololNum(num) => format!("{}", num),

            Token::Newline => "\n".to_owned(),

            Token::Equal => "=".to_owned(),
            Token::Plus => "+".to_owned(),
            Token::Minus => "-".to_owned(),
            Token::Star => "*".to_owned(),
            Token::Slash => "/".to_owned(),
            Token::LParen => "(".to_owned(),
            Token::RParen => ")".to_owned(),
            Token::LAngleBrak => "<".to_owned(),
            Token::RAngleBrak => ">".to_owned(),
            Token::Exclam => "!".to_owned(),
            Token::Caret => "^".to_owned(),
            Token::Percent => "%".to_owned(),
        };

        write!(f, "{}", write_value)
    }
}








