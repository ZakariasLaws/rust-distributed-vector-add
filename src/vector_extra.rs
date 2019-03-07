use std::ops::{Add};

#[derive(Debug, PartialEq)]
pub struct Evec<T> {
    pub vec: Vec<T>,
}

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
    pub fn new() -> Evec<T>{
        Evec { vec: Vec::new()}
    }

    pub fn size(&self) -> usize{
        self.vec.len()
    }

    pub fn smaller(&self, other: &Evec<T>) -> bool{
        if self.size() < other.size() {
            return true;
        }

        return false;
    }
}