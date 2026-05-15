//! Traits for types that can be modified with flags.

use crate::types::requirement::arithmetic::Arithmetic;

use super::ArithmeticFlag;

/// A trait for types that can be modified with pause if.
pub trait PauseIf {
    /// The output type.
    type Output;

    /// Sets the [`PauseIf`][`super::ComparisonFlag::PauseIf`] flag on this type.
    fn pause_if(self) -> Self::Output;
}

/// A trait for types that can be modified with reset if.
pub trait ResetIf {
    /// The output type.
    type Output;

    /// Sets the [`ResetIf`][`super::ComparisonFlag::ResetIf`] flag on this type.
    fn reset_if(self) -> Self::Output;
}

/// A trait for types that can be modified with reset next if.
pub trait ResetNextIf {
    /// The output type.
    type Output;

    /// Sets the [`ResetNextIf`][`super::ComparisonFlag::ResetNextIf`] flag on this type.
    fn reset_next_if(self) -> Self::Output;
}

/// A trait for types that can be modified with add source.
pub trait AddSource {
    /// The output type.
    type Output;

    /// Sets the [`AddSource`][`super::ArithmeticFlag::AddSource`] flag on this type.
    fn add_source(self) -> Self::Output;
}

impl AddSource for u32 {
    /// The output type.
    type Output = Arithmetic;

    /// Sets the [`AddSource`][`super::ArithmeticFlag::AddSource`] flag on this type.
    fn add_source(self) -> Self::Output {
        Arithmetic::new(ArithmeticFlag::AddSource, self)
    }
}

/// A trait for types that can be modified with sub source.
pub trait SubSource {
    /// The output type.
    type Output;

    /// Sets the [`SubSource`][`super::ArithmeticFlag::SubSource`] flag on this type.
    fn sub_source(self) -> Self::Output;
}

impl SubSource for u32 {
    type Output = Arithmetic;

    fn sub_source(self) -> Self::Output {
        Arithmetic::new(ArithmeticFlag::SubSource, self)
    }
}

/// A trait for types that can be modified with add hits.
pub trait AddHits {
    /// The output type.
    type Output;

    /// Sets the [`AddHits`][`super::ComparisonFlag::AddHits`] flag on this type.
    fn add_hits(self) -> Self::Output;
}

/// A trait for types that can be modified with sub hits.
pub trait SubHits {
    /// The output type.
    type Output;

    /// Sets teh [`SubHits`][`super::ComparisonFlag::SubHits`] flag on this type.
    fn sub_hits(self) -> Self::Output;
}

/// A trait for types that can be modified with add address.
pub trait AddAddress {
    /// The output type.
    type Output;

    /// Sets the [`AddAddress`][`super::ArithmeticFlag::AddAddress`] flag on this type.
    fn add_address(self) -> Self::Output;
}

/// A trait for types that can be modified with and next.
pub trait AndNext {
    /// The output type.
    type Output;

    /// Sets the [`AndNext`][`super::ComparisonFlag::AndNext`] flag on this type.
    fn and_next(self) -> Self::Output;
}

/// A trait for types that can be modified with or next.
pub trait OrNext {
    /// The output type.
    type Output;

    /// Sets the [`OrNext`][`super::ComparisonFlag::OrNext`] flag on this type.
    fn or_next(self) -> Self::Output;
}

/// A trait for types that can be modified with measured.
pub trait Measured {
    /// The output type.
    type Output;

    /// Sets the [`Measured`][`super::ComparisonFlag::Measured`] flag on this type.
    fn measured(self) -> Self::Output;
}

/// A trait for types that can be modified with measured percentage.
pub trait MeasuredPercentage {
    /// The output type.
    type Output;

    /// Sets the [`MeasuredPercentage`][`super::ComparisonFlag::MeasuredPercentage`] flag on this type.
    fn measured_pct(self) -> Self::Output;
}

/// A trait for types that can be modified with measured if.
pub trait MeasuredIf {
    /// The output type.
    type Output;

    /// Sets the [`MeasuredIf`][`super::ComparisonFlag::MeasuredIf`] flag on this type.
    fn measured_if(self) -> Self::Output;
}

/// A trait for types that can be modified with trigger.
pub trait Trigger {
    /// The output type.
    type Output;

    /// Sets the [`Trigger`][`super::ComparisonFlag::Trigger`] flag on this type.
    fn trigger(self) -> Self::Output;
}

/// A trait for types that can be modified with remember.
pub trait Remember {
    /// The output type.
    type Output;

    /// Sets the [`Remember`][`super::ArithmeticFlag::Remember`] flag on this type.
    fn remember(self) -> Self::Output;
}

/// Implements all comparison flag traits for a struct.
///
/// # Args
/// - `$struct`: The struct to implement traits for.
/// - `$method`: The method to call on self with the flag (e.g., `with_flag`).
///
/// # Generated Traits
/// - `PauseIf`
/// - `ResetIf`
/// - `ResetNextIf`
/// - `AddHits`
/// - `SubHits`
/// - `AndNext`
/// - `OrNext`
/// - `Measured`
/// - `MeasuredPercentage`
/// - `MeasuredIf`
/// - `Trigger`
#[doc(hidden)]
#[macro_export]
macro_rules! impl_comparison_flag_traits {
    ($struct:ident, $method:ident) => {
        impl $crate::types::flag::traits::PauseIf for $struct {
            type Output = Self;
            fn pause_if(self) -> Self::Output {
                self.$method($crate::types::flag::ComparisonFlag::PauseIf)
            }
        }

        impl $crate::types::flag::traits::ResetIf for $struct {
            type Output = Self;
            fn reset_if(self) -> Self::Output {
                self.$method($crate::types::flag::ComparisonFlag::ResetIf)
            }
        }

        impl $crate::types::flag::traits::ResetNextIf for $struct {
            type Output = Self;
            fn reset_next_if(self) -> Self::Output {
                self.$method($crate::types::flag::ComparisonFlag::ResetNextIf)
            }
        }

        impl $crate::types::flag::traits::AddHits for $struct {
            type Output = Self;
            fn add_hits(self) -> Self::Output {
                self.$method($crate::types::flag::ComparisonFlag::AddHits)
            }
        }

        impl $crate::types::flag::traits::SubHits for $struct {
            type Output = Self;
            fn sub_hits(self) -> Self::Output {
                self.$method($crate::types::flag::ComparisonFlag::SubHits)
            }
        }

        impl $crate::types::flag::traits::AndNext for $struct {
            type Output = Self;
            fn and_next(self) -> Self::Output {
                self.$method($crate::types::flag::ComparisonFlag::AndNext)
            }
        }

        impl $crate::types::flag::traits::OrNext for $struct {
            type Output = Self;
            fn or_next(self) -> Self::Output {
                self.$method($crate::types::flag::ComparisonFlag::OrNext)
            }
        }

        impl $crate::types::flag::traits::Measured for $struct {
            type Output = Self;
            fn measured(self) -> Self::Output {
                self.$method($crate::types::flag::ComparisonFlag::Measured)
            }
        }

        impl $crate::types::flag::traits::MeasuredPercentage for $struct {
            type Output = Self;
            fn measured_pct(self) -> Self::Output {
                self.$method($crate::types::flag::ComparisonFlag::MeasuredPercentage)
            }
        }

        impl $crate::types::flag::traits::MeasuredIf for $struct {
            type Output = Self;
            fn measured_if(self) -> Self::Output {
                self.$method($crate::types::flag::ComparisonFlag::MeasuredIf)
            }
        }

        impl $crate::types::flag::traits::Trigger for $struct {
            type Output = Self;
            fn trigger(self) -> Self::Output {
                self.$method($crate::types::flag::ComparisonFlag::Trigger)
            }
        }
    };
}

/// Implements all arithmetic flag traits for a struct.
///
/// # Args
/// - `$struct`: The struct to implement traits for.
/// - `$method`: The method to call on self with the flag (e.g., `with_flag`).
/// - `$output`: (optional) The output type. Defaults to `Self`.
///
/// # Generated Traits
/// - `AddSource`
/// - `SubSource`
/// - `AddAddress`
/// - `Remember`
#[doc(hidden)]
#[macro_export]
macro_rules! impl_arithmetic_flag_traits {
    ($struct:ident, $method:ident) => {
        impl_arithmetic_flag_traits!($struct, $method, Self);
    };
    ($struct:ident, $method:ident, $output:ty) => {
        impl $crate::types::flag::traits::AddSource for $struct {
            type Output = $output;
            fn add_source(self) -> Self::Output {
                self.$method($crate::types::flag::ArithmeticFlag::AddSource)
            }
        }

        impl $crate::types::flag::traits::SubSource for $struct {
            type Output = $output;
            fn sub_source(self) -> Self::Output {
                self.$method($crate::types::flag::ArithmeticFlag::SubSource)
            }
        }

        impl $crate::types::flag::traits::AddAddress for $struct {
            type Output = $output;
            fn add_address(self) -> Self::Output {
                self.$method($crate::types::flag::ArithmeticFlag::AddAddress)
            }
        }

        impl $crate::types::flag::traits::Remember for $struct {
            type Output = $output;
            fn remember(self) -> Self::Output {
                self.$method($crate::types::flag::ArithmeticFlag::Remember)
            }
        }
    };
}
