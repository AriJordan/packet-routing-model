use std::cmp::Ordering;
use std::{cmp, fmt};
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Fraction {
    pub numerator: i64,
    pub denominator: i64,
}

impl Fraction {
    /// Create a new (non-negative) fraction with the given numerator and denominator
    /// Panic if fraction illegal
    pub fn new(numerator: i64, denominator: i64) -> Self {
        assert!(numerator >= 0, "Error: Fraction numerator must be non-negative");
        assert!(denominator > 0, "Error: Fraction denominator must be positiove");
        Self { numerator, denominator }
    }

    /// Return a new Fraction that is equal to this one, but simplified
    pub fn reduce(&self) -> Self {
        let gcd = gcd(self.numerator, self.denominator);
        Self {
            numerator: (self.numerator / gcd),
            denominator: (self.denominator / gcd),
        }
    }

    // Return the Fraction rounded down to the nearest integer
    pub fn floor(&self) -> Self {
        return Fraction{numerator : self.numerator / self.denominator, denominator : 1};
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let temp = self.reduce();
        if temp.denominator == 1 {
            write!(f, "{}", temp.numerator)
        } else {
            write!(f, "{}/{}", temp.numerator, temp.denominator)
        }
    }
}

impl cmp::PartialEq for Fraction {
    fn eq(&self, other: &Fraction) -> bool {
        let simp_self = self.reduce();
        let simp_other = other.reduce();
        simp_self.numerator == simp_other.numerator &&
            simp_self.denominator == simp_other.denominator
    }
}

// inherits PartialEq
impl cmp::Eq for Fraction {}

impl cmp::PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Fraction) -> Option<cmp::Ordering> {
        assert!(self.numerator < i32::MAX as i64 && self.denominator < i32::MAX as i64);
        assert!(other.numerator < i32::MAX as i64 && other.denominator < i32::MAX as i64);
        if self.eq(other){
            return Some(Ordering::Equal);
        }
        else{
            match self.numerator * other.denominator < other.numerator * self.denominator{
                true => Some(Ordering::Less),
                false => Some(Ordering::Greater),
            }
        }
    }
}

impl cmp::Ord for Fraction {
    fn cmp(&self, other: &Fraction) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Add for Fraction {
    type Output = Fraction;

    fn add(self, other: Fraction) -> Fraction {
        assert!(self.numerator < i32::MAX as i64 && self.denominator < i32::MAX as i64);
        assert!(other.numerator < i32::MAX as i64 && other.denominator < i32::MAX as i64);
        Fraction {
            numerator: (self.numerator * other.denominator + other.numerator * self.denominator),
            denominator: (self.denominator * other.denominator),
        }.reduce()
    }
}

impl Sub for Fraction {
    type Output = Fraction;

    fn sub(self, other: Fraction) -> Fraction {
        assert!(self.numerator < i32::MAX as i64 && self.denominator < i32::MAX as i64);
        assert!(other.numerator < i32::MAX as i64 && other.denominator < i32::MAX as i64);
        Fraction {
            numerator: (self.numerator * other.denominator - other.numerator * self.denominator),
            denominator: (self.denominator * other.denominator),
        }.reduce()
    }
}

// Calculate the greatest common denominator for two numbers
pub fn gcd(a: i64, b: i64) -> i64 {
    if b == 0{
        return a;
    }
    else{
        return gcd(b, a % b);
    }
}

#[test]
fn ordering_test() {
    let a = Fraction::new(1, 2);
    let b = Fraction::new(3, 4);
    let c = Fraction::new(4, 3);
    assert!(a < b);
    assert!(a <= b);
    assert!(c > b);
    assert!(c >= a);
}

#[test]
fn equality_test() {
    let a = Fraction::new(1, 2);
    let b = Fraction::new(2, 4);
    let c = Fraction::new(5, 5);
    assert!(a == b);
    assert!(a != c);
}

#[test]
fn arithmetic_test() {
    let a = Fraction::new(1, 2);
    let b = Fraction::new(3, 4);
    let c = Fraction::new(1, 3);
    assert_eq!(a + a, Fraction::new(1, 1));
    assert_eq!(a - a, Fraction::new(0, 5));
    assert_eq!(a + b, Fraction::new(5, 4));
    assert_eq!(b - a, Fraction::new(1, 4));
    assert_eq!(a + c, Fraction::new(5, 6));
    assert_eq!(a - c, Fraction::new(1, 6));
}