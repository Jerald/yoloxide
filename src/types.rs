use std::fmt;
use std::error;
use std::convert::TryInto;

mod sliding_window;
pub use sliding_window::*;

mod yolol_number;
pub use yolol_number::*;

mod token;
pub use token::*;

// TODO: make a value enum for strings and yololnums

#[derive(Debug, PartialEq, Clone)]
pub enum Statement
{
    If(Box<Expression>, Vec<Statement>, Option<Vec<Statement>>),
    Goto(Box<Expression>),
    Assignment(Token, Operator, Box<Expression>),
    Expression(Box<Expression>)
}

impl fmt::Display for Statement
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        let write_value: String = match self
        {
            Statement::If(cond, body, Some(else_body)) => format!("if {} then {:?} else {:?} end", cond, body, else_body),
            Statement::If(cond, body, None) => format!("if {} then {:?} end", cond, body),

            Statement::Goto(expr) => format!("goto {}", expr.as_ref()),
            Statement::Assignment(ident, op, value) => format!("{} {} {}", ident, op, value),

            Statement::Expression(expr) => format!("{}", expr.as_ref()),
        };

        write!(f, "{}", write_value)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression
{
    Grouping(Box<Expression>),
    BinaryOp(Operator, Box<Expression>, Box<Expression>),
    UnaryOp(Operator, Box<Expression>),
    Value(Token)
}

impl fmt::Display for Expression
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        let write_value: String = match self
        {
            Expression::Grouping(expr) => format!("({})", expr.as_ref()),
            Expression::BinaryOp(op, left, right) => format!("{} {} {}", left.as_ref(), op, right.as_ref()),

            Expression::UnaryOp(op @ Operator::Negate, value) |
            Expression::UnaryOp(op @ Operator::PreInc, value) |
            Expression::UnaryOp(op @ Operator::PreDec, value)  => format!("{}{}", op, value.as_ref()),

            Expression::UnaryOp(op @ Operator::Abs, value) |
            Expression::UnaryOp(op @ Operator::Sqrt, value) |
            Expression::UnaryOp(op @ Operator::Sin, value) |
            Expression::UnaryOp(op @ Operator::Cos, value) |
            Expression::UnaryOp(op @ Operator::Tan, value) |
            Expression::UnaryOp(op @ Operator::Arcsin, value) |
            Expression::UnaryOp(op @ Operator::Arccos, value) |
            Expression::UnaryOp(op @ Operator::Arctan, value) |
            Expression::UnaryOp(op @ Operator::Not, value) => format!("{} {}", op, value.as_ref()),

            Expression::UnaryOp(op, value) => format!("{}{}", value.as_ref(), op),

            Expression::Value(token) => format!("{}", token),
        };

        write!(f, "{}", write_value)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator
{
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
    PowAssign,

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
            Operator::PowAssign => "^=",

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
            Operator::Arcsin => "arcsin",
            Operator::Arccos => "arccos",
            Operator::Arctan => "arctan",
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ParseErrorKind
{
    NoParseRuleMatch,

    RepeatedElseTokens,

    UnbalancedParenthesis,
    NoExtensionAvailable
}

#[derive(Debug, Clone)]
pub struct ExprError
{
    pub input_expr: Option<Box<Expression>>,
    pub kind: ParseErrorKind,
    pub error_text: String,
}

impl ExprError
{
    pub fn new(expr: Option<Box<Expression>>, kind: ParseErrorKind, error_text: &str) -> ExprError
    {
        ExprError {
            input_expr: expr,
            kind: kind,
            error_text: String::from(error_text)
        }
    }
}

impl fmt::Display for ExprError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", self.error_text)
    }
}

impl error::Error for ExprError
{
    fn source(&self) -> Option<&(dyn error::Error + 'static)>
    {
        None
    }
}

#[derive(Debug, Clone)]
pub struct StatError
{
    pub input_stat: Option<Box<Statement>>,
    pub kind: ParseErrorKind,
    pub error_text: String,
}

impl StatError
{
    pub fn new(stat: Option<Box<Statement>>, kind: ParseErrorKind, error_text: &str) -> StatError
    {
        StatError {
            input_stat: stat,
            kind: kind,
            error_text: String::from(error_text)
        }
    }
}

impl fmt::Display for StatError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", self.error_text)
    }
}

impl error::Error for StatError
{
    fn source(&self) -> Option<&(dyn error::Error + 'static)>
    {
        None
    }
}

impl std::convert::From<ExprError> for StatError
{
    fn from(error: ExprError) -> Self
    {
        let ExprError {
            input_expr,
            kind,
            error_text } = error;

        let stat = match input_expr
        {
            Some(expr) => Some(Box::new(Statement::Expression(expr))),
            None => None
        };

        StatError {
            input_stat: stat,
            kind,
            error_text
        }
    }
}

