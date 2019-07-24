use std::fmt;
use std::convert::{TryFrom, TryInto};

use super::line::Line;

use super::cylon_ast::Program as CylonProgram;

#[derive(Debug, Clone)]
pub struct Program(pub Vec<Line>);

impl fmt::Display for Program
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        let mut output_string = String::from("");
        for line in &self.0
        {
            output_string += &format!("{}\n", &line);
        }

        write!(f, "{}", output_string)
    }
}

impl TryFrom<CylonProgram> for Program
{
    type Error = String;
    fn try_from(program: CylonProgram) -> Result<Self, Self::Error>
    {
        let mut ast_program = vec![];
        for line in program.lines
        {
            ast_program.push(line.try_into()?);
        }

        Ok(Program(ast_program))
    }
}