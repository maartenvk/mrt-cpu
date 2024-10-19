#[cfg(test)]
mod tests {
    use mrt_cpu::{machine::computer::System, types::Opcode};

    #[test]
    fn system_can_load_rom() {
        let mut sys = System::new(1);
        let result = sys.load_rom([Opcode::XOR as u8].to_vec());
        assert!(result.is_ok());
        assert_eq!(sys.get_mem(0), Opcode::XOR as u8);
    }

    #[test]
    fn system_rom_loading_preserves_ram_size() {
        let mut sys = System::new(2);

        let result = sys.load_rom([Opcode::HLT as u8].to_vec());
        assert!(result.is_ok());

        sys.set_mem(1, 255);
        assert_eq!(sys.get_mem(1), 255);
    }

    #[test]
    fn system_ram_increases_with_bigger_rom() {
        let mut sys = System::new(0);

        let result = sys.load_rom([Opcode::XOR as u8].to_vec());
        assert!(result.is_ok());

        assert_eq!(sys.get_mem(0), Opcode::XOR as u8);
    }

    // Tests for individual instructions
    #[test]
    fn system_instr_hlt_returns_true() {
        let mut sys = System::new(0);
        let _ = sys.load_rom([Opcode::HLT as u8].to_vec());

        let is_halt = sys.tick();
        assert!(is_halt);

        assert_eq!(sys.get_ip(), 0x0000);
    }
}
