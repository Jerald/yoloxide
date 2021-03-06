use std::convert::TryInto;
use std::iter::FromIterator;

pub trait SlidingWindow
{
    type Value;

    /// Gets the value at the index relative to the window view
    fn get_value(&self, index: usize) -> Option<&Self::Value>;
    /// Gets a window view of the specified size
    fn get_window(&self, view_size: usize) -> Option<&[Self::Value]>;
    /// Returns how many elements there are remaining, relative to the window view
    fn remaining_length(&self) -> usize;

    /// Moves the window view by the specified distance, may be negative.
    /// Returns the new index.
    fn move_view(&mut self, distance: isize) -> usize;
}

pub struct VecWindow<T>
{
    vector: Vec<T>,
    index: usize,
}

impl<T> VecWindow<T>
{
    pub fn new(vector: Vec<T>, starting_index: usize) -> VecWindow<T>
    {
        VecWindow {
            vector,
            index: starting_index,
        }
    }
}

impl<T> SlidingWindow for VecWindow<T>
{
    type Value = T;

    /// Gets the value at the index relative to the current window view
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

    /// Gets a window view of the specified size
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

    /// Moves the window view by the specified distance, may be negative.
    /// Returns the new index.
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
                panic!("[SlidingWindow::move_view] Trying to move view outside it's bounds! Check with window.remaining_length() before moving!");
            }

            self.index += distance_magnitude;
        }
        else if distance_sign == -1
        {
            if distance_magnitude > self.index
            {
                panic!("[SlidingWindow::move_view] Trying to move view outside it's bounds! Check with window.remaining_length() before moving!");
            }
            
            self.index -= distance_magnitude;
        }

        self.index
    }

    /// Returns how many elements there are remaining, relative to the window view
    fn remaining_length(&self) -> usize
    {
        // The length minus the current view index gives the remaining elements
        // We _don't_ subtract an additional one since the index is the current value as well
        self.vector.len() - self.index
    }
}

impl<T> From<Vec<T>> for VecWindow<T>
{
    fn from(input: Vec<T>) -> Self
    {
        VecWindow::new(input, 0)
    }
}

// Since we can go from a Vec to a VecWindow,
// why not just skip a step and go from the iterator directly?
impl<T> FromIterator<T> for VecWindow<T>
{
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self
    {
        VecWindow::from(iter.into_iter().collect::<Vec<T>>())
    }
}