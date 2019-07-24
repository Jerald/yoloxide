use std::fmt;
use std::convert::{TryFrom, TryInto};

use crate::types::ast::statement::Statement;

use super::cylon_ast::Line as CylonLine;

#[derive(Debug, Clone)]
pub struct Line(pub Vec<Statement>);

impl fmt::Display for Line
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        let mut output_string = String::from("");
        for statement in &self.0
        {
            output_string += &format!("{} ", &statement);
        }

        write!(f, "{}", output_string)
    }
}

impl TryFrom<CylonLine> for Line
{
    type Error = String;
    fn try_from(line: CylonLine) -> Result<Self, Self::Error>
    {
        let mut ast_line = vec![];
        for stat in line.code
        {
            ast_line.push(stat.try_into()?);
        }

        if line.comment.is_empty() == false
        {
            ast_line.push(Statement::Comment(line.comment));
        }

        Ok(Line(ast_line))
    }
}