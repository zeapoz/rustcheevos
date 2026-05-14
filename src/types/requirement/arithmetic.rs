//! Type definition for arithmetic requirements.

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
    /// The flag of the arithmetic operation.
    pub flag: ArithmeticFlag,
    /// The left hand side of the arithmetic requirement.
    pub lhs: TypedValue,
    /// The operation of the arithmetic requirement, containing the operator and right hand side.
    pub operation: Option<ArithmeticOperation>,
}

impl ArithmeticRequirement {
    /// Creates a new arithmetic requirement.
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::types::flag::ArithmeticFlag;
    /// # use rustcheevos::types::requirement::arithmetic::ArithmeticRequirement;
    /// # use rustcheevos::types::value::TypedValue;
    /// let arithmetic = ArithmeticRequirement::new(ArithmeticFlag::AddSource, 10);
    /// assert_eq!(arithmetic.flag(), ArithmeticFlag::AddSource);
    /// assert_eq!(arithmetic.lhs(), &TypedValue::from(10));
    /// ```
    pub fn new(flag: ArithmeticFlag, lhs: impl Into<TypedValue>) -> Self {
        Self {
            flag,
            lhs: lhs.into(),
            operation: None,
        }
    }

    /// Returns the arithmetic flag.
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::types::flag::ArithmeticFlag;
    /// # use rustcheevos::types::requirement::arithmetic::ArithmeticRequirement;
    /// let arithmetic = ArithmeticRequirement::new(ArithmeticFlag::AddSource, 10);
    /// assert_eq!(arithmetic.flag(), ArithmeticFlag::AddSource);
    /// ```
    #[must_use]
    pub fn flag(&self) -> ArithmeticFlag {
        self.flag
    }

    /// Returns the left hand side of the arithmetic requirement.
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::types::flag::ArithmeticFlag;
    /// # use rustcheevos::types::requirement::arithmetic::ArithmeticRequirement;
    /// # use rustcheevos::types::value::TypedValue;
    /// let arithmetic = ArithmeticRequirement::new(ArithmeticFlag::AddSource, 10);
    /// assert_eq!(arithmetic.lhs(), &TypedValue::from(10));
    /// ```
    #[must_use]
    pub fn lhs(&self) -> &TypedValue {
        &self.lhs
    }

    /// Returns the arithmetic operator.
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::types::flag::ArithmeticFlag;
    /// # use rustcheevos::types::requirement::arithmetic::ArithmeticRequirement;
    /// let arithmetic = ArithmeticRequirement::new(ArithmeticFlag::AddSource, 10);
    /// assert_eq!(arithmetic.operator(), None);
    /// ```
    #[must_use]
    pub fn operator(&self) -> Option<ArithmeticOperator> {
        self.operation.map(|o| o.operator)
    }

    /// Returns the right hand side of the arithmetic requirement.
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::types::flag::ArithmeticFlag;
    /// # use rustcheevos::types::requirement::arithmetic::ArithmeticRequirement;
    /// let arithmetic = ArithmeticRequirement::new(ArithmeticFlag::AddSource, 10);
    /// assert_eq!(arithmetic.rhs(), None);
    /// ```
    #[must_use]
    pub fn rhs(&self) -> Option<TypedValue> {
        self.operation.map(|o| o.rhs)
    }

    /// Sets the arithmetic flag on this requirement.
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::types::flag::ArithmeticFlag;
    /// # use rustcheevos::types::requirement::arithmetic::ArithmeticRequirement;
    /// let arithmetic = ArithmeticRequirement::new(ArithmeticFlag::AddSource, 10);
    /// assert_eq!(arithmetic.flag(), ArithmeticFlag::AddSource);
    /// let arithmetic = arithmetic.with_flag(ArithmeticFlag::SubSource);
    /// assert_eq!(arithmetic.flag(), ArithmeticFlag::SubSource);
    /// ```
    #[must_use]
    pub fn with_flag(mut self, flag: ArithmeticFlag) -> Self {
        self.flag = flag;
        self
    }

    /// Sets the operation on this requirement.
    ///
    /// # Arguments
    ///
    /// * `operation` - The operation to perform.
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::types::flag::ArithmeticFlag;
    /// # use rustcheevos::types::operator::ArithmeticOperator;
    /// # use rustcheevos::types::requirement::arithmetic::{ArithmeticRequirement, ArithmeticOperation};
    /// let arithmetic = ArithmeticRequirement::new(ArithmeticFlag::AddSource, 10)
    ///     .with_operation(ArithmeticOperation::add(5));
    /// assert_eq!(arithmetic.operator(), Some(ArithmeticOperator::Add));
    /// ```
    #[must_use]
    pub fn with_operation(mut self, operation: ArithmeticOperation) -> Self {
        self.operation = Some(operation);
        self
    }

    /// Adds with the given value.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right hand side of the operation.
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::types::flag::ArithmeticFlag;
    /// # use rustcheevos::types::operator::ArithmeticOperator;
    /// # use rustcheevos::types::requirement::arithmetic::ArithmeticRequirement;
    /// let arithmetic = ArithmeticRequirement::new(ArithmeticFlag::AddSource, 10).add(5);
    /// assert_eq!(arithmetic.operator(), Some(ArithmeticOperator::Add));
    /// ```
    #[expect(
        clippy::should_implement_trait,
        reason = "not using arithmetic in the traditional sense"
    )]
    #[must_use]
    pub fn add(self, rhs: impl Into<TypedValue>) -> Self {
        self.with_operation(ArithmeticOperation::add(rhs))
    }

    /// Subtracts with the given value.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right hand side of the operation.
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::types::flag::ArithmeticFlag;
    /// # use rustcheevos::types::operator::ArithmeticOperator;
    /// # use rustcheevos::types::requirement::arithmetic::ArithmeticRequirement;
    /// let arithmetic = ArithmeticRequirement::new(ArithmeticFlag::AddSource, 10).sub(5);
    /// assert_eq!(arithmetic.operator(), Some(ArithmeticOperator::Subtract));
    /// ```
    #[expect(
        clippy::should_implement_trait,
        reason = "not using arithmetic in the traditional sense"
    )]
    #[must_use]
    pub fn sub(self, rhs: impl Into<TypedValue>) -> Self {
        self.with_operation(ArithmeticOperation::sub(rhs))
    }

    /// Multiplies with the given value.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right hand side of the operation.
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::types::flag::ArithmeticFlag;
    /// # use rustcheevos::types::operator::ArithmeticOperator;
    /// # use rustcheevos::types::requirement::arithmetic::ArithmeticRequirement;
    /// let arithmetic = ArithmeticRequirement::new(ArithmeticFlag::AddSource, 10).mul(5);
    /// assert_eq!(arithmetic.operator(), Some(ArithmeticOperator::Multiply));
    /// ```
    #[expect(
        clippy::should_implement_trait,
        reason = "not using arithmetic in the traditional sense"
    )]
    #[must_use]
    pub fn mul(self, rhs: impl Into<TypedValue>) -> Self {
        self.with_operation(ArithmeticOperation::mul(rhs))
    }

    /// Divides with the given value.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right hand side of the operation.
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::types::flag::ArithmeticFlag;
    /// # use rustcheevos::types::operator::ArithmeticOperator;
    /// # use rustcheevos::types::requirement::arithmetic::ArithmeticRequirement;
    /// let arithmetic = ArithmeticRequirement::new(ArithmeticFlag::AddSource, 10).div(5);
    /// assert_eq!(arithmetic.operator(), Some(ArithmeticOperator::Divide));
    /// ```
    #[expect(
        clippy::should_implement_trait,
        reason = "not using arithmetic in the traditional sense"
    )]
    #[must_use]
    pub fn div(self, rhs: impl Into<TypedValue>) -> Self {
        self.with_operation(ArithmeticOperation::div(rhs))
    }

    /// Modulos with the given value.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right hand side of the operation.
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::types::flag::ArithmeticFlag;
    /// # use rustcheevos::types::operator::ArithmeticOperator;
    /// # use rustcheevos::types::requirement::arithmetic::ArithmeticRequirement;
    /// let arithmetic = ArithmeticRequirement::new(ArithmeticFlag::AddSource, 10).modulo(3);
    /// assert_eq!(arithmetic.operator(), Some(ArithmeticOperator::Modulo));
    /// ```
    #[must_use]
    pub fn modulo(self, rhs: impl Into<TypedValue>) -> Self {
        self.with_operation(ArithmeticOperation::modulo(rhs))
    }

    /// Bitwise ands with the given value.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right hand side of the operation.
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::types::flag::ArithmeticFlag;
    /// # use rustcheevos::types::operator::ArithmeticOperator;
    /// # use rustcheevos::types::requirement::arithmetic::ArithmeticRequirement;
    /// let arithmetic = ArithmeticRequirement::new(ArithmeticFlag::AddSource, 10).bitwise_and(6);
    /// assert_eq!(arithmetic.operator(), Some(ArithmeticOperator::BitwiseAnd));
    /// ```
    #[must_use]
    pub fn bitwise_and(self, rhs: impl Into<TypedValue>) -> Self {
        self.with_operation(ArithmeticOperation::bitwise_and(rhs))
    }

    /// Bitwise xors with the given value.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right hand side of the operation.
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::types::flag::ArithmeticFlag;
    /// # use rustcheevos::types::operator::ArithmeticOperator;
    /// # use rustcheevos::types::requirement::arithmetic::ArithmeticRequirement;
    /// let arithmetic = ArithmeticRequirement::new(ArithmeticFlag::AddSource, 10).bitwise_xor(6);
    /// assert_eq!(arithmetic.operator(), Some(ArithmeticOperator::BitwiseXor));
    /// ```
    #[must_use]
    pub fn bitwise_xor(self, rhs: impl Into<TypedValue>) -> Self {
        self.with_operation(ArithmeticOperation::bitwise_xor(rhs))
    }
}

impl FromStr for ArithmeticRequirement {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_arithmetic_requirement
            .parse(s)
            .map_err(|s| ParseError::Requirement(s.to_string()))
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
    /// The operator of the arithmetic operation.
    pub operator: ArithmeticOperator,
    /// The right hand side of the arithmetic operation.
    pub rhs: TypedValue,
}

impl ArithmeticOperation {
    /// Creates a new arithmetic operation.
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::types::operator::ArithmeticOperator;
    /// # use rustcheevos::types::requirement::arithmetic::ArithmeticOperation;
    /// # use rustcheevos::types::value::TypedValue;
    /// let operation = ArithmeticOperation::new(ArithmeticOperator::Add, 10);
    /// assert_eq!(operation.operator, ArithmeticOperator::Add);
    /// assert_eq!(operation.rhs, TypedValue::from(10));
    /// ```
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
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::types::operator::ArithmeticOperator;
    /// # use rustcheevos::types::requirement::arithmetic::ArithmeticOperation;
    /// let operation = ArithmeticOperation::add(10);
    /// assert_eq!(operation.operator, ArithmeticOperator::Add);
    /// ```
    pub fn add(rhs: impl Into<TypedValue>) -> Self {
        Self::new(ArithmeticOperator::Add, rhs)
    }

    /// Creates a new subtract operation.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right hand side of the operation.
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::types::operator::ArithmeticOperator;
    /// # use rustcheevos::types::requirement::arithmetic::ArithmeticOperation;
    /// let operation = ArithmeticOperation::sub(10);
    /// assert_eq!(operation.operator, ArithmeticOperator::Subtract);
    /// ```
    pub fn sub(rhs: impl Into<TypedValue>) -> Self {
        Self::new(ArithmeticOperator::Subtract, rhs)
    }

    /// Creates a new multiply operation.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right hand side of the operation.
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::types::operator::ArithmeticOperator;
    /// # use rustcheevos::types::requirement::arithmetic::ArithmeticOperation;
    /// let operation = ArithmeticOperation::mul(10);
    /// assert_eq!(operation.operator, ArithmeticOperator::Multiply);
    /// ```
    pub fn mul(rhs: impl Into<TypedValue>) -> Self {
        Self::new(ArithmeticOperator::Multiply, rhs)
    }

    /// Creates a new divide operation.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right hand side of the operation.
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::types::operator::ArithmeticOperator;
    /// # use rustcheevos::types::requirement::arithmetic::ArithmeticOperation;
    /// let operation = ArithmeticOperation::div(10);
    /// assert_eq!(operation.operator, ArithmeticOperator::Divide);
    /// ```
    pub fn div(rhs: impl Into<TypedValue>) -> Self {
        Self::new(ArithmeticOperator::Divide, rhs)
    }

    /// Creates a new modulo operation.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right hand side of the operation.
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::types::operator::ArithmeticOperator;
    /// # use rustcheevos::types::requirement::arithmetic::ArithmeticOperation;
    /// let operation = ArithmeticOperation::modulo(10);
    /// assert_eq!(operation.operator, ArithmeticOperator::Modulo);
    /// ```
    pub fn modulo(rhs: impl Into<TypedValue>) -> Self {
        Self::new(ArithmeticOperator::Modulo, rhs)
    }

    /// Creates a new bitwise and operation.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right hand side of the operation.
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::types::operator::ArithmeticOperator;
    /// # use rustcheevos::types::requirement::arithmetic::ArithmeticOperation;
    /// let operation = ArithmeticOperation::bitwise_and(10);
    /// assert_eq!(operation.operator, ArithmeticOperator::BitwiseAnd);
    /// ```
    pub fn bitwise_and(rhs: impl Into<TypedValue>) -> Self {
        Self::new(ArithmeticOperator::BitwiseAnd, rhs)
    }

    /// Creates a new bitwise xor operation.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The right hand side of the operation.
    ///
    /// # Examples
    /// ```
    /// # use rustcheevos::types::operator::ArithmeticOperator;
    /// # use rustcheevos::types::requirement::arithmetic::ArithmeticOperation;
    /// let operation = ArithmeticOperation::bitwise_xor(10);
    /// assert_eq!(operation.operator, ArithmeticOperator::BitwiseXor);
    /// ```
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
