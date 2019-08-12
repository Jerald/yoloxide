use std::fmt;
use std::error;

pub mod ast;

mod sliding_window;
pub use sliding_window::*;

mod token;
pub use token::*;

#[derive(Debug, Clone)]
pub struct EvaluationError
{
    pub kind: EvaluationErrorKind,
    pub error_text: String
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EvaluationErrorKind
{
    OperatorError,
    NonExhaustivePattern,
    HitGoto,
    Misc
}

impl error::Error for EvaluationError
{
    fn source(&self) -> Option<&(dyn error::Error + 'static)>
    {
        None
    }
}

impl fmt::Display for EvaluationError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "[Evaluation Error] Kind: {:?} Error: {}", self.kind, self.error_text)
    }
}

impl From<ast::operators::OperatorError> for EvaluationError
{
    fn from(input: ast::operators::OperatorError) -> EvaluationError
    {
        EvaluationError {
            kind: EvaluationErrorKind::OperatorError,
            error_text: format!("op: {:?}, left: {:?}, right: {:?}, message: {}", input.op, input.left, input.right, input.error_text)
        }
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
    pub input_expr: Option<ast::expression::Expression>,
    pub kind: ParseErrorKind,
    pub error_text: String,
}

impl ExprError
{
    pub fn new(expr: Option<ast::expression::Expression>, kind: ParseErrorKind, error_text: &str) -> ExprError
    {
        ExprError {
            input_expr: expr,
            kind,
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
    pub input_stat: Option<ast::statement::Statement>,
    pub kind: ParseErrorKind,
    pub error_text: String,
}

impl StatError
{
    pub fn new(stat: Option<ast::statement::Statement>, kind: ParseErrorKind, error_text: &str) -> StatError
    {
        StatError {
            input_stat: stat,
            kind,
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
            Some(expr) => Some(ast::statement::Statement::Expression(Box::new(expr))),
            None => None
        };

        StatError {
            input_stat: stat,
            kind,
            error_text
        }
    }
}

