mod num;
mod units;
use crate::num::*;
use crate::units::*;
use std::ops;
use std::fmt;
use numeric_literals::replace_numeric_literals;

/// This function will, depending on calling context, yield things such as
/// ints or floats or SymbolicManips.
#[replace_numeric_literals(T::from(literal))]
fn exp_a<T>() -> T
    where T: ops::Mul<Output = T> + ops::Div<Output = T> + ops::Add<Output = T> + From<i32>
{
    (5 * 3 + 10 * 20 * 1 + 0) / 2
}

// A little shortcut for creating a Unit
fn c<U: Clone>(num: U, unit: &str) -> Units<U> {
    Units::new(num, String::from(unit))
}

/// Similar to exp_a, but this time with units.
#[replace_numeric_literals(T::from(literal))]
fn exp_b<T>() -> Units<T>
    where T: ops::Add<Output = T> + Clone + ops::Div<Output = T> + From<f64> + PartialEq<T> + fmt::Debug
{
    (c(96.0, "m") + c(2.0, "m")) / c(10.0, "s")
}

fn main() {
    let a_sym: SymbolicManip<i32> = exp_a();
    let a_num: i32 = exp_a();
    let a_float: f64 = exp_a();
    println!("a = {:?}", a_sym);
    println!("a nice = {}", a_sym);
    println!("a rpn = {}", a_sym.to_rpn());
    println!("a_num = {}", a_num);
    println!("a_float = {}", a_float);
    println!("AFTER SIMPLIFICATION:");
    println!("a = {}", a_sym.simplify());
    println!("a rpn = {}", a_sym.simplify().to_rpn());
    println!("UNITS:");
    let b_num: Units<f64> = exp_b();
    let b_sym: Units<SymbolicManip<f64>> = exp_b();
    println!("b_num = {:?}", b_num);
    println!("b_num nice = {}", b_num);
    println!("b_sym = {:?}", b_sym);
    println!("b_sym nice = {}", b_sym);
}
