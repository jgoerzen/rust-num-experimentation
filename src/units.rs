use std::fmt;
use std::ops;
use std::ops::Neg;

use crate::num::*;

/// New data type: Units.  A Units type contains a number
/// and a SymbolicManip, which represents the units of measure.
/// A simple label would be something like (Symbol("m")).
#[derive(Debug, PartialEq, Clone)]
pub struct Units<T> {
    number: T,
    unit: SymbolicManip<T>
}

impl<T: ops::Add<Output = T> + PartialEq + fmt::Debug> ops::Add for Units<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.unit != other.unit {
            panic!("Mismatched units in add: {:?} vs {:?}", self.unit, other.unit);
        }
        let x: T = self.number + other.number;
        Units {number: x, ..self}
    }
}

impl<T: ops::Add<Output = T> + ops::Sub<Output = T> + ops::Neg<Output = T> + PartialEq + fmt::Debug> ops::Sub for Units<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + other.neg()
    }
}

impl<T: ops::Mul<Output = T>> ops::Mul for Units<T> {
    type Output = Self;

    fn mul(self, other:Self) -> Self {
        Units { number: self.number * other.number,
                unit: self.unit * other.unit }
    }
}

impl<T: ops::Neg<Output = T>> ops::Neg for Units<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Units{number: self.number.neg(), ..self}
    }
}

impl<T: ops::Div<Output = T>> ops::Div for Units<T> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Units {number: self.number / other.number,
               unit: self.unit / other.unit}
    }
}

impl<T: fmt::Display> fmt::Display for Units<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}_{}", self.number, self.unit)
    }
}

impl<T: Clone> Units<T> {
    pub fn new(num: T, u: String) -> Self {
        Units {number: num, unit: SymbolicManip::Symbol(u)}
    }

    pub fn drop_units(&self) -> T {
        self.number.clone()
    }
}

