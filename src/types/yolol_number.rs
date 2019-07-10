use std::ops;
use std::cmp;

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

    pub fn is_negative(&self) -> bool
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
        let adjustment = CONVERSION_CONST - first_decimal;

        // Then by adding that adjustment, we bring us to the next whole value
        YololNumber::new(self.0 + adjustment)
    }

    pub fn clamp(self, min: InnerType, max: InnerType) -> YololNumber
    {
        if self.0 < min * CONVERSION_CONST
        {
            YololNumber::from(min)
        }
        else if self.0 > max * CONVERSION_CONST
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

        let new_inner = (pow as i64).saturating_mul(CONVERSION_CONST);
        YololNumber::new(new_inner)
    }

    pub fn abs(self) -> YololNumber
    {
        YololNumber::new(self.0.abs())
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

impl std::fmt::Display for YololNumber
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        let main_digits: InnerType = self.into();
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
        let output = (self.0 * other.0) / CONVERSION_CONST;
        YololNumber::new(output)
    }
}

impl ops::Div for YololNumber
{
    type Output = Self;
    fn div(self, other: Self) -> Self
    {
        let output = (self.0 / other.0) * CONVERSION_CONST;
        YololNumber::new(output)
    }
}

impl ops::Rem for YololNumber
{
    type Output = Self;
    fn rem(self, other: Self) -> Self
    {
        YololNumber::new(self.0 % other.0)
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
        YololNumber::new(-self.0)
    }
}

impl std::ops::Not for YololNumber
{
    type Output = YololNumber;
    fn not(self) -> YololNumber
    {
        if self.0 == 0
        {
            YololNumber::new(0)
        }
        else
        {
            YololNumber::new(1 * CONVERSION_CONST)
        }
    }
}