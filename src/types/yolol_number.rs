use std::ops;
use std::cmp;
use std::str::FromStr;

use serde::{Serialize, Deserialize, Serializer};

const CONVERSION_CONST: i64 = 10000;

type InnerType = i64;

#[derive(Clone, Copy)]
pub struct YololNumber(InnerType);

impl YololNumber
{
    pub fn new(inner: InnerType) -> YololNumber
    {
        YololNumber(inner)
    }

    pub fn from_split(main: InnerType, decimal: InnerType) -> YololNumber
    {
        YololNumber(YololNumber::to_inner(main) + decimal)
    }

    fn to_inner(num: InnerType) -> InnerType
    {
        num.saturating_mul(CONVERSION_CONST)
    }

    fn from_inner(num: InnerType) -> InnerType
    {
        num / CONVERSION_CONST
    }

    pub fn is_negative(self) -> bool
    {
        self.0.is_negative()
    }

    pub fn floor(self) -> YololNumber
    {
        // By dividing by the conversion const, we wipe out all the decimal places
        YololNumber::from(self.0 / CONVERSION_CONST)
    }

    pub fn ceiling(self) -> YololNumber
    {
        // We get the value in the first decimal place
        let first_decimal = self.0 % CONVERSION_CONST;
        // Then we find out how far it is from 10
        let adjustment = CONVERSION_CONST.saturating_sub(first_decimal);

        // Then by adding that adjustment, we bring us to the next whole value
        YololNumber::new(self.0.saturating_add(adjustment))
    }

    pub fn clamp(self, min: InnerType, max: InnerType) -> YololNumber
    {
        if self.0 < min.saturating_mul(CONVERSION_CONST)
        {
            YololNumber::from(min)
        }
        else if self.0 > max.saturating_mul(CONVERSION_CONST)
        {
            YololNumber::from(max)
        }
        else
        {
            self
        }
    }

    pub fn pow(self, other: Self) -> Self
    {
        let float_self = (self.0 as f64) / (CONVERSION_CONST as f64);
        let float_other = (other.0 as f64) / (CONVERSION_CONST as f64);

        let pow = float_self.powf(float_other);

        // If our float pow overflowed, we need to map the value back to i64 space
        let int_pow = if pow.abs() > (std::i64::MAX as f64)
        {
            // This will map the sign of infinity to the correct i64 sign
            std::i64::MAX.saturating_mul(pow.signum() as i64)
        }
        else
        {
            // If it didn't overflow we can just directly cast it back
            pow as i64
        };

        let new_inner = int_pow.saturating_mul(CONVERSION_CONST);
        YololNumber::new(new_inner)
    }

    pub fn abs(self) -> YololNumber
    {
        YololNumber::new(self.0.saturating_abs())
    }

    pub fn sqrt(self) -> YololNumber
    {
        let float_value = (self.0 as f64) / (CONVERSION_CONST as f64);
        let output = float_value.sqrt();
        YololNumber::from(output as i64)
    }

    pub fn sin(self) -> YololNumber
    {
        let float_value = (self.0 as f64) / (CONVERSION_CONST as f64);
        let rads = float_value.to_radians();
        YololNumber::from(rads.sin() as i64)
    }

    pub fn cos(self) -> YololNumber
    {
        let float_value = (self.0 as f64) / (CONVERSION_CONST as f64);
        let rads = float_value.to_radians();
        YololNumber::from(rads.cos() as i64)
    }

    pub fn tan(self) -> YololNumber
    {
        let float_value = (self.0 as f64) / (CONVERSION_CONST as f64);
        let rads = float_value.to_radians();
        YololNumber::from(rads.tan() as i64)
    }

    pub fn arcsin(self) -> YololNumber
    {
        let float_value = (self.0 as f64) / (CONVERSION_CONST as f64);
        let rads = float_value.asin();
        YololNumber::from(rads.to_degrees() as i64)
    }

    pub fn arccos(self) -> YololNumber
    {
        let float_value = (self.0 as f64) / (CONVERSION_CONST as f64);
        let rads = float_value.acos();
        YololNumber::from(rads.to_degrees() as i64)
    }

    pub fn arctan(self) -> YololNumber
    {
        let float_value = (self.0 as f64) / (CONVERSION_CONST as f64);
        let rads = float_value.atan();
        YololNumber::from(rads.to_degrees() as i64)
    }
}

impl From<InnerType> for YololNumber
{
    fn from(input: InnerType) -> YololNumber
    {
        let num = YololNumber::to_inner(input);
        YololNumber(num)
    }
}

impl From<&InnerType> for YololNumber
{
    fn from(input: &InnerType) -> YololNumber
    {
        let num = YololNumber::to_inner(*input);
        YololNumber(num)
    }
}

impl From<YololNumber> for InnerType
{
    fn from(input: YololNumber) -> InnerType
    {
        YololNumber::from_inner(input.0)
    }
}

impl From<&YololNumber> for InnerType
{
    fn from(input: &YololNumber) -> InnerType
    {
        YololNumber::from_inner(input.0)
    }
}

impl FromStr for YololNumber
{
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err>
    {
        let (left_string, right_string) = if string.contains('.')
        {
            let split: Vec<&str> = string.split('.').collect();
            
            if split.len() != 2
            {
                return Err(format!("[YololNumber::from_str] Input string had {} decimal points!", split.len()));
            }

            (split[0], split[1])
        }
        else
        {
            (string, "")
        };

        // Ensure the left string is all ascii digits
        if !left_string.chars().all(|c| c.is_ascii_digit())
        {
            return Err("[YololNumber::from_str] Chars to left of decimal point aren't all numbers!".to_owned())
        }

        // Ensure the right string is either empty or all ascii digits
        if !right_string.is_empty() && !right_string.chars().all(|c| c.is_ascii_digit())
        {
            return Err("[YololNumber::from_str] Chars to right of decimal point aren't all numbers!".to_owned())
        }

        let parse_error_handler = |error: std::num::ParseIntError| {
            use std::num::IntErrorKind;
            match error.kind()
            {
                IntErrorKind::Empty |
                IntErrorKind::Zero => 0,

                IntErrorKind::Overflow => std::i64::MAX,
                IntErrorKind::Underflow => std::i64::MIN,

                IntErrorKind::InvalidDigit => panic!("[YololNumber::from_str] String to i64 parse error: somehow encountered a letter in the characters collected for a yolol number!"),
                _ => panic!("[YololNumber::from_str] Unknown String to i64 parse error when converting yolol number!")
            }
        };

        let left_num: i64 = left_string.parse::<i64>().unwrap_or_else(parse_error_handler);

        let right_num: i64 = match right_string.len()
        {
            0 => 0,

            len @ 1..=3 => {
                let shift: i64 = (10i64).pow(4 - (len as u32));
                let num = right_string[0..len].parse::<i64>().unwrap_or_else(parse_error_handler);
                num * shift
            },

            _ => {
                match right_string[0..4].parse::<i64>()
                {
                    Ok(num) => num,
                    Err(_) => return Err("[YololNumber::from_str] Failure to parse 4 decimals into number!".to_owned())
                }
            }
        };

        let yolol_num = YololNumber::from_split(left_num, right_num);
        Ok(yolol_num)
    }
}

impl std::fmt::Display for YololNumber
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        let main_digits: InnerType = self.into();
        let sign = self.0.signum();

        let ones = (self.0 % 10) * sign;
        let tens = ((self.0/10) % 10) * sign;
        let hundreds = ((self.0/100) % 10) * sign;
        let thousands = ((self.0/1000) % 10) * sign;

        let format = if ones != 0
        {
            format!("{}.{}{}{}{}", main_digits, thousands, hundreds, tens, ones)
        }
        else if tens != 0
        {
            format!("{}.{}{}{}", main_digits, thousands, hundreds, tens)
        }
        else if hundreds != 0
        {
            format!("{}.{}{}", main_digits, thousands, hundreds)
        }
        else if thousands != 0
        {
            format!("{}.{}", main_digits, thousands)
        }
        else
        {
            format!("{}", main_digits)
        };

        write!(f, "{}", format)
    }
}

impl std::fmt::Debug for YololNumber
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "{}", self)
    }
}

impl Serialize for YololNumber
{
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
    {
        serializer.serialize_str(&self.to_string())
    }
}

use serde::{Deserializer, de::Visitor};

struct YololNumberVisitor;

impl<'de> Visitor<'de> for YololNumberVisitor
{
    type Value = YololNumber;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "a string containing only numerical characters, possibly with a decimal point")
    }

    fn visit_str<E>(self, input: &str) -> Result<Self::Value, E>
    where E: serde::de::Error
    {
        match input.parse::<YololNumber>()
        {
            Ok(num) => Ok(num),
            Err(error) => Err(E::custom(error))
        }
    }
}

impl<'de> Deserialize<'de> for YololNumber
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de>
    {
        deserializer.deserialize_str(YololNumberVisitor)
    }
}

impl cmp::PartialEq for YololNumber
{
    fn eq(&self, other: &Self) -> bool
    {
        self.0 == other.0
    }
}

impl cmp::Eq for YololNumber {}

impl cmp::Ord for YololNumber
{
    fn cmp(&self, other: &YololNumber) -> cmp::Ordering
    {
        self.0.cmp(&other.0)
    }
}

impl cmp::PartialOrd for YololNumber
{
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering>
    {
        Some(self.cmp(other))
    }
}

impl ops::Add for YololNumber
{
    type Output =  Self;
    fn add(self, other: Self) -> Self
    {
        YololNumber::new(self.0.saturating_add(other.0))
    }
}

impl ops::Sub for YololNumber
{
    type Output = Self;
    fn sub(self, other: Self) -> Self
    {
        YololNumber::new(self.0.saturating_sub(other.0))
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl ops::Mul for YololNumber
{
    type Output = Self;
    fn mul(self, other: Self) -> Self
    {
        let output_sign = self.0.signum() * other.0.signum();
        let output = match self.0.checked_mul(other.0)
        {
            Some(num) => num / CONVERSION_CONST,
            None => if output_sign == -1 { std::i64::MIN } else { std::i64::MAX }
        };

        YololNumber::new(output)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl ops::Div for YololNumber
{
    type Output = Self;
    fn div(self, other: Self) -> Self
    {
        let output = (self.0 * CONVERSION_CONST).checked_div(other.0).unwrap_or(0);
        YololNumber::new(output)
    }
}

impl ops::Rem for YololNumber
{
    type Output = Self;
    fn rem(self, other: Self) -> Self
    {
        YololNumber::new(self.0.checked_rem(other.0).unwrap_or(0))
    }
}

impl std::ops::Add<String> for YololNumber
{
    type Output = String;
    fn add(self, other: String) -> String
    {
        self.to_string() + other.as_str()
    }
}

impl std::ops::Add<YololNumber> for String
{
    type Output = String;
    fn add(self, other: YololNumber) -> String
    {
        self + other.to_string().as_str()
    }
}

impl std::ops::Neg for YololNumber
{
    type Output = YololNumber;
    fn neg(self) -> YololNumber
    {
        YololNumber::new(self.0.saturating_neg())
    }
}

impl std::ops::Not for YololNumber
{
    type Output = YololNumber;
    fn not(self) -> YololNumber
    {
        if self.0 == 0
        {
            YololNumber::new(1 * CONVERSION_CONST)
        }
        else
        {
            YololNumber::new(0)
        }
    }
}

