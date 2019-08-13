use std::convert::{TryFrom, TryInto};

use yolol_number::YololNumber;

use cylon_ast::{
    CylonProg,
    CylonLine,
    CylonStat,
    CylonExpr,

    boxed_from_impl,
    boxed_try_from_impl,
};

use super::{
    program::Program        as AstProgram,
    line::Line              as AstLine,
    statement::Statement    as AstStat,
    expression::Expression  as AstExpr,
    value::Value            as AstValue,
    operators::Operator     as Op,
};

boxed_from_impl! {
    From<Box<AstExpr>> for Box<CylonExpr>
}

boxed_try_from_impl! {
    TryFrom<Box<CylonStat>> for Box<AstStat>;
    TryFrom<Box<CylonExpr>> for Box<AstExpr>
}

impl From<AstProgram> for CylonProg
{
    fn from(program: AstProgram) -> CylonProg
    {
        let lines = program.0.into_iter().map(Into::into).collect();

        CylonProg {
            lines
        }
    }
}

impl TryFrom<CylonProg> for AstProgram
{
    type Error = String;
    fn try_from(program: CylonProg) -> Result<Self, Self::Error>
    {
        let mut ast_program = vec![];
        for line in program.lines
        {
            ast_program.push(line.try_into()?);
        }

        Ok(AstProgram(ast_program))
    }
}

// impl From<&AstProgram> for CylonProgram
// {
//     fn from (program: &AstProgram) -> CylonProgram
//     {
//         program.clone().into()
//     }
// }

impl From<AstLine> for CylonLine
{
    fn from(line: AstLine) -> CylonLine
    {
        let mut output = Vec::new();
        let mut comment = None;

        for stat in line.0
        {
            match stat
            {
                AstStat::Comment(string) => {
                    comment = Some(string);
                }

                _ => {
                    output.push(stat.into());
                }
            }
        }

        CylonLine {
            comment: comment.unwrap_or_else(|| String::from("")),
            code: output
        }
    }
}

impl TryFrom<CylonLine> for AstLine
{
    type Error = String;
    fn try_from(line: CylonLine) -> Result<Self, Self::Error>
    {
        // let mut ast_line = vec![];
        // for stat in line.code
        // {
        //     ast_line.push(stat.try_into()?);
        // }

        let mut ast_line: Vec<AstStat> = line.code.into_iter()
            .map(|s| s.try_into())
            .collect::<Result<_, Self::Error>>()?;

        if !line.comment.is_empty()
        {
            ast_line.push(AstStat::Comment(line.comment));
        }

        Ok(AstLine(ast_line))
    }
}

impl From<AstStat> for CylonStat
{
    fn from(stat: AstStat) -> CylonStat
    {
        match stat
        {
            AstStat::Comment(_) => {
                panic!("Converting a ast::Statement comment into a cylon_ast::Statement isn't supported currently!")
            },

            AstStat::If(cond, body, else_body) => {
                let body: Vec<CylonStat> = body.into_iter()
                    .map(|s| s.into())
                    .collect();

                let else_body = match else_body {
                    Some(vec) => vec.into_iter()
                        .map(|s| s.into())
                        .collect(),
                        
                    None => vec![]
                };

                CylonStat::If {
                    condition: (*cond).into(),
                    body,
                    else_body,
                }
            },

            AstStat::Goto(expr) => {
                CylonStat::Goto {
                    expression: (*expr).into()
                }
            },

            AstStat::Assignment(ident, op, expr) => {
                CylonStat::Assignment {
                    identifier: ident.to_string(),
                    operator: op.to_string(),
                    value: (*expr).into()
                }
            },

            AstStat::Expression(expr) => {
                CylonStat::Expression {
                    expression: (*expr).into()
                }
            }
        }
    }
}

impl TryFrom<CylonStat> for AstStat
{
    type Error = String;
    fn try_from(stat: CylonStat) -> Result<Self, Self::Error>
    {
        match stat
        {
            CylonStat::Goto { expression } => {
                let expr = Box::new(expression.try_into()?);
                Ok(AstStat::Goto(expr))
            },
            CylonStat::If { condition, body, else_body } => {
                let cond = Box::new(condition.try_into()?);

                let ast_body: Vec<AstStat> = body.into_iter()
                    .map(|s| s.try_into())
                    .collect::<Result<_, Self::Error>>()?;

                let ast_else_body: Vec<AstStat> = else_body.into_iter()
                    .map(|s| s.try_into())
                    .collect::<Result<_, Self::Error>>()?;

                if ast_else_body.is_empty()
                {
                    Ok(AstStat::If(cond, ast_body, None))
                }
                else
                {
                    Ok(AstStat::If(cond, ast_body, Some(ast_else_body)))
                }
            },
            CylonStat::Assignment { identifier, operator, value } => {
                let ident = if identifier.starts_with(':')
                {
                    AstValue::DataField(identifier)
                }
                else
                {
                    AstValue::LocalVar(identifier)
                };

                let op = match operator.as_str()
                {
                    "=" => Op::Assign,
                    "+=" => Op::AddAssign,
                    "-=" => Op::SubAssign,
                    "*=" => Op::MulAssign,
                    "/=" => Op::DivAssign,
                    "%=" => Op::ModAssign,

                    bad_op => return Err(format!("[Statement::TryFrom<CylonStat>] Unable to convert to assignment op from string! Found '{}'", bad_op))
                };

                let value = Box::new(value.try_into()?);
                Ok(AstStat::Assignment(ident, op, value))
            },
            CylonStat::Expression { expression } => {
                let expr = Box::new(expression.try_into()?);
                Ok(AstStat::Expression(expr))
            }
        }
    }
}

impl From<AstExpr> for CylonExpr
{
    fn from(expr: AstExpr) -> CylonExpr
    {
        match expr
        {
            AstExpr::BinaryOp(op, left, right) => {
                CylonExpr::BinaryOp {
                    operator: op.to_string(),
                    left: left.into(),
                    right: right.into(),
                }
            },
            AstExpr::UnaryOp(op, operand) => {
                // Specific fix for pre/post ops, due to their special form
                let op_string = match op
                {
                    Op::PreInc  => "++a".to_owned(),
                    Op::PostInc => "a++".to_owned(),

                    Op::PreDec  => "--a".to_owned(),
                    Op::PostDec => "a--".to_owned(),

                    op => op.to_string()
                };

                CylonExpr::UnaryOp {
                    operator: op_string,
                    operand: operand.into()
                }
            },
            AstExpr::Value(value) => {
                value.into()
            }
        }
    }
}

impl TryFrom<CylonExpr> for AstExpr
{
    type Error = String;
    
    fn try_from(expr: CylonExpr) -> Result<Self, Self::Error>
    {
        match expr
        {
            CylonExpr::Group { group } => {
                let value = AstValue::Group(group.try_into()?);
                Ok(AstExpr::Value(value))
            },
            CylonExpr::BinaryOp { operator, left, right } => {
                let op = match operator.as_str()
                {
                    "<" => Op::Lesser,
                    ">" => Op::Greater,
                    "<=" => Op::LesserEq,
                    ">=" => Op::GreaterEq,
                    "==" => Op::Equal,
                    "!=" => Op::NotEqual,
                    "and" => Op::And,
                    "or" => Op::Or,

                    "+" => Op::Add,
                    "-" => Op::Sub,
                    "*" => Op::Mul,
                    "/" => Op::Div,
                    "%" => Op::Mod,
                    "^" => Op::Pow,

                    bad_op => return Err(format!("[AstExpr::TryFrom<CylonExpr>] Unable to convert to binary op from string! Found {}", bad_op))
                };

                Ok(AstExpr::BinaryOp(op, left.try_into()?, right.try_into()?))
            },
            CylonExpr::UnaryOp { operator, operand } => {
                let op = match operator.as_str()
                {
                    "-" => Op::Negate,
                    "++a" => Op::PreInc,
                    "a++" => Op::PostInc,
                    "--a" => Op::PreDec,
                    "a--" => Op::PostDec,
                    "!" => Op::Fact,

                    "abs" => Op::Abs,
                    "sqrt" => Op::Sqrt,
                    "sin" => Op::Sin,
                    "cos" => Op::Cos,
                    "tan" => Op::Tan,
                    "asin" => Op::Arcsin,
                    "acos" => Op::Arccos,
                    "atan" => Op::Arctan,
                    "not" => Op::Not,

                    bad_op => return Err(format!("[AstExpr::TryFrom<CylonExpr>] Unable to convert to unary op from string! Found {}", bad_op))
                };

                Ok(AstExpr::UnaryOp(op, operand.try_into()?))
            },
            CylonExpr::Number { num } => {
                let yolol_num = num.parse::<YololNumber>()?;
                let value = AstValue::NumberVal(yolol_num);

                Ok(AstExpr::Value(value))
            },
            CylonExpr::String { str } => {
                let value = AstValue::StringVal(str);
                Ok(AstExpr::Value(value))
            },
            CylonExpr::Identifier { name } => {
                let value = if name.starts_with(':')
                {
                    AstValue::DataField(name)
                }
                else
                {
                    AstValue::LocalVar(name)
                };

                Ok(AstExpr::Value(value))
            }
        }
    }
}

impl From<AstValue> for CylonExpr
{
    fn from(value: AstValue) -> CylonExpr
    {
        match value
        {
            AstValue::Group(expr) => {
                CylonExpr::Group {
                    group: expr.into()
                }
            },

            AstValue::LocalVar(ident) |
            AstValue::DataField(ident) => {
                CylonExpr::Identifier {
                    name: ident
                }
            },

            AstValue::NumberVal(num) => {
                CylonExpr::Number {
                    num: num.to_string()
                }
            },

            AstValue::StringVal(string) => {
                CylonExpr::String {
                    str: string
                }
            }
        }
    }
}
