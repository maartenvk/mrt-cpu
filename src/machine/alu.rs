use crate::machine::flags::{Flags, FlagsRegister};

pub struct Result {
    pub value: u8,
    pub flags: FlagsRegister,
}

pub fn is_signed(byte: u8) -> bool {
    return (byte & 0b1000_0000) > 0;
}

fn flags_for_operation(a: u8, b: u8, result: (u8, bool)) -> FlagsRegister {
    let mut flags = FlagsRegister::new();
    if result.0 == 0 {
        flags.set(Flags::Zero);
    }

    if result.1 {
        flags.set(Flags::Carry);
    }

    if is_signed(result.0) {
        flags.set(Flags::Sign);
    }

    // if both a, b are either signed or unsigned and different with result
    if is_signed(a) == is_signed(b) && is_signed(a) != is_signed(result.0) {
        flags.set(Flags::Overflow);
    }

    return flags;
}

pub fn add(a: u8, b: u8) -> Result {
    let result = a.overflowing_add(b);

    return Result {
        value: result.0,
        flags: flags_for_operation(a, b, result),
    };
}

pub fn sub(a: u8, b: u8) -> Result {
    let result = a.overflowing_sub(b);

    return Result {
        value: result.0,
        flags: flags_for_operation(a, b, result),
    };
}

pub fn and(a: u8, b: u8) -> Result {
    let result = a & b;

    return Result {
        value: result,
        flags: flags_for_operation(a, b, (result, false)),
    };
}

pub fn or(a: u8, b: u8) -> Result {
    let result = a | b;

    return Result {
        value: result,
        flags: flags_for_operation(a, b, (result, false)),
    };
}

pub fn xor(a: u8, b: u8) -> Result {
    let result = a ^ b;

    return Result {
        value: result,
        flags: flags_for_operation(a, b, (result, false)),
    };
}

pub fn shl(a: u8, b: u8) -> Result {
    let result = a.overflowing_shl(b as u32);

    return Result {
        value: result.0,
        flags: flags_for_operation(a, b, result),
    };
}

pub fn shr(a: u8, b: u8) -> Result {
    let result = a.overflowing_shr(b as u32);

    return Result {
        value: result.0,
        flags: flags_for_operation(a, b, result),
    };
}
