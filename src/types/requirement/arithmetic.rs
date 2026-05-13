use std::{fmt, str::FromStr};

use winnow::Parser;

use crate::{
    impl_arithmetic_flag_traits,
    parsers::ParseError,
    parsers::parse_arithmetic_requirement,
    types::{flag::ArithmeticFlag, operator::ArithmeticOperator, value::TypedValue},
};

/// An arithmetic operation between two values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ArithmeticRequirement {
    pub flag: ArithmeticFlag,
    pub lhs: TypedValue,
    pub operation: Option<ArithmeticOperation>,
}

impl ArithmeticRequirement {
    /// Creates a new arithmetic requirement.
    pub fn new(flag: ArithmeticFlag, lhs: impl Into<TypedValue>) -> Self {
        Self {
            flag,
            lhs: lhs.into(),
            operation: None,
        }
    }

    /// Sets the arithmetic flag.
    pub fn with_flag(mut self, flag: ArithmeticFlag) -> Self {
        self.flag = flag;
        self
    }

    /// Sets the operation on this requirement.
    ///
    /// # Arguments
    ///
    /// * `operation` - The operation to perform.
    pub fn with_operation(mut self, operation: ArithmeticOperation) -> Self {
        self.operation = Some(operation);
        self
    }

    /// Adds with the given value.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right hand side of the operation.
    pub fn add(self, rhs: impl Into<TypedValue>) -> Self {
        self.with_operation(ArithmeticOperation::add(rhs))
    }

    /// Subtracts with the given value.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right hand side of the operation.
    pub fn sub(self, rhs: impl Into<TypedValue>) -> Self {
        self.with_operation(ArithmeticOperation::sub(rhs))
    }

    /// Multiplies with the given value.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right hand side of the operation.
    pub fn mul(self, rhs: impl Into<TypedValue>) -> Self {
        self.with_operation(ArithmeticOperation::mul(rhs))
    }

    /// Divides with the given value.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right hand side of the operation.
    pub fn div(self, rhs: impl Into<TypedValue>) -> Self {
        self.with_operation(ArithmeticOperation::div(rhs))
    }

    /// Modulos with the given value.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right hand side of the operation.
    pub fn modulo(self, rhs: impl Into<TypedValue>) -> Self {
        self.with_operation(ArithmeticOperation::modulo(rhs))
    }

    /// Bitwise ands with the given value.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right hand side of the operation.
    pub fn bitwise_and(self, rhs: impl Into<TypedValue>) -> Self {
        self.with_operation(ArithmeticOperation::bitwise_and(rhs))
    }

    /// Bitwise xors with the given value.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right hand side of the operation.
    pub fn bitwise_xor(self, rhs: impl Into<TypedValue>) -> Self {
        self.with_operation(ArithmeticOperation::bitwise_xor(rhs))
    }
}

impl FromStr for ArithmeticRequirement {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_arithmetic_requirement
            .parse(s)
            .map_err(|s| ParseError::InvalidRequirement(s.to_string()))
    }
}

impl fmt::Display for ArithmeticRequirement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let operation = self.operation.map(|o| o.to_string()).unwrap_or_default();
        write!(f, "{}{}{}", self.flag, self.lhs, operation)
    }
}

/// An operation in an arithmetic expression.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ArithmeticOperation {
    pub operator: ArithmeticOperator,
    pub rhs: TypedValue,
}

impl ArithmeticOperation {
    pub fn new(operator: ArithmeticOperator, rhs: impl Into<TypedValue>) -> Self {
        Self {
            operator,
            rhs: rhs.into(),
        }
    }

    /// Creates a new add operation.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right hand side of the operation.
    pub fn add(rhs: impl Into<TypedValue>) -> Self {
        Self::new(ArithmeticOperator::Add, rhs)
    }

    /// Creates a new subtract operation.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right hand side of the operation.
    pub fn sub(rhs: impl Into<TypedValue>) -> Self {
        Self::new(ArithmeticOperator::Subtract, rhs)
    }

    /// Creates a new multiply operation.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right hand side of the operation.
    pub fn mul(rhs: impl Into<TypedValue>) -> Self {
        Self::new(ArithmeticOperator::Multiply, rhs)
    }

    /// Creates a new divide operation.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right hand side of the operation.
    pub fn div(rhs: impl Into<TypedValue>) -> Self {
        Self::new(ArithmeticOperator::Divide, rhs)
    }

    /// Creates a new modulo operation.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right hand side of the operation.
    pub fn modulo(rhs: impl Into<TypedValue>) -> Self {
        Self::new(ArithmeticOperator::Modulo, rhs)
    }

    /// Creates a new bitwise and operation.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right hand side of the operation.
    pub fn bitwise_and(rhs: impl Into<TypedValue>) -> Self {
        Self::new(ArithmeticOperator::BitwiseAnd, rhs)
    }

    /// Creates a new bitwise xor operation.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right hand side of the operation.
    pub fn bitwise_xor(rhs: impl Into<TypedValue>) -> Self {
        Self::new(ArithmeticOperator::BitwiseXor, rhs)
    }
}

impl fmt::Display for ArithmeticOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.operator, self.rhs)
    }
}

impl_arithmetic_flag_traits!(ArithmeticRequirement, with_flag);
