use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd)]
pub enum MathTypeName {
    Undefined,
    Variable(String),
    NaturalNumber(u128),
    Sum,
    FlipSign,
    Product,
    Reciprocal,
}

impl Ord for MathTypeName {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.index() != other.index() {
            return self.index().cmp(&other.index());
        }

        return match (self.clone(), other.clone()) {
            (MathTypeName::Variable(a), MathTypeName::Variable(b)) => a.cmp(&b),
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
            
            MathTypeName::FlipSign => (1, 1),
            MathTypeName::Reciprocal => (1, 1),
            
            _ => (0, 0),
        }
    }

    pub fn precedence(&self) -> u8 {
        if self.parameter_range().1 == 0 {
            return u8::MAX;
        }

        match self {
            MathTypeName::Reciprocal => 4,
            MathTypeName::FlipSign => 3,
            MathTypeName::Product => 2,
            MathTypeName::Sum => 1,
            _ => 0,
        }
    }

    pub fn index(&self) -> u8 {
        match self {
            MathTypeName::Undefined => 0,
            MathTypeName::Variable(_) => 2,
            MathTypeName::NaturalNumber(_) => 1,
            MathTypeName::Sum => 3,
            MathTypeName::FlipSign => 4,
            MathTypeName::Product => 5,
            MathTypeName::Reciprocal => 6,
        }
    }
}