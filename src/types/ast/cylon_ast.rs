use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use super::{
    program::Program        as AstProgram,
    line::Line              as AstLine,
    statement::Statement    as Stat,
    expression::Expression  as Expr,
    value::Value
};

const CYLON_AST_VERSION: &str = "0.3.0";

#[derive(Serialize, Deserialize)]
#[serde(rename = "root")]
#[serde(rename_all = "lowercase")]
pub struct Root
{
    pub version: String,
    pub metadata: HashMap<String, String>,
    pub program: Program
}

impl Root
{
    pub fn new(program: Program) -> Root
    {
        Root {
            program,
            ..Default::default()
        }
    }
}

impl Default for Root
{
    fn default() -> Self
    {
        let mut metadata = HashMap::new();
        metadata.insert("exporter".to_owned(), format!("yoloxide {}", env!("CARGO_PKG_VERSION")));

        Root {
            version: CYLON_AST_VERSION.to_owned(),
            metadata,
            program: Program {
                lines: vec![]
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "program")]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type")]
pub struct Program
{
    pub lines: Vec<Line>
}

impl From<AstProgram> for Program
{
    fn from(program: AstProgram) -> Program
    {
        let lines = program.0.into_iter().map(Into::into).collect();

        Program {
            lines
        }
    }
}

impl From<&AstProgram> for Program
{
    fn from (program: &AstProgram) -> Program
    {
        program.clone().into()
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "line")]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type")]
pub struct Line
{
    pub comment: String,
    pub code: Vec<Statement>
}

impl From<AstLine> for Line
{
    fn from(line: AstLine) -> Line
    {
        let mut output = Vec::new();
        let mut comment = None;

        for stat in line.0
        {
            match stat
            {
                Stat::Comment(string) => {
                    comment = Some(string);
                }

                _ => {
                    output.push(stat.into());
                }
            }
        }

        Line {
            comment: comment.unwrap_or_else(|| String::from("")),
            code: output
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type")]
pub enum Statement
{
    #[serde(rename = "statement::goto")]
    Goto { expression: Expression },

    #[serde(rename = "statement::if")]
    If { condition: Expression, body: Vec<Statement>, else_body: Vec<Statement> },

    #[serde(rename = "statement::assignment")]
    Assignment { identifier: String, operator: String, value: Expression },

    #[serde(rename = "statement::expression")]
    Expression { expression: Expression }
}

impl From<Stat> for Statement
{
    fn from(stat: Stat) -> Statement
    {
        match stat
        {
            Stat::Comment(_) => {
                panic!("Converting a ast::Statement comment into a cylon_ast::Statement isn't supported currently!")
            },

            Stat::If(cond, body, else_body) => {
                let body: Vec<Statement> = body.into_iter().map(Into::into).collect();

                let else_body = match else_body {
                    Some(vec) => vec.into_iter().map(Into::into).collect(),
                    None => vec![]
                };

                Statement::If {
                    condition: (*cond).into(),
                    body,
                    else_body,
                }
            },

            Stat::Goto(expr) => {
                Statement::Goto {
                    expression: (*expr).into()
                }
            },

            Stat::Assignment(ident, op, expr) => {
                Statement::Assignment {
                    identifier: ident.to_string(),
                    operator: op.to_string(),
                    value: (*expr).into()
                }
            },

            Stat::Expression(expr) => {
                let expr = (*expr).into();

                Statement::Expression {
                    expression: expr
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type")]
pub enum Expression
{
    #[serde(rename = "expression::group")]
    Group { group: Box<Expression> },

    #[serde(rename = "expression::binary_op")]
    BinaryOp { operator: String, left: Box<Expression>, right: Box<Expression> },

    #[serde(rename = "expression::unary_op")]
    UnaryOp { operator: String, operand: Box<Expression> },

    #[serde(rename = "expression::number")]
    Number { num: String },

    #[serde(rename = "expression::string")]
    String { str: String },

    #[serde(rename = "expression::identifier")]
    Identifier { name: String }
}

impl From<Expr> for Expression
{
    fn from(expr: Expr) -> Expression
    {
        match expr
        {
            Expr::BinaryOp(op, left, right) => {
                let left = (*left).into();
                let right = (*right).into();

                Expression::BinaryOp {
                    operator: op.to_string(),
                    left: Box::new(left),
                    right: Box::new(right)
                }
            },
            Expr::UnaryOp(op, operand) => {
                let operand = (*operand).into();

                Expression::UnaryOp {
                    operator: op.to_string(),
                    operand: Box::new(operand)
                }
            },
            Expr::Value(value) => {
                value.into()
            }
        }
    }
}

impl From<Value> for Expression
{
    fn from(value: Value) -> Expression
    {
        match value
        {
            Value::Group(expr) => {
                let expr = (*expr).into();

                Expression::Group {
                    group: Box::new(expr)
                }
            },

            Value::LocalVar(ident) |
            Value::DataField(ident) => {
                Expression::Identifier {
                    name: ident
                }
            },

            Value::NumberVal(num) => {
                Expression::Number {
                    num: num.to_string()
                }
            },

            Value::StringVal(string) => {
                Expression::String {
                    str: string
                }
            }
        }
    }
}
