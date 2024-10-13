#[cfg(test)]
mod tests {
    use mrt_cpu::{alu as ALU, flags::*};

    #[test]
    fn alu_is_signed() {
        let mut value = 0u8;
        let mut result = ALU::is_signed(value);

        assert!(!result);

        value = -1i8 as u8;
        result = ALU::is_signed(value);

        assert!(result);
    }

    #[test]
    fn alu_flags_zero() {
        let result = ALU::sub(1, 1);
        assert!(result.flags.is_set(Flags::Zero));

        let result = ALU::sub(1, 0);
        assert!(!result.flags.is_set(Flags::Zero));
    }

    #[test]
    fn alu_flags_carry() {
        let result = ALU::add(255, 255);
        assert!(result.flags.is_set(Flags::Carry));

        let result = ALU::add(1, 1);
        assert!(!result.flags.is_set(Flags::Carry));
    }

    #[test]
    fn alu_flags_sign() {
        let result = ALU::sub(0, 1);
        assert!(result.flags.is_set(Flags::Sign));

        let result = ALU::sub(0, 0);
        assert!(!result.flags.is_set(Flags::Sign));
    }

    #[test]
    fn alu_flags_overflow() {
        let result = ALU::add(127, 1);
        assert!(result.flags.is_set(Flags::Overflow));

        let result = ALU::sub(0, 0);
        assert!(!result.flags.is_set(Flags::Overflow));
    }
}
