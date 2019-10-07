use std::fmt;
use std::ops;
use std::cmp;

use serde::{Serialize, Deserialize};

use yolol_number::prelude::*;

use crate::types::{
    ast::operators::{Operator, OperatorError},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LiteralValue
{
    NumberVal(YololNumber),
    StringVal(String),
}

impl LiteralValue
{
    pub fn get_false() -> Self
    {
        LiteralValue::NumberVal(YololNumber::falsy())
    }

    pub fn get_true() -> Self
    {
        LiteralValue::NumberVal(YololNumber::truthy())
    }

    fn string_sub(self, other: Self) -> Self
    {
        let self_string = match self
        {
            LiteralValue::NumberVal(num) => num.to_string(),
            LiteralValue::StringVal(string) => string
        };

        let other_string = match other
        {
            LiteralValue::NumberVal(num) => num.to_string(),
            LiteralValue::StringVal(string) => string
        };

        let mut match_iter = self_string.rmatch_indices(other_string.as_str());
        let match_len = other_string.len();

        let match_index = match match_iter.next()
        {
            Some((index, _)) => index,
            _ => return LiteralValue::StringVal(self_string)
        };

        let input_front = &self_string[0..match_index];
        let input_back = &self_string[(match_index + match_len)..self_string.len()];

        let output_string = String::from(input_front);
        LiteralValue::StringVal(output_string + input_back)
    }

    pub fn pow(self, other: Self) -> Result<LiteralValue, OperatorError>
    {
        match (self, other)
        {
            (LiteralValue::NumberVal(self_num), LiteralValue::NumberVal(other_num)) => {
                Ok(LiteralValue::NumberVal(self_num.pow(other_num)))
            },

            (left, right) => Err(OperatorError::new(Operator::Pow, Some(left), Some(right),
                        String::from("Attempt to do pow operation without two number values")))
        }
    }

    pub fn factorial(self) -> Result<LiteralValue, OperatorError>
    {
        Err(OperatorError::new(Operator::Fact, Some(self), None,
            String::from("Factorial operation isn't implemented currently!")))
    }

    pub fn abs(self) -> Result<LiteralValue, OperatorError>
    {
        if let LiteralValue::NumberVal(num) = self
        {
            Ok(LiteralValue::NumberVal(num.abs()))
        }
        else
        {
            Err(OperatorError::new(Operator::Abs, Some(self), None,
                String::from("Tried to abs on non-number literal value!")))
        }
    }

    pub fn sqrt(self) -> Result<LiteralValue, OperatorError>
    {
        if let LiteralValue::NumberVal(num) = self
        {
            let output = num.sqrt();
            if output.is_negative()
            {
                Err(OperatorError::new(Operator::Sqrt, Some(self), None,
                    String::from("Sqrt input was out of range!")))
            }
            else
            {
                Ok(LiteralValue::NumberVal(output))
            }
        }
        else
        {
            Err(OperatorError::new(Operator::Sqrt, Some(self), None,
                String::from("Tried to sqrt on non-number literal value!")))
        }
    }

    pub fn sin(self) -> Result<LiteralValue, OperatorError>
    {
        if let LiteralValue::NumberVal(num) = self
        {
            Ok(LiteralValue::NumberVal(num.sin()))
        }
        else
        {
            Err(OperatorError::new(Operator::Sin, Some(self), None,
                String::from("Tried to sin on non-number literal value!")))
        }
    }

    pub fn cos(self) -> Result<LiteralValue, OperatorError>
    {
        if let LiteralValue::NumberVal(num) = self
        {
            Ok(LiteralValue::NumberVal(num.cos()))
        }
        else
        {
            Err(OperatorError::new(Operator::Cos, Some(self), None,
                String::from("Tried to cos on non-number literal value!")))
        }
    }

    pub fn tan(self) -> Result<LiteralValue, OperatorError>
    {
        if let LiteralValue::NumberVal(num) = self
        {
            Ok(LiteralValue::NumberVal(num.tan()))
        }
        else
        {
            Err(OperatorError::new(Operator::Tan, Some(self), None,
                String::from("Tried to tan on non-number literal value!")))
        }
    }

    pub fn arcsin(self) -> Result<LiteralValue, OperatorError>
    {
        if let LiteralValue::NumberVal(num) = self
        {
            let output = num.asin();
            if output.is_negative()
            {
                Err(OperatorError::new(Operator::Arcsin, Some(self), None,
                    String::from("Arcsin input was out of range!")))
            }
            else
            {
                Ok(LiteralValue::NumberVal(output))
            }
        }
        else
        {
            Err(OperatorError::new(Operator::Arcsin, Some(self), None,
                String::from("Tried to arcsin on non-number literal value!")))
        }
    }

    pub fn arccos(self) -> Result<LiteralValue, OperatorError>
    {
        if let LiteralValue::NumberVal(num) = self
        {
            let output = num.acos();
            if output.is_negative()
            {
                Err(OperatorError::new(Operator::Arccos, Some(self), None,
                    String::from("Arccos input was out of range!")))
            }
            else
            {
                Ok(LiteralValue::NumberVal(output))
            }
        }
        else
        {
            Err(OperatorError::new(Operator::Arccos, Some(self), None,
                String::from("Tried to arccos on non-number literal value!")))
        }
    }

    pub fn arctan(self) -> Result<LiteralValue, OperatorError>
    {
        if let LiteralValue::NumberVal(num) = self
        {
            let output = num.atan();
            if output.is_negative()
            {
                Err(OperatorError::new(Operator::Arctan, Some(self), None,
                    String::from("Arctan input was out of range!")))
            }
            else
            {
                Ok(LiteralValue::NumberVal(output))
            }
        }
        else
        {
            Err(OperatorError::new(Operator::Arctan, Some(self), None,
                String::from("Tried to arctan on non-number literal value!")))
        }
    }
}

impl fmt::Display for LiteralValue
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self
        {
            LiteralValue::NumberVal(num) => write!(f, "{}", num),
            LiteralValue::StringVal(string) => write!(f, "\"{}\"", string),
        }
    }
}

impl From<bool> for LiteralValue
{
    // Ignore this clippy lint since this makes the conversion more expressive
    #[allow(clippy::match_bool)]
    fn from(input: bool) -> LiteralValue
    {
        match input
        {
            true => LiteralValue::NumberVal(YololNumber::truthy()),
            false => LiteralValue::NumberVal(YololNumber::falsy())
        }
    }
}

impl From<YololNumber> for LiteralValue
{
    fn from(input: YololNumber) -> LiteralValue
    {
        LiteralValue::NumberVal(input)
    }
}

impl From<i64> for LiteralValue
{
    fn from(input: i64) -> LiteralValue
    {
        LiteralValue::NumberVal(YololNumber::from_value(input))
    }
}

impl From<&str> for LiteralValue
{
    fn from(input: &str) -> LiteralValue
    {
        LiteralValue::StringVal(String::from(input))
    }
}

impl PartialEq for LiteralValue
{
    fn eq(&self, other: &Self) -> bool
    {
        match (self, other)
        {
            (LiteralValue::NumberVal(self_num), LiteralValue::NumberVal(other_num)) => {
                self_num == other_num
            }

            (LiteralValue::StringVal(self_string), LiteralValue::StringVal(other_string)) => {
                self_string == other_string
            },

            _ => false
        }
    }
}

impl Eq for LiteralValue {}

impl Ord for LiteralValue
{
    fn cmp(&self, other: &Self) -> cmp::Ordering
    {
        match (self, other)
        {
            (LiteralValue::NumberVal(self_num), LiteralValue::NumberVal(other_num)) => {
                self_num.cmp(other_num)
            }

            (LiteralValue::NumberVal(self_num), LiteralValue::StringVal(other_string)) => {
               self_num.to_string().cmp(other_string)
            },

            (LiteralValue::StringVal(self_string), LiteralValue::StringVal(other_string)) => {
                self_string.cmp(other_string)
            },

            (LiteralValue::StringVal(self_string), LiteralValue::NumberVal(other_num)) => {
                self_string.cmp(&other_num.to_string())
            },
        }
    }
}

impl PartialOrd for LiteralValue
{
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering>
    {
        Some(self.cmp(other))
    }
}

impl ops::Add<LiteralValue> for LiteralValue
{
    type Output = Result<LiteralValue, OperatorError>;
    fn add(self: Self, other: Self) -> Self::Output
    {
        let output = match (self, other)
        {
            (LiteralValue::NumberVal(self_num), LiteralValue::NumberVal(other_num)) => {
                LiteralValue::NumberVal(self_num + other_num)
            }

            (LiteralValue::NumberVal(self_num), LiteralValue::StringVal(other_string)) => {
                LiteralValue::StringVal(self_num.to_string() + other_string.as_str())
            },

            (LiteralValue::StringVal(self_string), LiteralValue::StringVal(other_string)) => {
                LiteralValue::StringVal(self_string + other_string.as_str())
            },

            (LiteralValue::StringVal(self_string), LiteralValue::NumberVal(other_num)) => {
                LiteralValue::StringVal(self_string + other_num.to_string().as_str())
            },
        };

        Ok(output)
    }
}

impl ops::Sub<LiteralValue> for LiteralValue
{
    type Output = Result<LiteralValue, OperatorError>;
    fn sub(self: Self, other: Self) -> Self::Output
    {
        let output = match (self, other)
        {
            (LiteralValue::NumberVal(self_num), LiteralValue::NumberVal(other_num)) => {
                LiteralValue::NumberVal(self_num - other_num)
            }

            (left, right) => left.string_sub(right)
        };

        Ok(output)
    }
}

impl ops::Mul<LiteralValue> for LiteralValue
{
    type Output = Result<LiteralValue, OperatorError>;
    fn mul(self: Self, other: Self) -> Self::Output
    {
        match (self, other)
        {
            (LiteralValue::NumberVal(self_num), LiteralValue::NumberVal(other_num)) => {
                Ok(LiteralValue::NumberVal(self_num * other_num))
            },

            (left, right) => Err(OperatorError::new(Operator::Mul, Some(left), Some(right),
                        String::from("Attempt to do mul operation without two number values")))
        }
    }
}

impl ops::Div<LiteralValue> for LiteralValue
{
    type Output = Result<LiteralValue, OperatorError>;
    fn div(self: Self, other: Self) -> Self::Output
    {
        match (self, other)
        {
            (LiteralValue::NumberVal(self_num), LiteralValue::NumberVal(other_num)) => {
                Ok(LiteralValue::NumberVal(self_num.yolol_div(other_num)
                    .ok_or_else(|| OperatorError::new(Operator::Div, None, None, "Did bad div thing! This error needs to be made better too!".to_owned()))?))
            },

            (left, right) => Err(OperatorError::new(Operator::Div, Some(left), Some(right),
                        String::from("Attempt to do div operation without two number values")))
        }
    }
}

impl ops::Rem<LiteralValue> for LiteralValue
{
    type Output = Result<LiteralValue, OperatorError>;
    fn rem(self: Self, other: Self) -> Self::Output
    {
        match (self, other)
        {
            (LiteralValue::NumberVal(self_num), LiteralValue::NumberVal(other_num)) => {
                Ok(LiteralValue::NumberVal(self_num % other_num))
            },

            (left, right) => Err(OperatorError::new(Operator::Mod, Some(left), Some(right),
                        String::from("Attempt to do mod operation without two number values")))
        }
    }
}

impl ops::Neg for LiteralValue
{
    type Output = Result<LiteralValue, OperatorError>;
    fn neg(self: Self) -> Self::Output
    {
        match self
        {
            LiteralValue::NumberVal(self_num) => {
                Ok(LiteralValue::NumberVal(-self_num))
            },

            _ => Err(OperatorError::new(Operator::Negate, Some(self), None,
                        String::from("Attempt to do neg operation without a number value")))
        }
    }
}

impl ops::Not for LiteralValue
{
    type Output = Result<LiteralValue, OperatorError>;
    fn not(self: Self) -> Self::Output
    {
        if self == LiteralValue::get_false()
        {
            Ok(LiteralValue::get_true())
        }
        else
        {
            Ok(LiteralValue::get_false())
        }
    }
}
