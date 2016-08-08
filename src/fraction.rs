use divisors::DivisorIterator;
use iter_ord::AscendingIntersection;
use std::error::Error;
use std::fmt;
use std::str;

#[derive(Debug, Eq)]
pub enum Fraction {
    Raw(i32, i32),
    Reduced(i32, i32),
}

impl Fraction {
    pub fn new(a: i32, b: i32) -> Fraction {
        Fraction::Raw(a, b)
    }

    pub fn reduce(&self) -> Fraction {
        match *self {
            Fraction::Reduced(a, b) => Fraction::Reduced(a, b),
            Fraction::Raw(a, b) => {
                let (a, b) = reduce(a, b);
                Fraction::Reduced(a, b)
            }
        }
    }

    pub fn numerator(&self) -> i32 {
        match *self {
            Fraction::Raw(n, _) | Fraction::Reduced(n, _) => n
        }
    }

    pub fn denominator(&self) -> i32 {
        match *self {
            Fraction::Raw(_, n) | Fraction::Reduced(_, n) => n
        }
    }
}

impl PartialEq for Fraction {
    fn eq(&self, other: &Fraction) -> bool {
        self.numerator() == other.numerator() && self.denominator() == other.denominator() 
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Fraction::Raw(a, b) | Fraction::Reduced(a, b) => write!(f, "{}/{}", a, b)
        }
    }
}

impl str::FromStr for Fraction {
    type Err = FractionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');

        let numerator: i32 = match parts.next().map(|n| n.parse()) {
            Some(Ok(n)) => n,
            Some(Err(e)) => return Err(FractionError::IntParse(box e)),
            None => return Err(FractionError::TooFewParts),
        };

        let denominator: i32 = match parts.next().map(|n| n.parse()) {
            Some(Ok(n)) => n,
            Some(Err(e)) => return Err(FractionError::IntParse(box e)),
            None => return Err(FractionError::TooFewParts),
        };

        if parts.next().is_some() {
            return Err(FractionError::TooManyParts);
        }

        Ok(Fraction::new(numerator, denominator))
    }
}

#[derive(Debug)]
pub enum FractionError {
    IntParse(Box<Error>),
    TooFewParts,
    TooManyParts,
}

impl fmt::Display for FractionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FractionError::IntParse(ref e) => write!(f, "{}", e),
            FractionError::TooManyParts => write!(f, "Too many parts"),
            FractionError::TooFewParts => write!(f, "Too few parts"),
        }
    }
}

fn reduce(a: i32, b: i32) -> (i32, i32) {
    let divisors_a = DivisorIterator::new(a);
    let divisors_b = DivisorIterator::new(b);
    let greatest_common_divisor = AscendingIntersection::new(divisors_a, divisors_b)
        .map(|(a, _)| a)
        .max()
        .unwrap_or(1);

    (a / greatest_common_divisor, b / greatest_common_divisor)
}

#[cfg(test)]
mod tests {
    use fraction::Fraction;

    #[test]
    fn reducible_fractions_are_reduced() {
        let fraction = Fraction::new(12, 16);

        assert_eq!(Fraction::new(3, 4), fraction.reduce());
    }

    #[test]
    fn irreducible_fractions_are_unchanged() {
        let fraction = Fraction::new(13, 16);

        assert_eq!(Fraction::new(13, 16), fraction.reduce());
    }
}
