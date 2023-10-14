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
    POW = 6,

    JMP = 7,
    JMPF = 8,
    JMPB = 9,

    EQ = 10,
    NOT = 11,

    GT = 12,
    GTQ = 13,

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
            6 => POW,
            7 => JMP,
            8 => JMPF,
            9 => JMPB,
            10 => EQ,
            11 => NOT,
            12 => GT,
            13 => GTQ,

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
    pub enum Instr {
        Halt,
        Load(Register, Value),
        Add(Register, Register, Register),
        Subtract(Register, Register, Register),
        Multiply(Register, Register, Register),
        Divide(Register, Register, Register),
        Power(Register, Register, Register),
        Jump(Register),
        JumpForward(Register),
        JumpBack(Register),
        Equal(Register, Register),
        Not,
        GreaterThan(Register, Register),
        GreaterThanEqual(Register, Register),
        Illegal,
    }

    impl Instr {
        pub fn to_bytes(self) -> Vec<u8> {
            use super::Opcode::*;
            use Instr::*;
            match self {
                Halt => vec![HLT.into()],
                Load(r, v) => vec![LOAD.into(), r, to_le_bytes(v).0, to_le_bytes(v).1],
                Add(r1, r2, dr) => vec![ADD.into(), r1, r2, dr],
                Subtract(r1, r2, dr) => vec![SUB.into(), r1, r2, dr],
                Multiply(r1, r2, dr) => vec![MUL.into(), r1, r2, dr],
                Divide(r1, r2, dr) => vec![DIV.into(), r1, r2, dr],
                Power(r1, r2, dr) => vec![POW.into(), r1, r2, dr],
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
        use crate::opcode::instructions::Instr;

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
            use Instr::*;
            byte_check!(Halt => 0);
            byte_check!(Not => 11);
            byte_check!(Illegal => 255);

            byte_check!(Jump(0) => [7,0]);
            byte_check!(JumpForward(0) => [8,0]);
            byte_check!(JumpBack(0) => [9,0]);

            byte_check!(Load(0, 2) => [1, 0, 0, 2]);
            byte_check!(Load(1, 19) => [1, 1, 0, 19]);

            byte_check!(Equal(0, 2) => [10, 0, 2]);
            byte_check!(GreaterThan(0, 2) => [12, 0, 2]);
            byte_check!(GreaterThanEqual(0, 2) => [13, 0, 2]);

            byte_check!(Add(0, 1, 2) => [2, 0, 1, 2]);
            byte_check!(Subtract(0, 1, 2) => [3, 0, 1, 2]);
            byte_check!(Multiply(0, 1, 2) => [4, 0, 1, 2]);
            byte_check!(Divide(0, 1, 2) => [5, 0, 1, 2]);
            byte_check!(Power(0, 3, 2) => [6, 0, 3, 2]);
        }
    }
}
