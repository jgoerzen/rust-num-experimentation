use std::fmt;
use std::ops;

/// Default Display in terms of debug
macro_rules! dispdebug {
    ($x:ty) => {
        impl fmt::Display for $x {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                fmt::Debug::fmt(self, f)
            }
        }
    }
}


/// The "operators" that we're going to support.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Op {Plus, Minus, Mul, Div, Pow, }
impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x = match self {
            &Op::Plus => "+",
            &Op::Minus => "-",
            &Op::Mul => "*",
            &Op::Div => "/",
            &Op::Pow => "**",
        };
        write!(f, "{}", x)
    }
}
               
/// The core symbolic manipulation type.  It can be a simple number,
/// a symbol, a binary arithmetic operation (such as +), or a unary
/// arithmetic operation (such as cos).
///
/// Notice the types of BinaryArith and UnaryArith: it's a
/// recursive type.  So, we could represent + over two SymbolicManips.
#[derive(Debug, PartialEq, Clone)]
pub enum SymbolicManip<T> {
    Number(T),                   /// A simple number, such as 5
    Symbol(String),              /// A symbol, such as "x"
    BinaryArith(Op, Box<SymbolicManip<T>>, Box<SymbolicManip<T>>),
    UnaryArith(String, Box<SymbolicManip<T>>)
}

impl<T: ops::Add> ops::Add for SymbolicManip<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        SymbolicManip::BinaryArith(Op::Plus, Box::new(self), Box::new(other))
    }
}

impl<T: ops::Sub> ops::Sub for SymbolicManip<T> {
    type Output = Self;

    fn sub(self, other:Self) -> Self {
        SymbolicManip::BinaryArith(Op::Minus, Box::new(self), Box::new(other))
    }
}

impl<T: ops::Mul> ops::Mul for SymbolicManip<T> {
    type Output = Self;

    fn mul(self, other:Self) -> Self {
        SymbolicManip::BinaryArith(Op::Mul, Box::new(self), Box::new(other))
    }
}

impl<T: ops::Div> ops::Div for SymbolicManip<T> {
    type Output = Self;

    fn div(self, other:Self) -> Self {
        SymbolicManip::BinaryArith(Op::Div, Box::new(self), Box::new(other))
    }
}

impl<T> From<T> for SymbolicManip<T> {
    fn from(other: T) -> Self {
        SymbolicManip::Number(other)
    }
}

impl<T: ops::Neg + ops::Mul + From<i32>> ops::Neg for SymbolicManip<T> {
    type Output = Self;

    fn neg(self) -> Self {
        let x: i32 = -1;
        self * SymbolicManip::Number(x.into())
    }
}

impl<T: Clone> SymbolicManip<T> {
    pub fn abs(self) -> Self {
        SymbolicManip::UnaryArith(String::from("abs"), Box::new(self))
    }
}

impl<T> SymbolicManip<T> {
    pub fn pi() -> Self {
        SymbolicManip::Symbol(String::from("pi"))
    }
    
    pub fn sqrt(self) -> Self {
        SymbolicManip::UnaryArith(String::from("sqrt"), Box::new(self))
    }

    pub fn pow(self, other: Self) -> Self {
        SymbolicManip::BinaryArith(Op::Pow, Box::new(self), Box::new(other))
    }

    pub fn new(other: T) -> Self {
        SymbolicManip::Number(other)
    }
}

impl<T: fmt::Display> SymbolicManip<T> {
    /// Show a SymbolicManip as a String, with conventional algebraic notation.
    pub fn pretty_show(&self) -> String {
        match self {
        &SymbolicManip::Number(ref x) => x.to_string(), // A number is a bare string
        &SymbolicManip::Symbol(ref x) => x.to_string(),
        &SymbolicManip::BinaryArith(op, ref a, ref b) => format!("{}{}{}",
                                                         a.simple_paren(), op, b.simple_paren()),
        &SymbolicManip::UnaryArith(ref op, ref a) => format!("{}({})", op, a)
        }
    }

    /// Add parenthesis where needed.  This function is fairly conservative
    /// and will add parenthesis when not needed in some cases.
   ///
    /// Rust will have already figured out precedence for us while building
    /// the SymbolicMnaip.
    pub fn simple_paren(&self) -> String {
        match self {
            &SymbolicManip::Number(ref x) => x.to_string(),
            &SymbolicManip::Symbol(ref x) => x.to_string(),
            &SymbolicManip::BinaryArith(_, _, _) => format!("({})", self.pretty_show()),
            x => x.pretty_show()
        }
    }

    /// Show a SymbolicManip using RPN.  HP calculator users may find this familiar.
    pub fn to_rpn(&self) -> String {
        match self {
            &SymbolicManip::Number(ref x) => x.to_string(),
            &SymbolicManip::Symbol(ref x) => x.to_string(),
            &SymbolicManip::BinaryArith(op, ref a, ref b) =>
                format!("{} {} {}", a.to_rpn(), b.to_rpn(), op),
            &SymbolicManip::UnaryArith(ref op, ref a) =>
                format!("{} {}", a.to_rpn(), op)
        }
    }
}

impl<T: fmt::Display + PartialEq + From<i32> + Clone> SymbolicManip<T> {
    pub fn simplify(&self) -> SymbolicManip<T> {
        let one = SymbolicManip::new(1i32.into());
        let zero = SymbolicManip::new(0i32.into());
        match self {
            &SymbolicManip::BinaryArith(op, ref origa, ref origb) => {
                let sa = origa.simplify();
                let sb = origb.simplify();
/* This match used to look like this.  There was no
particular reason to pull sa or sb into the match here.
I could never figure out how to please the borrow checker
while returning sa/sb (or a derivative) without cloning,
so I pulled it out.
                match (op, sa, sb) {
                    (Op::Mul, ref a, ref b) if *a == one => (*b).clone(),
                    (Op::Mul, ref a, ref b) if *b == one => (*a).clone(),
                    (Op::Mul, ref a, ref b) if *a == zero || *b == zero => zero,
                    (Op::Div, ref a, ref b) if *b == one => (*a).clone(),
                    (Op::Plus, ref a, ref b) if *a == zero => (*b).clone(),
                    (Op::Plus, ref a, ref b) if *b == zero => (*a).clone(),
                    (Op::Minus, ref a, ref b) if *b == zero => (*a).clone(),
                    (newop, newa, newb) => SymbolicManip::BinaryArith(op, Box::new(newa), Box::new(newb))
*/
                match op {
                    Op::Mul if sa == one => sb,
                    Op::Mul if sb == one => sa,
                    Op::Mul if sa == zero || sb == zero => zero,
                    Op::Div if sb == one => sa,
                    Op::Plus if sa == zero  => sb,
                    Op::Plus if sb == zero => sa,
                    Op::Minus if sb == zero => sa,
                    _ => SymbolicManip::BinaryArith(op, Box::new(sa), Box::new(sb))
                }
            },
            &SymbolicManip::UnaryArith(ref op, ref a) => SymbolicManip::UnaryArith(op.to_string(), Box::new(a.simplify())),
            _ => self.clone(),
        }
    }

}


impl<T: fmt::Display> fmt::Display for SymbolicManip<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.pretty_show())
    }
}

