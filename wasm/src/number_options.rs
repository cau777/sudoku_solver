use std::fmt::{Debug, Formatter};
use std::ops::{BitAnd, BitOr, BitOrAssign, Not};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct NumberOptions<const SIZE: usize> {
    data: u16,
}

impl<const SIZE: usize> NumberOptions<SIZE> {
    const U8SIZE: u8 = SIZE as u8;

    pub fn has_number(&self, num: u8) -> bool {
        (self.data >> (num - 1)) & 1 == 1
    }

    pub fn add_number(&mut self, num: u8) {
        self.data |= 1 << (num - 1);
    }

    pub fn remove_number(&mut self, num: u8) {
        self.data &= !(1 << (num - 1));
    }

    pub fn empty(&self) -> bool { self.data == 0 }

    pub fn all(&self) -> bool {
        self.as_vec().len() == SIZE
    }

    pub fn count(&self) -> u16 {
        let mut result = 0_u16;
        for value in 0..Self::U8SIZE {
            result += (self.data >> value) & 1;
        }
        result
    }

    pub fn first(&self) -> Option<u8> {
        for i in 1..=Self::U8SIZE {
            if self.has_number(i) {
                return Some(i);
            }
        }

        None
    }

    pub fn as_vec(&self) -> Vec<u8> {
        let mut result = Vec::<u8>::with_capacity(SIZE);

        for i in 1..=Self::U8SIZE {
            if self.has_number(i) {
                result.push(i)
            }
        }
        result
    }

    pub fn as_bool_array(&self) -> [bool; SIZE] {
        let mut result = [false; SIZE];
        for i in 0..SIZE {
            result[i] = self.has_number(i as u8 + 1);
        }
        result
    }
}

impl<const SIZE: usize> Default for NumberOptions<SIZE> {
    fn default() -> Self {
        NumberOptions {
            data: 0
        }
    }
}

impl<const SIZE: usize> BitOr for NumberOptions<SIZE> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        NumberOptions {
            data: self.data | rhs.data
        }
    }
}

impl<const SIZE: usize> BitOrAssign for NumberOptions<SIZE> {
    fn bitor_assign(&mut self, rhs: Self) {
        self.data |= rhs.data;
    }
}

impl<const SIZE: usize> BitAnd for NumberOptions<SIZE> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        NumberOptions {
            data: self.data & rhs.data
        }
    }
}

impl<const SIZE: usize> Not for NumberOptions<SIZE> {
    type Output = Self;

    fn not(self) -> Self::Output {
        NumberOptions {
            data: !self.data
        }
    }
}

impl<const SIZE: usize> Debug for NumberOptions<SIZE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ ")?;

        for (index, value) in self.as_bool_array().iter().enumerate() {
            if *value {
                write!(f, "{} ", index + 1)?;
            }
        }

        write!(f, "]")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::number_options::NumberOptions;

    #[test]
    fn util() {
        let mut options = NumberOptions::<9>::default();
        println!("{:?}", options);

        options.add_number(1);
        options.add_number(9);
        println!("{:?}", options);

        options.remove_number(9);
        options.remove_number(2);
        println!("{:?}", options);
    }
}