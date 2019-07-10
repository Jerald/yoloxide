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
    StringToken(String),
    YololNum(YololNumber),

    Goto,

    If,
    Then,
    Else,
    End,

    Abs,
    Sqrt,
    Sin,
    Cos,
    Tan,
    Arcsin,
    Arccos,
    Arctan,
    Not,

    Or,
    And,

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
            Token::StringToken(string) => format!("\"{}\"", string),
            Token::YololNum(num) => format!("{}", num),

            Token::Goto => "goto".to_owned(),

            Token::If => "if".to_owned(),
            Token::Then => "then".to_owned(),
            Token::Else => "else".to_owned(),
            Token::End => "end".to_owned(),

            Token::Abs => "abs".to_owned(),
            Token::Sqrt => "sqrt".to_owned(),
            Token::Sin => "sin".to_owned(),
            Token::Cos => "cos".to_owned(),
            Token::Tan => "tan".to_owned(),
            Token::Arcsin => "arcsin".to_owned(),
            Token::Arccos => "arccos".to_owned(),
            Token::Arctan => "arctan".to_owned(),
            Token::Not => "not".to_owned(),

            Token::Or => "or".to_owned(),
            Token::And => "and".to_owned(),

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








