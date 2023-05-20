pub struct CPU {
    register_a: u8, //the accumilator
    register_x: u8, //the X register
    status: u8,     //flag register
    program_counter: u16,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            register_a: 0,
            register_x: 0,
            status: 0,
            program_counter: 0,
        }
    }

    pub fn update_zero_and_negative_flags(&mut self, value: u8) {
        // if the accumilator is zero then set the zero flag
        if value == 0 {
            self.status = self.status | 0b0000_0010;
        } else {
            self.status = self.status & 0b1111_1101;
        }
        // if the accumilator's 7th bit is set then set the negative flag
        if value & 0b1000_0000 != 0 {
            self.status = self.status | 0b1000_0000;
        } else {
            self.status = self.status & 0b0111_1111;
        }
    }

    //load immediate value into the register A
    pub fn lda(&mut self, value: u8) {
        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }
    //transfer the value of A to X
    pub fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    //increment the X register
    pub fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(x);
        self.update_zero_and_negative_flags(self.register_x);
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        self.program_counter = 0;
        loop {
            let opcode = program[self.program_counter as usize];
            self.program_counter += 1;

            match opcode {
                //LDA 0xA9
                0xA9 => {
                    // Load the next byte to load in the A register
                    let param = program[self.program_counter as usize];
                    self.program_counter += 1;
                    //Load it
                    self.lda(param);
                }

                //TAX 0xAA
                0xAA => self.tax(),

                //INX 0xE8
                0xE8 => self.inx(),

                //BRK 0x00
                0x00 => return,

                _ => todo!(),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0xc0, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0b00);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xa9_lda_negative_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0xF0, 0x00]);
        assert!(cpu.status & 0b1000_0000 == 0b1000_0000);
    }

    #[test]
    fn test_0xaa_tax() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0xc0, 0xaa, 0x00]);
        assert_eq!(cpu.register_x, 0xc0);
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xff;
        cpu.interpret(vec![0xe8, 0xe8, 0x00]);
        assert_eq!(cpu.register_x, 1);
    }
}
