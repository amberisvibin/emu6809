#[derive(Debug)]
pub struct CPU {
    pub x_index: u16,
    pub y_index: u16,
    pub user_stack: u16,
    pub system_stack: u16,
    pub program_counter: u16,
    pub accumulator_a: u8,
    pub accumulator_b: u8,
    pub direct_page: u8,
    pub condition_code: ConditionCode,
}

#[derive(Debug)]
pub struct ConditionCode {
    pub carry: bool,
    pub overflow: bool,
    pub zero: bool,
    pub neg: bool,
    pub irq_mask: bool,
    pub half_carry: bool,
    pub firq_mask: bool,
    pub entire: bool,
}

impl ConditionCode {
    pub fn new() -> Self {
        ConditionCode {
            carry: false,
            overflow: false,
            zero: false,
            neg: false,
            irq_mask: false,
            half_carry: false,
            firq_mask: false,
            entire: false,
        }
    }
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            x_index: 0,
            y_index: 0,
            user_stack: 0,
            system_stack: 0,
            program_counter: 0,
            accumulator_a: 0,
            accumulator_b: 0,
            direct_page: 0,
            condition_code: ConditionCode::new(),
        }
    }
    pub fn reset(&mut self) -> () {
        self.x_index = 0;
        self.y_index = 0;
        self.user_stack = 0;
        self.system_stack = 0;
        self.program_counter = 0;
        self.accumulator_a = 0;
        self.accumulator_b = 0;
        self.direct_page = 0;
        self.condition_code = ConditionCode::new();
    }
    pub fn run(&mut self, program: Vec<u8>) {
        //TODO: run the program
        loop {
            let mut counter = program.len();
            let opcode = program[self.program_counter as usize];
            let operator = program[self.program_counter as usize + 1];

            match opcode {
                0x86 => self.accumulator_a = operator,
                _ => todo!("match opcode"),
            }
            self.program_counter += 1;
            counter -= 2;
            match counter {
                0 => break,
                _ => (),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let cpu = CPU::new();
        assert_eq!(cpu.program_counter, 0x0000);
    }
    #[test]
    fn set_pc() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0xFFFF;
        assert_eq!(cpu.program_counter, 0xFFFF);
    }
    #[test]
    fn reset() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0xFFFF;
        cpu.reset();
        assert_eq!(cpu.program_counter, 0x0000);
    }
    #[test]
    fn im_lda() {
        let mut cpu = CPU::new();
        let mut program = vec![0x86, 0xCA];
        cpu.reset();
        cpu.run(program);
        assert_eq!(cpu.accumulator_a, 0xCA)
    }
}
