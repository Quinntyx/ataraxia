use gc::{Finalize, Trace};
use rug::Integer as BigInt;

#[derive(Trace, Finalize, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fraction {
    #[unsafe_ignore_trace]
    pub numerator: BigInt,
    #[unsafe_ignore_trace]
    pub denominator: BigInt,
}

impl std::ops::Add for Fraction {
    type Output = Fraction;
    fn add(self, rhs: Self) -> Self::Output {
        if self.denominator == rhs.denominator {
            Fraction {
                numerator: self.numerator.clone() + rhs.numerator.clone(),
                denominator: self.denominator.clone(),
            }
        } else {
            Fraction {
                numerator: self.numerator.clone() * rhs.denominator.clone() + rhs.numerator.clone() * self.denominator.clone(),
                denominator: self.denominator.clone() * rhs.denominator.clone(),
            }
        }
    }
}