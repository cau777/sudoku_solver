use std::fmt::{Debug, Formatter};
use std::ops::{BitAnd, BitOr, Not};

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct NumberOptions {
    numbers: [bool; 9],
}

impl NumberOptions {
    pub fn has_number(&mut self, num: u8) -> bool {
        self.numbers[(num as usize) - 1]
    }

    pub fn set_number(&mut self, num: u8, state: bool) {
        self.numbers[(num as usize) - 1] = state
    }

    pub fn add_number(&mut self, num: u8) {
        self.set_number(num, true)
    }

    pub fn remove_number(&mut self, num: u8) {
        self.set_number(num, false)
    }

    pub fn all(&self) -> bool {
        self.numbers.iter().all(|x| *x)
    }

    pub fn count(&self) -> u8 {
        let mut result = 0_u8;
        for value in self.numbers {
            result += value as u8;
        }
        result
    }

    pub fn first(&self) -> Option<u8> {
        for (index, value) in self.numbers.into_iter().enumerate() {
            if value {
                return Some((index + 1) as u8);
            }
        }
        None
    }

    pub fn as_vec(&self) -> Vec<u8> {
        let mut result = Vec::<u8>::with_capacity(9);
        for (index, value) in self.numbers.into_iter().enumerate() {
            if value {
                result.push((index + 1) as u8);
            }
        }
        result
    }
}

impl Default for NumberOptions {
    fn default() -> Self {
        NumberOptions {
            numbers: [false; 9]
        }
    }
}

impl BitOr for NumberOptions {
    type Output = NumberOptions;

    fn bitor(self, rhs: Self) -> Self::Output {
        NumberOptions {
            numbers: [
                self.numbers[0] || rhs.numbers[0],
                self.numbers[1] || rhs.numbers[1],
                self.numbers[2] || rhs.numbers[2],
                self.numbers[3] || rhs.numbers[3],
                self.numbers[4] || rhs.numbers[4],
                self.numbers[5] || rhs.numbers[5],
                self.numbers[6] || rhs.numbers[6],
                self.numbers[7] || rhs.numbers[7],
                self.numbers[8] || rhs.numbers[8]]
        }
    }
}

impl BitAnd for NumberOptions {
    type Output = NumberOptions;

    fn bitand(self, rhs: Self) -> Self::Output {
        NumberOptions {
            numbers: [
                self.numbers[0] && rhs.numbers[0],
                self.numbers[1] && rhs.numbers[1],
                self.numbers[2] && rhs.numbers[2],
                self.numbers[3] && rhs.numbers[3],
                self.numbers[4] && rhs.numbers[4],
                self.numbers[5] && rhs.numbers[5],
                self.numbers[6] && rhs.numbers[6],
                self.numbers[7] && rhs.numbers[7],
                self.numbers[8] && rhs.numbers[8]]
        }
    }
}

impl Not for NumberOptions {
    type Output = NumberOptions;

    fn not(self) -> Self::Output {
        NumberOptions {
            numbers: self.numbers.map(|x| !x)
        }
    }
}

impl Debug for NumberOptions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ ")?;

        for (index, value) in self.numbers.iter().enumerate() {
            if *value {
                write!(f, "{} ", index + 1)?;
            }
        }

        write!(f, "]")?;

        Ok(())
    }
}
