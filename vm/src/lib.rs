#![allow(dead_code)]

use crate::opcode::Opcode;

#[derive(Default, Debug)]
pub struct VM {
    pub registers: [i32; 32],
    pc: usize,
    pub program: Vec<u8>,
    remainder: u32,
    cmp: bool,
}

impl VM {
    fn with_program(program: Vec<u8>) -> VM {
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
            Opcode::JMPF => {
                let target = self.registers[self.next_byte() as usize];
                self.pc += target as usize;
            }
            Opcode::JMPB => {
                let target = self.registers[self.next_byte() as usize];
                dbg!(target, self.pc);
                self.pc -= target as usize;
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
        }
        false
    }

    fn next_byte(&mut self) -> u8 {
        let byte = self.program[self.pc];
        self.pc += 1;
        byte
    }

    fn next_value(&mut self) -> i32 {
        (((self.next_byte() as u16) << 8) | self.next_byte() as u16) as i32
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

        assert_eq!(vm.registers, [0; 32]);
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
        ]); // load (1) into register (0) the value 1_i32
        vm.run();
        dbg!(&vm);
        assert_eq!(vm.registers[0], 1i32);
        assert_eq!(vm.registers[1], 256i32);
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
    fn opcode_jmpb() {
        let mut vm = VM::with_program(vec![
            Opcode::LOAD.into(),
            0,
            0,
            2,
            Opcode::LOAD.into(),
            1,
            0,
            1,
            Opcode::JMPB.into(),
            1,
        ]);
        vm.step();
        vm.step();
        assert_eq!(vm.pc, 8);
        vm.step();
        assert_eq!(vm.pc, 9);
    }

    #[test]
    fn opcode_jmpf() {
        let mut vm = VM::with_program(vec![
            Opcode::LOAD.into(),
            1,
            0,
            2,
            Opcode::JMPF.into(),
            1,
            Opcode::LOAD.into(),
            0,
            0,
            2,
        ]);
        vm.step();
        assert_eq!(vm.pc, 4);
        vm.step();
        assert_eq!(vm.pc, 8);
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

pub mod opcode {
    #[allow(clippy::upper_case_acronyms)]
    #[repr(u8)]
    #[derive(Clone, Copy)]
    pub enum Opcode {
        HLT = 0,

        LOAD = 1,

        ADD = 2,
        SUB = 3,
        MUL = 4,
        DIV = 5,

        JMP = 6,
        JMPF = 7,
        JMPB = 8,

        EQ = 9,
        NOT = 10,

        GT = 11,
        GTQ = 12,

        IGL = 255,
    }

    impl From<u8> for Opcode {
        fn from(v: u8) -> Self {
            use Opcode::*;
            match v {
                0 => HLT,
                1 => LOAD,
                2 => ADD,
                3 => SUB,
                4 => MUL,
                5 => DIV,
                6 => JMP,
                7 => JMPF,
                8 => JMPB,
                9 => EQ,
                10 => NOT,
                11 => GT,
                12 => GTQ,

                _ => IGL,
            }
        }
    }

    impl From<Opcode> for u8 {
        fn from(val: Opcode) -> Self {
            val as u8
        }
    }

    pub mod instructions {
        type Register = u8;
        type Value = i16;

        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum Instruction {
            Halt,
            Load(Register, Value),
            Add(Register, Register, Register),
            Subtract(Register, Register, Register),
            Multiply(Register, Register, Register),
            Divide(Register, Register, Register),
            Jump(Register),
            JumpForward(Register),
            JumpBack(Register),
            Equal(Register, Register),
            Not,
            GreaterThan(Register, Register),
            GreaterThanEqual(Register, Register),
            Illegal,
        }

        impl Instruction {
            pub fn to_bytes(self) -> Vec<u8> {
                use super::Opcode::*;
                use Instruction::*;
                match self {
                    Halt => vec![HLT.into()],
                    Load(r, v) => vec![LOAD.into(), r, to_le_bytes(v).0, to_le_bytes(v).1],
                    Add(r1, r2, dr) => vec![ADD.into(), r1, r2, dr],
                    Subtract(r1, r2, dr) => vec![SUB.into(), r1, r2, dr],
                    Multiply(r1, r2, dr) => vec![MUL.into(), r1, r2, dr],
                    Divide(r1, r2, dr) => vec![DIV.into(), r1, r2, dr],
                    Jump(r1) => vec![JMP.into(), r1],
                    JumpBack(r1) => vec![JMPB.into(), r1],
                    JumpForward(r1) => vec![JMPF.into(), r1],
                    Equal(r1, r2) => vec![EQ.into(), r1, r2],
                    Not => vec![NOT.into()],
                    GreaterThan(r1, r2) => vec![GT.into(), r1, r2],
                    GreaterThanEqual(r1, r2) => vec![GTQ.into(), r1, r2],
                    Illegal => vec![IGL.into()],
                }
            }
        }

        pub(super) fn to_le_bytes(v: i16) -> (u8, u8) {
            let first = ((v & 0xFF00u16 as i16) >> 8) as u8;
            let second = (v & 0x00FF) as u8;
            (first, second)
        }

        #[cfg(test)]
        mod tests {
            use crate::opcode::instructions::to_le_bytes;
            use crate::opcode::instructions::Instruction;

            #[test]
            fn byte_splitting() {
                let value = 0b00000010_00000011;
                assert_eq!(to_le_bytes(value), (0b00000010u8, 0b00000011u8));
                let value = 2;
                assert_eq!(to_le_bytes(value), (0u8, 2));
            }

            #[test]
            fn byte_splitting_two_bytes() {
                let value = 16;
                assert_eq!(to_le_bytes(value), (0u8, 16));
            }

            macro_rules! byte_check {
                ($in:expr => [$($ex:literal $(,)? )+]) => {{
                    let mut ex: Vec<u8> = vec![];
                    $(ex.push($ex);)+
                        assert_eq!($in.to_bytes(), ex)
                }};
                ($in:expr => $expect:literal) => {{
                    assert_eq!($in.to_bytes()[0], $expect as u8);
                }};
            }

            #[test]
            fn instructions_to_bytes() {
                use Instruction::*;
                byte_check!(Halt => 0);
                byte_check!(Not => 10);
                byte_check!(Illegal => 255);

                byte_check!(Jump(0) => [6,0]);
                byte_check!(JumpForward(0) => [7,0]);
                byte_check!(JumpBack(0) => [8,0]);

                byte_check!(Load(0, 2) => [1, 0, 0, 2]);
                byte_check!(Load(1, 19) => [1, 1, 0, 19]);

                byte_check!(Equal(0, 2) => [9, 0, 2]);
                byte_check!(GreaterThan(0, 2) => [11, 0, 2]);
                byte_check!(GreaterThanEqual(0, 2) => [12, 0, 2]);

                byte_check!(Add(0, 1, 2) => [2, 0, 1, 2]);
                byte_check!(Subtract(0, 1, 2) => [3, 0, 1, 2]);
                byte_check!(Multiply(0, 1, 2) => [4, 0, 1, 2]);
                byte_check!(Divide(0, 1, 2) => [5, 0, 1, 2]);
            }
        }
    }
}

pub mod parsing {
    use chumsky::prelude::*;

    use crate::opcode::instructions::Instruction;

    pub fn assemble() -> impl Parser<char, Vec<Instruction>, Error = Simple<char>> {
        let register = just(" $").ignore_then(
            text::digits::<char, Simple<char>>(10)
                .from_str::<u8>()
                .unwrapped(),
        );
        let value = just(" #").ignore_then(
            text::digits::<char, Simple<char>>(10)
                .from_str::<i16>()
                .unwrapped(),
        );

        let op_halt = just("HLT").to(Instruction::Halt);
        let op_not = just("NOT").to(Instruction::Not);

        let op_jmp = just("JMP").ignore_then(register).map(Instruction::Jump);
        let op_jmpf = just("JMPF")
            .ignore_then(register)
            .map(Instruction::JumpForward);
        let op_jmpb = just("JMPB")
            .ignore_then(register)
            .map(Instruction::JumpBack);

        let op_add = just("ADD")
            .ignore_then(register)
            .then(register)
            .then(register)
            .map(|((r1, r2), rd)| Instruction::Add(r1, r2, rd));
        let op_sub = just("SUB")
            .ignore_then(register)
            .then(register)
            .then(register)
            .map(|((r1, r2), rd)| Instruction::Subtract(r1, r2, rd));
        let op_mul = just("MUL")
            .ignore_then(register)
            .then(register)
            .then(register)
            .map(|((r1, r2), rd)| Instruction::Multiply(r1, r2, rd));
        let op_div = just("DIV")
            .ignore_then(register)
            .then(register)
            .then(register)
            .map(|((r1, r2), rd)| Instruction::Divide(r1, r2, rd));

        let op_eq = just("EQ")
            .ignore_then(register)
            .then(register)
            .map(|(r1, r2)| Instruction::Equal(r1, r2));
        let op_gt = just("GT")
            .ignore_then(register)
            .then(register)
            .map(|(r1, r2)| Instruction::GreaterThan(r1, r2));
        let op_gtq = just("GTQ")
            .ignore_then(register)
            .then(register)
            .map(|(r1, r2)| Instruction::GreaterThanEqual(r1, r2));

        let op_load = just("LOAD")
            .ignore_then(register)
            .then(value)
            .map(|(r, v)| Instruction::Load(r, v));

        let opcodes = choice((
            op_halt, op_not, op_jmp, op_jmpf, op_jmpb, op_add, op_sub, op_mul, op_div, op_eq,
            op_gt, op_gtq, op_load,
        ))
        .then_ignore(just('\n').or_not());
        opcodes.padded().repeated()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn parse_no_args() {
            let parser = assemble();
            let result = parser.parse("HLT".to_string()).unwrap();
            assert_eq!(result, vec![Instruction::Halt]);
            let result = parser.parse("NOT".to_string()).unwrap();
            assert_eq!(result, vec![Instruction::Not]);
        }

        #[test]
        fn parse_zero_vec() {
            let parser = assemble();

            let result = parser.parse("HLT NOT".to_string()).unwrap();
            assert_eq!(result, vec![Instruction::Halt, Instruction::Not]);
        }

        #[test]
        fn parse_one_arg() {
            let parser = assemble();

            let result = parser.parse("JMP $0").unwrap();
            assert_eq!(result, vec![Instruction::Jump(0)]);
            let result = parser.parse("JMPF $1").unwrap();
            assert_eq!(result, vec![Instruction::JumpForward(1)]);
            let result = parser.parse("JMPB $2").unwrap();
            assert_eq!(result, vec![Instruction::JumpBack(2)]);
        }

        #[test]
        fn parse_one_vec() {
            let parser = assemble();

            assert_eq!(
                parser
                    .parse(
                        r#"JMPB $2
                   JMPF $1
                   JMP $0"#,
                    )
                    .unwrap(),
                vec![
                    Instruction::JumpBack(2),
                    Instruction::JumpForward(1),
                    Instruction::Jump(0)
                ]
            );
        }

        #[test]
        fn parse_two_args() {
            let parser = assemble();

            let result = parser.parse("EQ $0 $1").unwrap();
            assert_eq!(result, vec![Instruction::Equal(0, 1)]);
            let result = parser.parse("GT $1 $3").unwrap();
            assert_eq!(result, vec![Instruction::GreaterThan(1, 3)]);
            let result = parser.parse("GTQ $2 $0").unwrap();
            assert_eq!(result, vec![Instruction::GreaterThanEqual(2, 0)]);
            let result = parser.parse("LOAD $2 #1").unwrap();
            assert_eq!(result, vec![Instruction::Load(2, 1)]);
        }

        #[test]
        fn parse_two_vec() {
            let parser = assemble();

            assert_eq!(
                parser
                    .parse(
                        r#"
                   EQ $0 $1
                   GT $1 $3
                   GTQ $2 $0
                   LOAD $2 #1"#,
                    )
                    .unwrap(),
                vec![
                    Instruction::Equal(0, 1),
                    Instruction::GreaterThan(1, 3),
                    Instruction::GreaterThanEqual(2, 0),
                    Instruction::Load(2, 1),
                ]
            );
        }

        #[test]
        fn load_multiple_bytes() {
            let parse = assemble();

            assert_eq!(
                parse
                    .parse(
                        "LOAD $0 #9
                     LOAD $1 #10
                     LOAD $2 #100"
                    )
                    .unwrap(),
                vec![
                    Instruction::Load(0, 9),
                    Instruction::Load(1, 10),
                    Instruction::Load(2, 100)
                ]
            );
        }

        #[test]
        fn parse_three_args() {
            let parser = assemble();

            let result = parser.parse("ADD $0 $1 $2").unwrap();
            assert_eq!(result, vec![Instruction::Add(0, 1, 2)]);
            let result = parser.parse("SUB $1 $0 $3").unwrap();
            assert_eq!(result, vec![Instruction::Subtract(1, 0, 3)]);
            let result = parser.parse("DIV $2 $0 $1").unwrap();
            assert_eq!(result, vec![Instruction::Divide(2, 0, 1)]);
            let result = parser.parse("MUL $2 $1 $3").unwrap();
            assert_eq!(result, vec![Instruction::Multiply(2, 1, 3)]);
        }

        #[test]
        fn parse_three_vec() {
            let parser = assemble();

            assert_eq!(
                parser
                    .parse(
                        r#" ADD $0 $1 $2
                        SUB $1 $0 $3
                        DIV $2 $0 $1
                        MUL $2 $1 $3"#,
                    )
                    .unwrap(),
                vec![
                    Instruction::Add(0, 1, 2),
                    Instruction::Subtract(1, 0, 3),
                    Instruction::Divide(2, 0, 1),
                    Instruction::Multiply(2, 1, 3)
                ]
            )
        }
    }
}
