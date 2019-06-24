use std::convert::TryInto;

mod yolol_number;
pub use yolol_number::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Token
{
    // Basic tokens
    Comment(String),
    AlphaNumToken(String),
    NumToken(String),
    Quote,
    Equal,
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Period,
    LAngleBrak,
    RAngleBrak,
    Exclam,
    Caret,
    Percent,
    Colon,
    Newline,

    // Extended tokens
    YololNum(YololNumber),
    PlusPlus,
    MinusMinus,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    PercentEqual,
    LAngleBrakEqual,
    RAngleBrakEqual,
    ExclamEqual,
    EqualEqual,
    Abs,
    Sqrt,
    Sin,
    Cos,
    Tan,
    Arcsin,
    Arccos,
    Arctan,
    Goto,
    If,
    End,
    Then
}

pub trait SlidingWindow<'a>
{
    type Value;

    // Gets the value at the index relative to the window view
    fn get_value(&self, index: usize) -> Option<&Self::Value>;
    // Gets a window view of the specified size
    fn get_window(&self, view_size: usize) -> Option<&[Self::Value]>;
    // Returns how many elements there are remaining, relative to the window view
    fn remaining_length(&self) -> usize;

    // Moves the window view by the specified distance, may be negative
    // Returns the new index
    fn move_view(&mut self, distance: isize) -> usize;
}

pub struct VecWindow<'a, T>
{
    vector: &'a Vec<T>,
    index: usize,
}

impl<'a, T> VecWindow<'a, T>
{
    pub fn new(vector: &'a Vec<T>, starting_index: usize) -> VecWindow<T>
    {
        VecWindow {
            vector,
            index: starting_index,
        }
    }
}

impl<'a, T> SlidingWindow<'a> for VecWindow<'a, T>
{
    type Value = T;

    // Gets the value at the index relative to the current window view
    fn get_value(&self, index: usize) -> Option<&Self::Value>
    {
        // Ensure we won't try to index outside the bounds of our vector
        if self.index + index >= self.vector.len()
        {
            return None;
        }
        
        let value = &self.vector[self.index + index];
        Some(value)
    }

    // Gets a window view of the specified size
    fn get_window(&self, view_size: usize) -> Option<&[Self::Value]>
    {
        let start = self.index;
        let end = start + view_size;

        // Ensure we don't try to slice outside the bounds of our vector
        if end > self.vector.len()
        {
            return None;
        }

        // Returns the requested view as a slice
        Some(&self.vector[start..end])
    }

    // Moves the window view by the specified distance, may be negative
    // Returns the new index
    fn move_view(&mut self, distance: isize) -> usize
    {
        let distance_magnitude: usize = distance.abs().try_into().unwrap();
        let distance_sign = distance.signum();

        // Move the index based on if the distance was positive or negative
        // If it's 0, we don't have to do anything, so no arm is needed
        if distance_sign == 1
        {
            if distance_magnitude > self.remaining_length()
            {
                panic!("Trying to move view outside it's bounds! Check with window.remaining_length() before moving!");
            }

            self.index += distance_magnitude;
        }
        else if distance_sign == -1
        {
            if distance_magnitude > self.index
            {
                panic!("Trying to move view outside it's bounds! Check with window.remaining_length() before moving!");
            }
            
            self.index -= distance_magnitude;
        }

        self.index
    }

    // Returns how many elements there are remaining, relative to the window view
    fn remaining_length(&self) -> usize
    {
        // The length minus the current view index gives the remaining elements
        // We _don't_ subtract an additional one since the index is the current value as well
        self.vector.len() - self.index
    }
}