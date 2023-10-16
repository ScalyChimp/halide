use chumsky::prelude::*;

use crate::opcode::instructions::Instr;

pub fn assemble() -> impl Parser<char, Vec<Instr>, Error = Simple<char>> {
    let register = just(" $").ignore_then(
        text::digits::<char, Simple<char>>(10)
            .from_str::<u8>()
            .unwrapped(),
    );
    let value = just(" #").ignore_then(
        just('-')
            .or_not()
            .then(
                text::digits::<char, Simple<char>>(10)
                    .from_str::<i16>()
                    .unwrapped(),
            )
            .foldr(|_, b| -b),
    );

    let op_halt = just("HLT").to(Instr::Halt);
    let op_not = just("NOT").to(Instr::Not);

    let op_jmp = just("JMP").ignore_then(register).map(Instr::Jump);
    let op_jmpif = just("JMPIF").ignore_then(register).map(Instr::JumpIf);

    let op_add = just("ADD")
        .ignore_then(register)
        .then(register)
        .then(register)
        .map(|((r1, r2), rd)| Instr::Add(r1, r2, rd));
    let op_sub = just("SUB")
        .ignore_then(register)
        .then(register)
        .then(register)
        .map(|((r1, r2), rd)| Instr::Subtract(r1, r2, rd));
    let op_mul = just("MUL")
        .ignore_then(register)
        .then(register)
        .then(register)
        .map(|((r1, r2), rd)| Instr::Multiply(r1, r2, rd));
    let op_div = just("DIV")
        .ignore_then(register)
        .then(register)
        .then(register)
        .map(|((r1, r2), rd)| Instr::Divide(r1, r2, rd));

    let op_eq = just("EQ")
        .ignore_then(register)
        .then(register)
        .map(|(r1, r2)| Instr::Equal(r1, r2));
    let op_gt = just("GT")
        .ignore_then(register)
        .then(register)
        .map(|(r1, r2)| Instr::GreaterThan(r1, r2));
    let op_gtq = just("GTQ")
        .ignore_then(register)
        .then(register)
        .map(|(r1, r2)| Instr::GreaterThanEqual(r1, r2));

    let op_load = just("LOAD")
        .ignore_then(register)
        .then(value)
        .map(|(r, v)| Instr::Load(r, v));

    let opcodes = choice((
        op_halt, op_not, op_jmp, op_jmpif, op_add, op_sub, op_mul, op_div, op_eq, op_gt, op_gtq,
        op_load,
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
        assert_eq!(result, vec![Instr::Halt]);
        let result = parser.parse("NOT".to_string()).unwrap();
        assert_eq!(result, vec![Instr::Not]);
    }

    #[test]
    fn parse_zero_vec() {
        let parser = assemble();

        let result = parser.parse("HLT NOT".to_string()).unwrap();
        assert_eq!(result, vec![Instr::Halt, Instr::Not]);
    }

    #[test]
    fn parse_one_arg() {
        let parser = assemble();

        let result = parser.parse("JMP $0").unwrap();
        assert_eq!(result, vec![Instr::Jump(0)]);
        let result = parser.parse("JMPIF $1").unwrap();
        assert_eq!(result, vec![Instr::JumpIf(1)]);
    }

    #[test]
    fn parse_one_vec() {
        let parser = assemble();

        assert_eq!(
            parser
                .parse(
                    r#"
                   JMP $2
                   JMPIF $1
                   "#,
                )
                .unwrap(),
            vec![Instr::Jump(2), Instr::JumpIf(1)]
        );
    }

    #[test]
    fn parse_two_args() {
        let parser = assemble();

        let result = parser.parse("EQ $0 $1").unwrap();
        assert_eq!(result, vec![Instr::Equal(0, 1)]);
        let result = parser.parse("GT $1 $3").unwrap();
        assert_eq!(result, vec![Instr::GreaterThan(1, 3)]);
        let result = parser.parse("GTQ $2 $0").unwrap();
        assert_eq!(result, vec![Instr::GreaterThanEqual(2, 0)]);
        let result = parser.parse("LOAD $2 #1").unwrap();
        assert_eq!(result, vec![Instr::Load(2, 1)]);
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
                Instr::Equal(0, 1),
                Instr::GreaterThan(1, 3),
                Instr::GreaterThanEqual(2, 0),
                Instr::Load(2, 1),
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
                     LOAD $2 #100
                     LOAD $3 #-2"
                )
                .unwrap(),
            vec![
                Instr::Load(0, 9),
                Instr::Load(1, 10),
                Instr::Load(2, 100),
                Instr::Load(3, -2)
            ]
        );
    }

    #[test]
    fn parse_three_args() {
        let parser = assemble();

        let result = parser.parse("ADD $0 $1 $2").unwrap();
        assert_eq!(result, vec![Instr::Add(0, 1, 2)]);
        let result = parser.parse("SUB $1 $0 $3").unwrap();
        assert_eq!(result, vec![Instr::Subtract(1, 0, 3)]);
        let result = parser.parse("DIV $2 $0 $1").unwrap();
        assert_eq!(result, vec![Instr::Divide(2, 0, 1)]);
        let result = parser.parse("MUL $2 $1 $3").unwrap();
        assert_eq!(result, vec![Instr::Multiply(2, 1, 3)]);
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
                Instr::Add(0, 1, 2),
                Instr::Subtract(1, 0, 3),
                Instr::Divide(2, 0, 1),
                Instr::Multiply(2, 1, 3)
            ]
        )
    }
}
