use chumsky::prelude::*;
use opcodes::instructions::Instruction;

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

    let op_jmp = just("JMP")
        .ignore_then(register)
        .map(|x| Instruction::Jump(x));
    let op_jmpf = just("JMPF")
        .ignore_then(register)
        .map(|x| Instruction::JumpForward(x));
    let op_jmpb = just("JMPB")
        .ignore_then(register)
        .map(|x| Instruction::JumpBack(x));

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
        op_halt, op_not, op_jmp, op_jmpf, op_jmpb, op_add, op_sub, op_mul, op_div, op_eq, op_gt,
        op_gtq, op_load,
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
        let result = parser.parse(format!("HLT")).unwrap();
        assert_eq!(result, vec![Instruction::Halt]);
        let result = parser.parse(format!("NOT")).unwrap();
        assert_eq!(result, vec![Instruction::Not]);
    }

    #[test]
    fn parse_zero_vec() {
        let parser = assemble();

        let result = parser.parse(format!("HLT NOT")).unwrap();
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
                    r#"EQ $0 $1
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
