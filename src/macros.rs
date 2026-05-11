/// Creates a memory reference to the value of the first bit (index 0) at the given address.
#[macro_export]
macro_rules! bit0 {
    ($addr:expr) => {
        $crate::types::memory::MemoryRef::bit0($addr)
    };
}

/// Creates a memory reference to the value of the second bit (index 1) at the given address.
#[macro_export]
macro_rules! bit1 {
    ($addr:expr) => {
        $crate::types::memory::MemoryRef::bit1($addr)
    };
}

/// Creates a memory reference to the value of the third bit (index 2) at the given address.
#[macro_export]
macro_rules! bit2 {
    ($addr:expr) => {
        $crate::types::memory::MemoryRef::bit2($addr)
    };
}

/// Creates a memory reference to the value of the fourth bit (index 3) at the given address.
#[macro_export]
macro_rules! bit3 {
    ($addr:expr) => {
        $crate::types::memory::MemoryRef::bit3($addr)
    };
}

/// Creates a memory reference to the value of the fifth bit (index 4) at the given address.
#[macro_export]
macro_rules! bit4 {
    ($addr:expr) => {
        $crate::types::memory::MemoryRef::bit4($addr)
    };
}

/// Creates a memory reference to the value of the sixth bit (index 5) at the given address.
#[macro_export]
macro_rules! bit5 {
    ($addr:expr) => {
        $crate::types::memory::MemoryRef::bit5($addr)
    };
}

/// Creates a memory reference to the value of the seventh bit (index 6) at the given address.
#[macro_export]
macro_rules! bit6 {
    ($addr:expr) => {
        $crate::types::memory::MemoryRef::bit6($addr)
    };
}

/// Creates a memory reference to the value of the eighth bit (index 7) at the given address.
#[macro_export]
macro_rules! bit7 {
    ($addr:expr) => {
        $crate::types::memory::MemoryRef::bit7($addr)
    };
}

/// Creates a memory reference to the value of the lower 4 bits at the given address.
#[macro_export]
macro_rules! lower4 {
    ($addr:expr) => {
        $crate::types::memory::MemoryRef::lower4($addr)
    };
}

/// Creates a memory reference to the value of the upper 4 bits at the given address.
#[macro_export]
macro_rules! upper4 {
    ($addr:expr) => {
        $crate::types::memory::MemoryRef::upper4($addr)
    };
}

/// Creates a memory reference to the 8-bit value at the given address.
#[macro_export]
macro_rules! bits8 {
    ($addr:expr) => {
        $crate::types::memory::MemoryRef::bits8($addr)
    };
}

/// Creates a memory reference to the 16-bit value at the given address.
#[macro_export]
macro_rules! bits16 {
    ($addr:expr) => {
        $crate::types::memory::MemoryRef::bits16($addr)
    };
}

/// Creates a memory reference to the 24-bit value at the given address.
#[macro_export]
macro_rules! bits24 {
    ($addr:expr) => {
        $crate::types::memory::MemoryRef::bits24($addr)
    };
}

/// Creates a memory reference to the 32-bit value at the given address.
#[macro_export]
macro_rules! bits32 {
    ($addr:expr) => {
        $crate::types::memory::MemoryRef::bits32($addr)
    };
}

/// Creates a memory reference to the 16-bit big endian value at the given address.
#[macro_export]
macro_rules! bits16be {
    ($addr:expr) => {
        $crate::types::memory::MemoryRef::bits16be($addr)
    };
}

/// Creates a memory reference to the 24-bit big endian value at the given address.
#[macro_export]
macro_rules! bits24be {
    ($addr:expr) => {
        $crate::types::memory::MemoryRef::bits24be($addr)
    };
}

/// Creates a memory reference to the 32-bit big endian value at the given address.
#[macro_export]
macro_rules! bits32be {
    ($addr:expr) => {
        $crate::types::memory::MemoryRef::bits32be($addr)
    };
}

/// Creates a memory reference to the bit count of the given address.
#[macro_export]
macro_rules! bitcount {
    ($addr:expr) => {
        $crate::types::memory::MemoryRef::bitcount($addr)
    };
}

/// Creates a memory reference to the float value at the given address.
#[macro_export]
macro_rules! float {
    ($addr:expr) => {
        $crate::types::memory::MemoryRef::float($addr)
    };
}

/// Creates a memory reference to the big endian float value at the given address.
#[macro_export]
macro_rules! floatbe {
    ($addr:expr) => {
        $crate::types::memory::MemoryRef::floatbe($addr)
    };
}

/// Creates a memory reference to the double value at the given address.
#[macro_export]
macro_rules! double {
    ($addr:expr) => {
        $crate::types::memory::MemoryRef::double($addr)
    };
}

/// Creates a memory reference to the big endian double value at the given address.
#[macro_export]
macro_rules! doublebe {
    ($addr:expr) => {
        $crate::types::memory::MemoryRef::doublebe($addr)
    };
}

/// Creates a memory reference to the MBF value at the given address.
#[macro_export]
macro_rules! mbf {
    ($addr:expr) => {
        $crate::types::memory::MemoryRef::mbf($addr)
    };
}

/// Creates a memory reference to the little endian MBF value at the given address.
#[macro_export]
macro_rules! mbfle {
    ($addr:expr) => {
        $crate::types::memory::MemoryRef::mbfle($addr)
    };
}

/// Sets the pause flag on a requirement.
#[macro_export]
macro_rules! pause_if {
    ($req:expr) => {
        $req.pause_if()
    };
}

/// Sets the reset flag on a requirement.
#[macro_export]
macro_rules! reset_if {
    ($req:expr) => {
        $req.reset_if()
    };
}

/// Sets the reset next flag on a requirement.
#[macro_export]
macro_rules! reset_next_if {
    ($req:expr) => {
        $req.reset_next_if()
    };
}

/// Sets the add source flag on a requirement.
#[macro_export]
macro_rules! add_source {
    ($req:expr) => {
        $req.add_source()
    };
}

/// Sets the sub source flag on a requirement.
#[macro_export]
macro_rules! sub_source {
    ($req:expr) => {
        $req.sub_source()
    };
}

/// Sets the add hits flag on a requirement.
#[macro_export]
macro_rules! add_hits {
    ($req:expr) => {
        $req.add_hits()
    };
}

/// Sets the sub hits flag on a requirement.
#[macro_export]
macro_rules! sub_hits {
    ($req:expr) => {
        $req.sub_hits()
    };
}

/// Sets the add address flag on a requirement.
#[macro_export]
macro_rules! add_address {
    ($req:expr) => {
        $req.add_address()
    };
}

/// Sets the and next flag on a requirement.
#[macro_export]
macro_rules! and_next {
    ($req:expr) => {
        $req.and_next()
    };
}

/// Sets the or next flag on a requirement.
#[macro_export]
macro_rules! or_next {
    ($req:expr) => {
        $req.or_next()
    };
}

/// Sets the measured flag on a requirement.
#[macro_export]
macro_rules! measured {
    ($req:expr) => {
        $req.measured()
    };
}

/// Sets the measured percentage flag on a requirement.
#[macro_export]
macro_rules! measured_pct {
    ($req:expr) => {
        $req.measured_pct()
    };
}

/// Sets the measured if flag on a requirement.
#[macro_export]
macro_rules! measured_if {
    ($req:expr) => {
        $req.measured_if()
    };
}

/// Sets the trigger flag on a requirement.
#[macro_export]
macro_rules! trigger {
    ($req:expr) => {
        $req.trigger()
    };
}

/// Sets the remember flag on a requirement.
#[macro_export]
macro_rules! remember {
    ($req:expr) => {
        $req.remember()
    };
}

/// Applies the delta memory type to a requirement.
#[macro_export]
macro_rules! delta {
    ($req:expr) => {
        $req.delta()
    };
}

/// Applies the prior memory type to a requirement.
#[macro_export]
macro_rules! prior {
    ($req:expr) => {
        $req.prior()
    };
}

/// Applies the BCD memory type to a requirement.
#[macro_export]
macro_rules! bcd {
    ($req:expr) => {
        $req.bcd()
    };
}

/// Applies the invert memory type to a requirement.
#[macro_export]
macro_rules! invert {
    ($req:expr) => {
        $req.invert()
    };
}
