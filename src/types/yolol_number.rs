use std::ops;
use std::cmp;

const CONVERSION_CONST: u64 = 10000u64;

#[derive(Clone, Copy)]
pub struct YololNumber(u64);

impl YololNumber
{
    pub fn new(inner: u64) -> YololNumber
    {
        YololNumber(inner)
    }

    pub fn from_split(main: u64, decimal: u64) -> YololNumber
    {
        YololNumber(YololNumber::to_inner(main) + decimal)
    }

    fn to_inner(num: u64) -> u64
    {
        num * CONVERSION_CONST
    }

    fn from_inner(num: u64) -> u64
    {
        num / CONVERSION_CONST
    }
}

impl From<u64> for YololNumber
{
    fn from(input: u64) -> YololNumber
    {
        let num = YololNumber::to_inner(input);
        YololNumber(num)
    }
}

impl From<&u64> for YololNumber
{
    fn from(input: &u64) -> YololNumber
    {
        let num = YololNumber::to_inner(*input);
        YololNumber(num)
    }
}

impl From<YololNumber> for u64
{
    fn from(input: YololNumber) -> u64
    {
        YololNumber::from_inner(input.0)
    }
}

impl From<&YololNumber> for u64
{
    fn from(input: &YololNumber) -> u64
    {
        YololNumber::from_inner(input.0)
    }
}

impl std::fmt::Display for YololNumber
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        let main_digits: u64 = self.into();
        let ones = self.0%10;
        let tens = (self.0/10)%10;
        let hundreds = (self.0/100)%10;
        let thousands = (self.0/1000)%10;
        write!(f, "{}.{}{}{}{}", main_digits, thousands, hundreds, tens, ones)
    }
}

impl std::fmt::Debug for YololNumber
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "{}", self)
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
        YololNumber::new(self.0 + other.0)
    }
}

impl ops::Sub for YololNumber
{
    type Output = Self;
    fn sub(self, other: Self) -> Self
    {
        YololNumber::new(self.0 - other.0)
    }
}

impl ops::Mul for YololNumber
{
    type Output = Self;
    fn mul(self, other: Self) -> Self
    {
        YololNumber::new(self.0 * other.0)
    }
}

impl ops::Div for YololNumber
{
    type Output = Self;
    fn div(self, other: Self) -> Self
    {
        YololNumber::new(self.0 / other.0)
    }
}

impl ops::Div<u64> for YololNumber
{
    type Output = Self;
    fn div(self, other: u64) -> YololNumber
    {
        let lhs = YololNumber::from_inner(self.0);
        YololNumber::from(lhs / other)
    }
}