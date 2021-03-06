use std::fmt;
use std::error;

use crate::types::ast::value::LiteralValue;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Operator
{
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,

    Negate,
    PreInc,
    PostInc,
    PreDec,
    PostDec,
    Fact,

    Abs,
    Sqrt,
    Sin,
    Cos,
    Tan,
    Arcsin,
    Arccos,
    Arctan,
    Not,

    Lesser,
    Greater,
    LesserEq,
    GreaterEq,
    Equal,
    NotEqual,
    And,
    Or,

    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow
}

impl fmt::Display for Operator
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        let write_value: &str = match self
        {
            Operator::Assign => "=",
            Operator::AddAssign => "+=",
            Operator::SubAssign => "-=",
            Operator::MulAssign => "*=",
            Operator::DivAssign => "/=",
            Operator::ModAssign => "%=",

            Operator::Negate => "-",
            Operator::PreInc => "++",
            Operator::PostInc => "++",
            Operator::PreDec => "--",
            Operator::PostDec => "--",
            Operator::Fact => "!",

            Operator::Abs  => "abs",
            Operator::Sqrt => "sqrt",
            Operator::Sin => "sin",
            Operator::Cos => "cos",
            Operator::Tan => "tan",
            Operator::Arcsin => "asin",
            Operator::Arccos => "acos",
            Operator::Arctan => "atan",
            Operator::Not => "not",

            Operator::Lesser => "<",
            Operator::Greater => ">",
            Operator::LesserEq => "<=",
            Operator::GreaterEq => ">=",
            Operator::Equal => "==",
            Operator::NotEqual => "!=",
            Operator::And => "and",
            Operator::Or => "or",

            Operator::Add => "+",
            Operator::Sub => "-",
            Operator::Mul => "*",
            Operator::Div => "/",
            Operator::Mod => "%",
            Operator::Pow => "^"
        };

        write!(f, "{}", write_value)
    }
}

// impl std::str::FromStr for Operator
// {
//     type Err = String;
//     fn from_str(string: &str) -> Result<Self, Self::Err>
//     {
//         let op = match string
//         {
//              "=" => Operator::Assign,
//              "+=" => Operator::AddAssign,
//              "-=" => Operator::SubAssign,
//              "*=" => Operator::MulAssign,
//              "/=" => Operator::DivAssign,
//              "%=" => Operator::ModAssign,

//              "-" => Operator::Negate,
//              "++" => Operator::PreInc,
//              "++" => Operator::PostInc,
//              "--" => Operator::PreDec,
//              "--" => Operator::PostDec,
//              "!" => Operator::Fact,

//              "abs" => Operator::Abs,
//              "sqrt" => Operator::Sqrt,
//              "sin" => Operator::Sin,
//              "cos" => Operator::Cos,
//              "tan" => Operator::Tan,
//              "arcsin" => Operator::Arcsin,
//              "arccos" => Operator::Arccos,
//              "arctan" => Operator::Arctan,
//              "not" => Operator::Not,

//              "<" => Operator::Lesser,
//              ">" => Operator::Greater,
//              "<=" => Operator::LesserEq,
//              ">=" => Operator::GreaterEq,
//              "==" => Operator::Equal,
//              "!=" => Operator::NotEqual,
//              "and" => Operator::And,
//              "or" => Operator::Or,

//              "+" => Operator::Add,
//              "-" => Operator::Sub,
//              "*" => Operator::Mul,
//              "/" => Operator::Div,
//              "%" => Operator::Mod,
//              "^" => Operator::Pow,

//              _ => return Err("Used FromStr on operator and didn't provide valid string!".to_owned())
//         };

//         Ok(op)
//     }
// }

#[derive(Debug, Clone)]
pub struct OperatorError
{
    pub op: Operator,
    pub left: Option<LiteralValue>,
    pub right: Option<LiteralValue>,
    pub error_text: String
}

impl OperatorError
{
    pub fn new(op: Operator, left: Option<LiteralValue>, right: Option<LiteralValue>, error_text: String) -> OperatorError
    {
        OperatorError {
            op,
            left,
            right,
            error_text
        }
    }
}

impl fmt::Display for OperatorError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "[OPERATOR ERROR] Op: {:?}, left: {:?}, right: {:?}. Error text: {}",
            self.op, self.left, self.right, self.error_text)
    }
}

impl error::Error for OperatorError
{
    fn source(&self) -> Option<&(dyn error::Error + 'static)>
    {
        None
    }
}