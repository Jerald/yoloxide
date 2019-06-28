use std::fmt;
use std::error;
use std::convert::TryInto;

use super::YololNumber;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token
{
    Comment(String),
    Identifier(String),
    DataField(String),
    StringToken(String),
    YololNum(YololNumber),

    Space,
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

impl std::ops::Add for Token
{
    type Output = Self;
    fn add(self, other: Self) -> Self
    {
        match (self, other)
        {
            (Token::StringToken(self_string), Token::StringToken(other_string)) => {
                Token::StringToken(self_string + &other_string)
            },
            (Token::StringToken(self_string), Token::YololNum(other_num)) => {
                Token::StringToken(self_string + &other_num.to_string())
            },

            (Token::YololNum(self_num), Token::StringToken(other_string)) => {
                Token::StringToken(self_num.to_string() + &other_string)
            },
            (Token::YololNum(self_num), Token::YololNum(other_num)) => {
                Token::YololNum(self_num + other_num)
            },

            _ => panic!("Fix code!")
        }
    }
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

            Token::Space => " ".to_owned(),
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