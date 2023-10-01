#![allow(dead_code)]
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
    use crate::Opcode;

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
            use Instruction::*;
            use Opcode::*;
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
        let first = (v >> 8) as u8;
        let second = (v & 0x0F) as u8;
        (first, second)
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::Instruction;

    use super::instructions::to_le_bytes;

    #[test]
    fn byte_splitting() {
        let value = 0b00000010_00000011;
        assert_eq!(to_le_bytes(value), (0b00000010u8, 0b00000011u8));
        let value = 2;
        assert_eq!(to_le_bytes(value), (0u8, 2))
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

        byte_check!(Equal(0, 2) => [9, 0, 2]);
        byte_check!(GreaterThan(0, 2) => [11, 0, 2]);
        byte_check!(GreaterThanEqual(0, 2) => [12, 0, 2]);

        byte_check!(Add(0, 1, 2) => [2, 0, 1, 2]);
        byte_check!(Subtract(0, 1, 2) => [3, 0, 1, 2]);
        byte_check!(Multiply(0, 1, 2) => [4, 0, 1, 2]);
        byte_check!(Divide(0, 1, 2) => [5, 0, 1, 2]);
    }
}
