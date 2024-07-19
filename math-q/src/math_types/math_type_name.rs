use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd)]
pub enum MathTypeName {
    Undefined,
    Function(String),
    NaturalNumber(u128),
    Sum,
    FlipSign,
    Product,
    Exp,
    LogN,
    Power,
}

impl Ord for MathTypeName {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.index() != other.index() {
            return self.index().cmp(&other.index());
        }

        return match (self.clone(), other.clone()) {
            (MathTypeName::Function(a), MathTypeName::Function(b)) => a.cmp(&b),
            (MathTypeName::NaturalNumber(a), MathTypeName::NaturalNumber(b)) => a.cmp(&b),
            _ => Ordering::Equal,
        };
    }
}

impl MathTypeName {
    pub fn parameter_range(&self) -> (usize, usize) {
        match self {
            MathTypeName::Sum => (0, usize::MAX),
            MathTypeName::Product => (0, usize::MAX),
            MathTypeName::Function(_) => (0, usize::MAX),
            
            MathTypeName::FlipSign => (1, 1),

            MathTypeName::Exp => (1, 1),
            MathTypeName::LogN => (1, 1),

            MathTypeName::Power => (2, 2),
            
            _ => (0, 0),
        }
    }

    pub fn precedence(&self) -> u8 {
        if self.parameter_range().1 == 0 || matches!(self, MathTypeName::Function(_)) {
            return u8::MAX;
        }

        match self {
            MathTypeName::Power => 4,
            MathTypeName::FlipSign => 3,
            MathTypeName::Product => 2,
            MathTypeName::Sum => 1,
            _ => 0,
        }
    }

    pub fn is_commutative(&self) -> bool {
        match self {
            MathTypeName::Sum => true,
            MathTypeName::Product => true,
            _ => false,
        }
    }

    pub fn index(&self) -> u8 {
        match self {
            MathTypeName::Undefined => 0,
            MathTypeName::NaturalNumber(_) => 1,
            MathTypeName::Function(_) => 2,
            MathTypeName::Power => 3,
            MathTypeName::Sum => 4,
            MathTypeName::FlipSign => 5,
            MathTypeName::Product => 6,
            MathTypeName::Exp => 7,
            MathTypeName::LogN => 8,
        }
    }
}