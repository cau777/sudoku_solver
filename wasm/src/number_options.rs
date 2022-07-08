use std::fmt::{Debug, Formatter};
use std::ops::{BitOr, Not};

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct NumberOptions {
    data: u16,
}

impl NumberOptions {
    pub fn has_number(&self, num: u8) -> bool {
        (self.data >> (num - 1)) & 1 == 1
    }

    pub fn add_number(&mut self, num: u8) {
        self.data |= 1 << (num - 1);
    }

    pub fn remove_number(&mut self, num: u8) {
        self.data &= !(1 << (num - 1));
    }

    pub fn all(&self) -> bool {
        self.data == 0b11_1111_1111
    }

    pub fn count(&self) -> u16 {
        let mut result = 0_u16;
        for value in 0..9_u8 {
            result += (self.data >> value) & 1;
        }
        result
    }

    pub fn first(&self) -> Option<u8> {
        for i in 1..=9_u8 {
            if self.has_number(i) {
                return Some(i);
            }
        }

        None
    }

    pub fn as_vec(&self) -> Vec<u8> {
        let mut result = Vec::<u8>::with_capacity(9);

        for i in 1..=9_u8 {
            if self.has_number(i) {
                result.push(i)
            }
        }
        result
    }

    pub fn as_bool_array(&self) -> [bool; 9] {
        let mut result = [false; 9];
        for i in 0..9 {
            result[i] = self.has_number(i as u8 + 1);
        }
        result
    }
}

impl Default for NumberOptions {
    fn default() -> Self {
        NumberOptions {
            data: 0
        }
    }
}

impl BitOr for NumberOptions {
    type Output = NumberOptions;

    fn bitor(self, rhs: Self) -> Self::Output {
        NumberOptions {
            data: self.data | rhs.data
        }
    }
}

impl Not for NumberOptions {
    type Output = NumberOptions;

    fn not(self) -> Self::Output {
        NumberOptions {
            data: !self.data
        }
    }
}

impl Debug for NumberOptions {
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
        let mut options = NumberOptions::default();
        println!("{:?}", options);

        options.add_number(1);
        options.add_number(9);
        println!("{:?}", options);

        options.remove_number(9);
        options.remove_number(2);
        println!("{:?}", options);
    }
}