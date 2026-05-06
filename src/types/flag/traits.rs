use crate::types::requirement::arithmetic::ArithmeticRequirement;

use super::ArithmeticFlag;

/// A trait for types that can be modified with pause if.
pub trait PauseIf {
    type Output;

    fn pause_if(self) -> Self::Output;
}

/// A trait for types that can be modified with reset if.
pub trait ResetIf {
    type Output;

    fn reset_if(self) -> Self::Output;
}

/// A trait for types that can be modified with reset next if.
pub trait ResetNextIf {
    type Output;

    fn reset_next_if(self) -> Self::Output;
}

/// A trait for types that can be modified with add source.
pub trait AddSource {
    type Output;

    fn add_source(self) -> Self::Output;
}

impl AddSource for u32 {
    type Output = ArithmeticRequirement;

    fn add_source(self) -> Self::Output {
        ArithmeticRequirement::new(ArithmeticFlag::AddSource, self)
    }
}

/// A trait for types that can be modified with sub source.
pub trait SubSource {
    type Output;

    fn sub_source(self) -> Self::Output;
}

impl SubSource for u32 {
    type Output = ArithmeticRequirement;

    fn sub_source(self) -> Self::Output {
        ArithmeticRequirement::new(ArithmeticFlag::SubSource, self)
    }
}

/// A trait for types that can be modified with add hits.
pub trait AddHits {
    type Output;

    fn add_hits(self) -> Self::Output;
}

/// A trait for types that can be modified with sub hits.
pub trait SubHits {
    type Output;

    fn sub_hits(self) -> Self::Output;
}

/// A trait for types that can be modified with add address.
pub trait AddAddress {
    type Output;

    fn add_address(self) -> Self::Output;
}

/// A trait for types that can be modified with and next.
pub trait AndNext {
    type Output;

    fn and_next(self) -> Self::Output;
}

/// A trait for types that can be modified with or next.
pub trait OrNext {
    type Output;

    fn or_next(self) -> Self::Output;
}

/// A trait for types that can be modified with measured.
pub trait Measured {
    type Output;

    fn measured(self) -> Self::Output;
}

/// A trait for types that can be modified with measured percentage.
pub trait MeasuredPercentage {
    type Output;

    fn measured_pct(self) -> Self::Output;
}

/// A trait for types that can be modified with measured if.
pub trait MeasuredIf {
    type Output;

    fn measured_if(self) -> Self::Output;
}

/// A trait for types that can be modified with trigger.
pub trait Trigger {
    type Output;

    fn trigger(self) -> Self::Output;
}

/// A trait for types that can be modified with remember.
pub trait Remember {
    type Output;

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
///
/// # Generated Traits
/// - `AddSource`
/// - `SubSource`
/// - `AddAddress`
/// - `Remember`
#[macro_export]
macro_rules! impl_arithmetic_flag_traits {
    ($struct:ident, $method:ident) => {
        impl $crate::types::flag::traits::AddSource for $struct {
            type Output = Self;
            fn add_source(self) -> Self::Output {
                self.$method($crate::types::flag::ArithmeticFlag::AddSource)
            }
        }

        impl $crate::types::flag::traits::SubSource for $struct {
            type Output = Self;
            fn sub_source(self) -> Self::Output {
                self.$method($crate::types::flag::ArithmeticFlag::SubSource)
            }
        }

        impl $crate::types::flag::traits::AddAddress for $struct {
            type Output = Self;
            fn add_address(self) -> Self::Output {
                self.$method($crate::types::flag::ArithmeticFlag::AddAddress)
            }
        }

        impl $crate::types::flag::traits::Remember for $struct {
            type Output = Self;
            fn remember(self) -> Self::Output {
                self.$method($crate::types::flag::ArithmeticFlag::Remember)
            }
        }
    };
}
