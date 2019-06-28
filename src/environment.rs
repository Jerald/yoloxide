use std::collections::HashMap;
use std::ops;

// use crate::types::Token;
// use crate::types::YololNumber;

use crate::types::Token;
use crate::types::Token as Val;
use crate::types::Statement as Stat;
use crate::types::Expression as Expr;
use crate::types::Operator as Op;

use crate::types::ParseErrorKind;
use crate::types::ExprError;
use crate::types::StatError;

use crate::types::SlidingWindow;
use crate::types::VecWindow;

use crate::types::YololNumber;


#[derive(Debug, Clone)]
pub struct Environment
{
    pub name: String,
    pub context: HashMap<String, Token>,
}

impl Environment
{
    pub fn new(name: String) -> Environment
    {
        let mut context = HashMap::new();
        context.insert(String::from("0"), Token::YololNum(YololNumber::from(0)));

        Environment {
            name,
            context,
        }
    }
}

pub trait ContextMap
{
    fn get_val(&self, ident: &str) -> Token;
    fn get_zero(&self) -> Token;
}

impl ContextMap for HashMap<String, Token>
{
    fn get_val(&self, ident: &str) -> Token
    {
        self.get(ident)
            .unwrap_or(self.get("0").unwrap())
            .clone()
    }

    fn get_zero(&self) -> Token
    {
        self.get("0")
            .unwrap()
            .clone()
    }
}
