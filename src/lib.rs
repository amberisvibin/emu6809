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
        todo!("run the program");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
