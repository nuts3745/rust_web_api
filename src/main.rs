use std::{
    fmt::{self, Display},
    ops::Add,
};

#[derive(Debug, PartialEq, Eq)]
struct Fraction(u32, u32);

impl Fraction {
    fn new(numerator: u32, denominator: u32) -> Self {
        let gcf_value = Self::gcf(numerator, denominator);
        Self(numerator / gcf_value, denominator / gcf_value)
    }

    fn gcf(value1: u32, value2: u32) -> u32 {
        let (mut a, mut b) = if value1 > value2 {
            (value1, value2)
        } else {
            (value2, value1)
        };
        let mut r = a % b;
        while r != 0 {
            a = b;
            b = r;
            r = a % b;
        }
        b
    }
}

impl Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", &self.0, &self.1)
    }
}

impl Add for Fraction {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let lcm = self.1 / Self::gcf(self.1, rhs.1) * rhs.1;
        let a = self.0 * (lcm / self.1);
        let b = rhs.0 * (lcm / rhs.1);
        Fraction::new(a + b, lcm)
    }
}

fn main() {
    let a = Fraction::new(8, 18);
    let b = Fraction::new(1, 2);
    println!("{}", a + b);
}
