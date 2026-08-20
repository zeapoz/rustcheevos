//! Re-exports of the most commonly used traits and macros.

// Traits
#[doc(inline)]
pub use crate::types::{
    chain::Chainable,
    flag::{
        AddAddress, AddHits, AddSource, AndNext, Measured, MeasuredIf, MeasuredPercentage, OrNext,
        PauseIf, Remember, ResetIf, ResetNextIf, SubHits, SubSource, Trigger,
    },
    memory::AccessModeModifier,
    value::TypedValueOps,
};

// Core macros
#[doc(inline)]
pub use crate::{
    add_address, add_hits, add_source, and_next, bcd, bit0, bit1, bit2, bit3, bit4, bit5, bit6,
    bit7, bitcount, bits8, bits16, bits16be, bits24, bits24be, bits32, bits32be, chain, delta,
    double, doublebe, float, floatbe, invert, lower4, mbf, mbfle, measured, measured_if,
    measured_pct, or_next, pause_if, prior, remember, reset_if, reset_next_if, sub_hits,
    sub_source, trigger, upper4,
};
