use std::ops::{Add};

/// Structure holding a vector with any type of elements.
#[derive(Debug, PartialEq)]
pub struct Evec<T> {
    pub vec: Vec<T>,
}

/// Add two Evec structures together by internally performing a
/// vector addition
///
/// '''
/// # Examples
///
/// let mut vec1: Evec<i32> = Evec<i32>;
/// let mut vec2: Evec<i32> = Evec::new();
///
/// vec1.vec.push(1);
/// vec2.vec.push(2);
///
/// let mut vec3 = vec1 + vec2;
///
/// assert_eq!(vec3, [3]);
/// '''
impl<T> Add<Evec<T>> for Evec<T>
    where T: Add<Output=T> + Copy {

    type Output = Evec<T>;

    fn add(self, other: Evec<T>) -> Evec<T> where T: Copy {
        let shortest: usize;

        if self.vec.len() < other.vec.len() {
            shortest = self.vec.len();
        } else {
            shortest = other.vec.len();
        }

        let mut new_vec = Evec::new();

        for x in 0..shortest {
            let value= self.vec[x] + other.vec[x];
            new_vec.vec.push(value);
        }

        return new_vec;
    }
}

impl <T> Evec<T>{
    /// Generate a new Evec<T>
    pub fn new() -> Evec<T>{
        Evec { vec: Vec::new()}
    }

    /// Get the length of the internal vector
    pub fn size(&self) -> usize{
        self.vec.len()
    }

    /// Compare to another Evec 'other' to check if internal
    /// vector is smaller than the one from 'other'
    pub fn smaller(&self, other: &Evec<T>) -> bool{
        if self.size() < other.size() {
            return true;
        }

        return false;
    }

    /// Get the length of the inner vector
    pub fn len(self) -> usize {
        return self.vec.len();
    }
}