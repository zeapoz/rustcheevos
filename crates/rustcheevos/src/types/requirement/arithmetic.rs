//! Type definition for arithmetic conditions.

use std::{fmt, str::FromStr};

use winnow::Parser;

use crate::{
    impl_arithmetic_flag_traits,
    parsers::ParseError,
    parsers::parse_arithmetic,
    types::{
        flag::ArithmeticFlag, memory::AccessMode, operator::ArithmeticOperator, value::TypedValue,
    },
};

use crate::types::memory::AccessModeModifier;

/// An arithmetic operation between two values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Arithmetic {
    /// The flag of the arithmetic operation.
    flag: ArithmeticFlag,
    /// The left hand side of the arithmetic requirement.
    lhs: TypedValue,
    /// The operation of the arithmetic requirement, containing the operator and right hand side.
    operation: Option<ArithmeticOperation>,
}

impl Arithmetic {
    /// Creates a new arithmetic requirement.
    pub(crate) fn new(
        flag: ArithmeticFlag,
        lhs: impl Into<TypedValue>,
        operation: impl Into<Option<ArithmeticOperation>>,
    ) -> Self {
        Self {
            flag,
            lhs: lhs.into(),
            operation: operation.into(),
        }
    }

    /// Returns the arithmetic flag.
    #[must_use]
    pub fn flag(&self) -> ArithmeticFlag {
        self.flag
    }

    /// Returns the left hand side of the arithmetic requirement.
    #[must_use]
    pub fn lhs(&self) -> &TypedValue {
        &self.lhs
    }

    /// Returns the right hand side of the arithmetic requirement.
    #[must_use]
    pub fn rhs(&self) -> Option<TypedValue> {
        self.operation.map(|o| o.rhs)
    }

    /// Sets the arithmetic flag on this requirement.
    #[must_use]
    pub fn with_flag(mut self, flag: ArithmeticFlag) -> Self {
        self.flag = flag;
        self
    }

    /// Sets the operation on this requirement.
    #[must_use]
    fn with_operation(mut self, operation: ArithmeticOperation) -> Self {
        self.operation = Some(operation);
        self
    }

    /// Adds with the given value.
    #[expect(
        clippy::should_implement_trait,
        reason = "not using arithmetic in the traditional sense"
    )]
    #[must_use]
    pub fn add(self, rhs: impl Into<TypedValue>) -> Self {
        self.with_operation(ArithmeticOperation::add(rhs))
    }

    /// Subtracts with the given value.
    #[expect(
        clippy::should_implement_trait,
        reason = "not using arithmetic in the traditional sense"
    )]
    #[must_use]
    pub fn sub(self, rhs: impl Into<TypedValue>) -> Self {
        self.with_operation(ArithmeticOperation::sub(rhs))
    }

    /// Multiplies with the given value.
    #[expect(
        clippy::should_implement_trait,
        reason = "not using arithmetic in the traditional sense"
    )]
    #[must_use]
    pub fn mul(self, rhs: impl Into<TypedValue>) -> Self {
        self.with_operation(ArithmeticOperation::mul(rhs))
    }

    /// Divides with the given value.
    #[expect(
        clippy::should_implement_trait,
        reason = "not using arithmetic in the traditional sense"
    )]
    #[must_use]
    pub fn div(self, rhs: impl Into<TypedValue>) -> Self {
        self.with_operation(ArithmeticOperation::div(rhs))
    }

    /// Modulos with the given value.
    #[must_use]
    pub fn modulo(self, rhs: impl Into<TypedValue>) -> Self {
        self.with_operation(ArithmeticOperation::modulo(rhs))
    }

    /// Bitwise ands with the given value.
    #[must_use]
    pub fn bitwise_and(self, rhs: impl Into<TypedValue>) -> Self {
        self.with_operation(ArithmeticOperation::bitwise_and(rhs))
    }

    /// Bitwise xors with the given value.
    #[must_use]
    pub fn bitwise_xor(self, rhs: impl Into<TypedValue>) -> Self {
        self.with_operation(ArithmeticOperation::bitwise_xor(rhs))
    }
}

impl AccessModeModifier for Arithmetic {
    fn with_access_mode(mut self, access_mode: AccessMode) -> Self {
        self.lhs = self.lhs.with_access_mode(access_mode);
        if let Some(op) = self.operation {
            let rhs = op.rhs.with_access_mode(access_mode);
            self.operation = Some(ArithmeticOperation { rhs, ..op });
        }
        self
    }
}

impl FromStr for Arithmetic {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_arithmetic
            .parse(s)
            .map_err(|s| ParseError::Condition(s.to_string()))
    }
}

impl fmt::Display for Arithmetic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let operation = self.operation.map(|o| o.to_string()).unwrap_or_default();
        write!(f, "{}{}{}", self.flag, self.lhs, operation)
    }
}

/// An operation in an arithmetic expression.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct ArithmeticOperation {
    /// The operator of the arithmetic operation.
    operator: ArithmeticOperator,
    /// The right hand side of the arithmetic operation.
    rhs: TypedValue,
}

impl ArithmeticOperation {
    /// Creates a new arithmetic operation.
    pub(crate) fn new(operator: ArithmeticOperator, rhs: impl Into<TypedValue>) -> Self {
        Self {
            operator,
            rhs: rhs.into(),
        }
    }

    /// Creates a new add operation.
    pub(crate) fn add(rhs: impl Into<TypedValue>) -> Self {
        Self::new(ArithmeticOperator::Add, rhs)
    }

    /// Creates a new subtract operation.
    pub(crate) fn sub(rhs: impl Into<TypedValue>) -> Self {
        Self::new(ArithmeticOperator::Subtract, rhs)
    }

    /// Creates a new multiply operation.
    pub(crate) fn mul(rhs: impl Into<TypedValue>) -> Self {
        Self::new(ArithmeticOperator::Multiply, rhs)
    }

    /// Creates a new divide operation.
    pub(crate) fn div(rhs: impl Into<TypedValue>) -> Self {
        Self::new(ArithmeticOperator::Divide, rhs)
    }

    /// Creates a new modulo operation.
    pub(crate) fn modulo(rhs: impl Into<TypedValue>) -> Self {
        Self::new(ArithmeticOperator::Modulo, rhs)
    }

    /// Creates a new bitwise and operation.
    pub(crate) fn bitwise_and(rhs: impl Into<TypedValue>) -> Self {
        Self::new(ArithmeticOperator::BitwiseAnd, rhs)
    }

    /// Creates a new bitwise xor operation.
    pub(crate) fn bitwise_xor(rhs: impl Into<TypedValue>) -> Self {
        Self::new(ArithmeticOperator::BitwiseXor, rhs)
    }
}

impl fmt::Display for ArithmeticOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.operator, self.rhs)
    }
}

impl_arithmetic_flag_traits!(Arithmetic, with_flag);
