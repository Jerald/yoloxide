use std::fmt;

use super::Statement;

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

// impl fmt::Display for Vec<Line>
// {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
//     {
//         let mut output = String::from("");
//         for line in self
//         {
//             output += &format!("{}\n", &line);
//         }

//         write!(f, "{}", output)
//     }
// }
