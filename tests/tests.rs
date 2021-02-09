extern crate emu6809;

#[test]
    fn inc_pc() {
        let mut cpu = emu6809::CPU::new();
        let program = vec![0x12, 0x12];
        cpu.reset();
        cpu.run(program);
        assert_eq!(cpu.program_counter, 0x0001);
    }
    #[test]
    fn new() {
        let cpu = emu6809::CPU::new();
        assert_eq!(cpu.program_counter, 0x0000);
    }
    #[test]
    fn set_pc() {
        let mut cpu = emu6809::CPU::new();
        cpu.program_counter = 0xFFFF;
        assert_eq!(cpu.program_counter, 0xFFFF);
    }
    #[test]
    fn reset() {
        let mut cpu = emu6809::CPU::new();
        cpu.program_counter = 0xFFFF;
        cpu.reset();
        assert_eq!(cpu.program_counter, 0x0000);
    }
    
    #[test]
    fn im_lda() {
        let mut cpu = emu6809::CPU::new();
        let program = vec![0x86, 0xCA];
        cpu.reset();
        cpu.run(program);
        assert_eq!(cpu.accumulator_a, 0xCA)
    }