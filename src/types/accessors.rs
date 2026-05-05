#[macro_export]
macro_rules! define_memory_accessors {
    ($($name:ident => $method:ident),* $(,)?) => {
        $(
            #[macro_export]
            macro_rules! $name {
                ($addr:expr) => {
                    $crate::types::memory::MemoryRef::$method($addr)
                };
            }
        )*
    };
}

define_memory_accessors! {
    bit0 => bit0,
    bit1 => bit1,
    bit2 => bit2,
    bit3 => bit3,
    bit4 => bit4,
    bit5 => bit5,
    bit6 => bit6,
    bit7 => bit7,
    lower4 => lower4,
    upper4 => upper4,
    bits8 => bits8,
    bits16 => bits16,
    bits24 => bits24,
    bits32 => bits32,
    bits16be => bits16be,
    bits24be => bits24be,
    bits32be => bits32be,
    bitcount => bitcount,
    float => float,
    floatbe => floatbe,
    double => double,
    doublebe => doublebe,
    mbf => mbf,
    mbfle => mbfle,
}

#[macro_export]
macro_rules! pause_if {
    ($cond:expr) => {
        $cond.with_flag($crate::types::flag::Flag::PauseIf)
    };
}

#[macro_export]
macro_rules! reset_if {
    ($cond:expr) => {
        $cond.with_flag($crate::types::flag::Flag::ResetIf)
    };
}

#[macro_export]
macro_rules! reset_next_if {
    ($cond:expr) => {
        $cond.with_flag($crate::types::flag::Flag::ResetNextIf)
    };
}

#[macro_export]
macro_rules! add_source {
    ($value:expr) => {
        $crate::types::condition::Condition {
            source: $crate::types::source::Source {
                reference: $value.into(),
                flag: Some($crate::types::flag::Flag::AddSource),
                memtype: None,
            },
            op: None,
            hits: 0,
        }
    };
}

#[macro_export]
macro_rules! sub_source {
    ($value:expr) => {
        $crate::types::condition::Condition {
            source: $crate::types::source::Source {
                reference: $value.into(),
                flag: Some($crate::types::flag::Flag::SubSource),
                memtype: None,
            },
            op: None,
            hits: 0,
        }
    };
}

#[macro_export]
macro_rules! add_hits {
    ($cond:expr) => {
        $cond.with_flag($crate::types::flag::Flag::AddHits)
    };
}

#[macro_export]
macro_rules! sub_hits {
    ($cond:expr) => {
        $cond.with_flag($crate::types::flag::Flag::SubHits)
    };
}

#[macro_export]
macro_rules! add_address {
    ($cond:expr) => {
        $cond.with_flag($crate::types::flag::Flag::AddAddress)
    };
}

#[macro_export]
macro_rules! and_next {
    ($cond:expr) => {
        $cond.with_flag($crate::types::flag::Flag::AndNext)
    };
}

#[macro_export]
macro_rules! or_next {
    ($cond:expr) => {
        $cond.with_flag($crate::types::flag::Flag::OrNext)
    };
}

#[macro_export]
macro_rules! measured {
    ($cond:expr) => {
        $cond.with_flag($crate::types::flag::Flag::Measured)
    };
}

#[macro_export]
macro_rules! measured_pct {
    ($cond:expr) => {
        $cond.with_flag($crate::types::flag::Flag::MeasuredPercentage)
    };
}

#[macro_export]
macro_rules! measured_if {
    ($cond:expr) => {
        $cond.with_flag($crate::types::flag::Flag::MeasuredIf)
    };
}

#[macro_export]
macro_rules! trigger {
    ($cond:expr) => {
        $cond.with_flag($crate::types::flag::Flag::Trigger)
    };
}

#[macro_export]
macro_rules! remember {
    ($cond:expr) => {
        $cond.with_flag($crate::types::flag::Flag::Remember)
    };
}

#[macro_export]
macro_rules! delta {
    ($cond:expr) => {
        $cond.with_memtype($crate::types::memory::MemoryType::Delta)
    };
}

#[macro_export]
macro_rules! prior {
    ($cond:expr) => {
        $cond.with_memtype($crate::types::memory::MemoryType::Prior)
    };
}

#[macro_export]
macro_rules! bcd {
    ($cond:expr) => {
        $cond.with_memtype($crate::types::memory::MemoryType::BCD)
    };
}

#[macro_export]
macro_rules! invert {
    ($cond:expr) => {
        $cond.with_memtype($crate::types::memory::MemoryType::Invert)
    };
}

#[macro_export]
macro_rules! chain {
    ($($item:expr),* $(,)?) => {{
        use $crate::types::condition::ConditionGroup;
        let mut vec = Vec::new();
        $(
            $crate::types::condition::extend_from_item(&mut vec, $item);
        )*
        ConditionGroup::new(vec)
    }};
}
