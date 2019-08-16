use std::fmt;
use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use yolol_number::prelude::*;

use crate::types::ast::value::LiteralValue;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment
{
    pub name: String,
    pub version: String,
    pub next_line: i64,
    pub error: String,

    local_context: HashMap<String, LiteralValue>,
    global_context: HashMap<String, LiteralValue>
}

impl Environment
{
    pub fn new(name: &str) -> Environment
    {
        let name = String::from(name);
        let version = String::from(env!("CARGO_PKG_VERSION"));

        // We start at the first line on a chip
        let next_line = 1;

        let local_context = HashMap::new();
        let global_context = HashMap::new();

        Environment {
            name,
            version,
            next_line,
            error: String::new(),
            local_context,
            global_context,
        }
    }

    pub fn set_next_line(&mut self, num: YololNumber)
    {
        self.next_line = num.bound().get_value();
    }
}

impl fmt::Display for Environment
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        let mut out_string = format!("Environment from Yoloxide version {}\n", self.version);
        out_string += &format!("Name: {}, next line: {}.\n", self.name, self.next_line);

        out_string += "\n";
        out_string += "Local context:\n";
        for (key, value) in &self.local_context
        {
            out_string += &format!("Key: '{}', Value: '{}'\n", key, value);
        }

        out_string += "\n";
        out_string += "Global context:\n";
        for (key, value) in &self.global_context
        {
            out_string += &format!("Key: '{}', Value: '{}'\n", key, value);
        }

        write!(f, "{}", out_string)
    }
}

pub trait ContextMap
{
    fn get_val(&self, ident: &str) -> LiteralValue;
    fn set_val(&mut self, ident: String, value: LiteralValue);
}

impl ContextMap for Environment
{
    fn get_val(&self, ident: &str) -> LiteralValue
    {
        // Means the ident is referencing a data field, aka the global context
        if let Some(':') = ident.chars().next()
        {
            self.global_context.get(ident)
                .unwrap_or(&LiteralValue::get_false())
                .clone()
        }
        else
        {
            self.local_context.get(ident)
                .unwrap_or(&LiteralValue::get_false())
                .clone()
        }
    }

    fn set_val(&mut self, ident: String, value: LiteralValue)
    {
         // Means the ident is referencing a data field, aka the global context
        if let Some(':') = ident.chars().next()
        {
            self.global_context.insert(ident, value);
        }
        else
        {
            self.local_context.insert(ident, value);
        }
    }
}
