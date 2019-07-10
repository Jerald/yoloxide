use std::fmt;
use std::collections::HashMap;

use crate::types::LiteralValue;
use crate::types::YololNumber;

#[derive(Debug, Clone)]
pub struct Environment<'a>
{
    pub name: &'a str,
    pub next_line: i64,
    pub context: HashMap<String, LiteralValue>,
}

impl<'a> Environment<'a>
{
    pub fn new(name: &'a str) -> Environment
    {
        // We start at the first line on a chip
        let next_line = 1;

        let context = HashMap::new();

        Environment {
            name,
            next_line,
            context,
        }
    }

    pub fn set_next_line(&mut self, num: YololNumber)
    {
        self.next_line = i64::from(num);
    }
}

impl fmt::Display for Environment<'_>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        let mut out_string = format!("Name: {}, next line: {}.\n", self.name, self.next_line);

        out_string += "Context:\n";
        for (key, value) in self.context.iter()
        {
            out_string += format!("Key: '{}', Value: '{}'\n", key, value).as_str();
        }

        write!(f, "{}", out_string)
    }
}

pub trait ContextMap
{
    fn get_val(&self, ident: &str) -> LiteralValue;
    // fn get_zero(&self) -> LiteralValue;
}

impl ContextMap for HashMap<String, LiteralValue>
{
    fn get_val(&self, ident: &str) -> LiteralValue
    {
        self.get(ident)
            .unwrap_or(&LiteralValue::get_false())
            .clone()
    }

    // fn get_zero(&self) -> LiteralValue
    // {
    //     LiteralValue::get_false()
    // }
}
