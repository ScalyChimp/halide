#![allow(dead_code)]

pub mod opcode;
pub mod parsing;

use opcode::Opcode;

#[derive(Debug)]
pub struct VM {
    pub registers: [i32; 256],
    pc: usize,
    pub program: Vec<u8>,
    remainder: u32,
    cmp: bool,
}

impl Default for VM {
    fn default() -> Self {
        Self {
            registers: [0; 256],
            pc: Default::default(),
            program: Default::default(),
            remainder: Default::default(),
            cmp: Default::default(),
        }
    }
}

impl VM {
    pub fn with_program(program: Vec<u8>) -> VM {
        VM {
            program,
            ..Default::default()
        }
    }

    pub fn run(&mut self) {
        let mut done = false;
        while !done {
            done = self.execute_once()
        }
    }

    pub fn step(&mut self) {
        self.execute_once();
    }

    fn execute_once(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return true;
        }

        match self.decode_opcode() {
            Opcode::JMP => {
                let target = self.registers[self.next_byte() as usize];
                self.pc = target as usize;
            }
            Opcode::JMPIF => {
                if self.cmp {
                    let target = self.registers[self.next_byte() as usize];
                    self.pc = target as usize;
                }
            }

            Opcode::LOAD => {
                let dest = self.next_byte() as usize;
                let val = self.next_value();

                self.registers[dest] = val;
            }

            Opcode::ADD => {
                let rhs = self.registers[self.next_byte() as usize];
                let lhs = self.registers[self.next_byte() as usize];

                let dest = self.next_byte() as usize;

                self.registers[dest] = rhs + lhs;
            }
            Opcode::SUB => {
                let rhs = self.registers[self.next_byte() as usize];
                let lhs = self.registers[self.next_byte() as usize];

                let dest = self.next_byte() as usize;

                self.registers[dest] = rhs - lhs;
            }
            Opcode::MUL => {
                let rhs = self.registers[self.next_byte() as usize];
                let lhs = self.registers[self.next_byte() as usize];

                let dest = self.next_byte() as usize;

                self.registers[dest] = rhs * lhs;
            }
            Opcode::DIV => {
                let rhs = self.registers[self.next_byte() as usize];
                let lhs = self.registers[self.next_byte() as usize];

                let dest = self.next_byte() as usize;

                self.registers[dest] = rhs / lhs;
                self.remainder = (rhs % lhs) as u32;
            }

            Opcode::HLT => {
                eprintln!("Halting");
                return true;
            }
            Opcode::IGL => panic!("Illegal opcode encountered"),
            Opcode::EQ => {
                let rhs = self.registers[self.next_byte() as usize];
                let lhs = self.registers[self.next_byte() as usize];

                self.cmp = rhs == lhs;
            }
            Opcode::NOT => {
                self.cmp = !self.cmp;
            }
            Opcode::GT => {
                let rhs = self.registers[self.next_byte() as usize];
                let lhs = self.registers[self.next_byte() as usize];

                self.cmp = rhs > lhs;
            }
            Opcode::GTQ => {
                let rhs = self.registers[self.next_byte() as usize];
                let lhs = self.registers[self.next_byte() as usize];

                self.cmp = rhs >= lhs;
            }
            Opcode::POW => {
                let rhs = self.registers[self.next_byte() as usize];
                let lhs = self.registers[self.next_byte() as usize];

                let dest = self.next_byte() as usize;

                self.registers[dest] = rhs.pow(lhs.try_into().unwrap());
            }
        }
        false
    }

    fn next_byte(&mut self) -> u8 {
        let byte = self.program[self.pc];
        self.pc += 1;
        byte
    }

    fn next_value(&mut self) -> i32 {
        (((self.next_byte() as u16) << 8) | self.next_byte() as u16) as i16 as i32
    }

    fn decode_opcode(&mut self) -> Opcode {
        Opcode::from(self.next_byte())
    }
}

#[cfg(test)]
#[allow(clippy::bool_assert_comparison)]
mod tests {
    use super::*;

    #[test]
    fn init_vm() {
        let vm = VM::default();

        assert_eq!(vm.registers, [0; 256]);
    }

    #[test]
    fn grab_byte() {
        let mut vm = VM::with_program(vec![20, 66]);

        assert_eq!(vm.next_byte(), 20u8);
        assert_eq!(vm.pc, 1);

        assert_eq!(vm.next_byte(), 66u8);
        assert_eq!(vm.pc, 2);
    }

    #[test]
    fn grab_2_bytes() {
        let mut vm = VM::with_program(vec![20, 66, 4, 8]);

        assert_eq!(vm.next_value(), 5186i32);
        assert_eq!(vm.pc, 2);

        assert_eq!(vm.next_value(), 1032i32);
        assert_eq!(vm.pc, 4);
    }
    #[test]
    fn opcode_load() {
        let mut vm = VM::with_program(vec![
            Opcode::LOAD.into(),
            0,
            0b0000000,
            0b00000001,
            Opcode::LOAD.into(),
            1,
            1,
            0,
            Opcode::LOAD.into(),
            2,
            255,
            255,
        ]);
        vm.run();
        dbg!(&vm);
        assert_eq!(vm.registers[0], 1i32);
        assert_eq!(vm.registers[1], 256i32);
        assert_eq!(vm.registers[2], -1i32);
    }

    #[test]
    fn opcode_add() {
        let mut vm = VM::with_program(vec![
            Opcode::LOAD.into(),
            0,
            0,
            1,
            Opcode::LOAD.into(),
            1,
            0,
            2,
            Opcode::ADD.into(),
            0,
            1,
            2,
            Opcode::HLT.into(),
        ]);

        vm.run();

        dbg!(&vm);
        assert_eq!(vm.registers[2], 3);
    }

    #[test]
    fn opcode_sub() {
        let mut vm = VM::with_program(vec![
            Opcode::LOAD.into(),
            0,
            0,
            1,
            Opcode::LOAD.into(),
            1,
            0,
            2,
            Opcode::SUB.into(),
            0,
            1,
            2,
            Opcode::HLT.into(),
        ]);

        vm.run();

        dbg!(&vm);
        assert_eq!(vm.registers[2], -1);
    }

    #[test]
    fn opcode_mul() {
        let mut vm = VM::with_program(vec![
            Opcode::LOAD.into(),
            0,
            0,
            3,
            Opcode::LOAD.into(),
            1,
            0,
            2,
            Opcode::MUL.into(),
            0,
            1,
            2,
            Opcode::HLT.into(),
        ]);

        vm.run();

        dbg!(&vm);
        assert_eq!(vm.registers[2], 6);
    }

    #[test]
    fn opcode_div() {
        let mut vm = VM::with_program(vec![
            Opcode::LOAD.into(),
            0,
            0,
            3,
            Opcode::LOAD.into(),
            1,
            0,
            2,
            Opcode::DIV.into(),
            0,
            1,
            2,
            Opcode::HLT.into(),
        ]);

        vm.run();

        dbg!(&vm);
        assert_eq!(vm.registers[2], 1);
        assert_eq!(vm.remainder, 1);
    }

    #[test]
    fn opcode_jmp() {
        let mut vm = VM::with_program(vec![Opcode::LOAD.into(), 1, 0, 0, Opcode::JMP.into(), 1]);

        vm.step();
        assert_eq!(vm.pc, 4);

        vm.step();
        assert_eq!(vm.pc, 0)
    }

    #[test]
    fn opcode_jmpif() {
        let mut vm = VM::with_program(vec![
            Opcode::LOAD.into(),
            0,
            0,
            3,
            Opcode::LOAD.into(),
            1,
            0,
            2,
            Opcode::GT.into(),
            0,
            1,
            Opcode::JMPIF.into(),
            0,
        ]);
        vm.step();
        assert_eq!(vm.pc, 4);
        vm.step();
        assert_eq!(vm.pc, 8);

        vm.step();
        assert_eq!(vm.pc, 11);
        assert!(vm.cmp);

        vm.step();
        assert_eq!(vm.pc, 3);
    }

    #[test]
    fn opcode_eq() {
        let mut vm = VM::with_program(vec![
            Opcode::LOAD.into(),
            0,
            0,
            2,
            Opcode::LOAD.into(),
            1,
            0,
            2,
            Opcode::EQ.into(),
            0,
            1,
            Opcode::LOAD.into(),
            0,
            0,
            1,
            Opcode::LOAD.into(),
            1,
            0,
            2,
            Opcode::EQ.into(),
            0,
            1,
        ]);
        vm.step();
        vm.step();
        assert_eq!(vm.cmp, false);
        vm.step();
        assert_eq!(vm.cmp, true);
        vm.step();
        vm.step();
        dbg!(&vm);
        vm.step();
        assert_eq!(vm.cmp, false);
    }

    #[test]
    fn opcode_not() {
        let mut vm = VM::with_program(vec![Opcode::NOT.into()]);

        vm.step();
        assert_eq!(vm.cmp, true)
    }

    #[test]
    fn opcode_gt() {
        let mut vm = VM::with_program(vec![
            Opcode::LOAD.into(),
            0,
            0,
            2,
            Opcode::LOAD.into(),
            1,
            0,
            1,
            Opcode::GT.into(),
            0,
            1,
            Opcode::LOAD.into(),
            0,
            0,
            1,
            Opcode::LOAD.into(),
            1,
            0,
            2,
            Opcode::GT.into(),
            0,
            1,
        ]);
        vm.step();
        vm.step();
        assert_eq!(vm.cmp, false);
        vm.step();
        assert_eq!(vm.cmp, true);
        vm.step();
        vm.step();
        vm.step();
        assert_eq!(vm.cmp, false);
    }

    #[test]
    fn opcode_gtq() {
        let mut vm = VM::with_program(vec![
            Opcode::LOAD.into(),
            0,
            0,
            2,
            Opcode::LOAD.into(),
            1,
            0,
            2,
            Opcode::GTQ.into(),
            0,
            1,
            Opcode::LOAD.into(),
            0,
            0,
            0,
            Opcode::LOAD.into(),
            1,
            0,
            2,
            Opcode::GTQ.into(),
            0,
            1,
        ]);
        vm.step();
        vm.step();
        assert_eq!(vm.cmp, false);
        vm.step();
        assert_eq!(vm.cmp, true);
        vm.step();
        vm.step();
        vm.step();
        assert_eq!(vm.cmp, false);
    }
}
