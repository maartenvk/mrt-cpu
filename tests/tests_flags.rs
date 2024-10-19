#[cfg(test)]
mod tests {
    use mrt_cpu::machine::flags::*;

    #[test]
    fn flags_initialized_off() {
        let flags = FlagsRegister::new();

        assert!(!flags.is_set(Flags::Zero));
        assert!(!flags.is_set(Flags::Carry));
        assert!(!flags.is_set(Flags::Sign));
        assert!(!flags.is_set(Flags::Overflow));
    }

    #[test]
    fn flags_can_set() {
        let mut flags = FlagsRegister::new();
        flags.set(Flags::Overflow);

        assert!(flags.is_set(Flags::Overflow));
    }

    #[test]
    fn flags_can_unset() {
        let mut flags = FlagsRegister::new();
        flags.set(Flags::Overflow);
        flags.unset(Flags::Overflow);

        assert!(!flags.is_set(Flags::Overflow));
    }

    #[test]
    fn flags_to_vec() {
        let mut flags = FlagsRegister::new();
        flags.set(Flags::Carry);
        flags.set(Flags::Sign);

        let result = flags.get_flags();
        assert_eq!(result.len(), 2);
        assert!(matches!(result[0], Flags::Carry));
        assert!(matches!(result[1], Flags::Sign));
    }
}
