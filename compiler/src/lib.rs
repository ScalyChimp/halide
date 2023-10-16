use vm::opcode::instructions::Instr;

pub mod parser;

type Int = i16;

#[derive(Debug, Eq, Clone, PartialEq)]
pub enum Ast {
    Let { ident: String, value: Expr },
}

#[derive(Debug, Eq, Clone, PartialEq)]
pub enum Expr {
    Int(Int),
    Negate(Box<Expr>),

    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
}

pub fn compile_expr(expr: Expr, mut next_register: u8) -> Vec<Instr> {
    let mut results = vec![];

    match expr {
        Expr::Int(x) => {
            results.push(Instr::Load(next_register, x));
        }
        Expr::Negate(x) => {
            results.append(&mut compile_expr(*x, next_register));
            results.push(Instr::Load(next_register + 1, -1));
            results.push(Instr::Multiply(
                next_register,
                next_register + 1,
                next_register,
            ));
        }
        Expr::Add(a, b) => {
            results.append(&mut compile_expr(*a, next_register));
            next_register += 1;
            results.append(&mut compile_expr(*b, next_register));
            results.push(Instr::Add(
                next_register - 1,
                next_register,
                next_register - 1,
            ))
        }
        Expr::Sub(a, b) => {
            results.append(&mut compile_expr(*a, next_register));
            next_register += 1;
            results.append(&mut compile_expr(*b, next_register));
            results.push(Instr::Subtract(
                next_register - 1,
                next_register,
                next_register - 1,
            ))
        }
        Expr::Mul(a, b) => {
            results.append(&mut compile_expr(*a, next_register));
            next_register += 1;
            results.append(&mut compile_expr(*b, next_register));
            results.push(Instr::Multiply(
                next_register - 1,
                next_register,
                next_register - 1,
            ))
        }
        Expr::Div(a, b) => {
            results.append(&mut compile_expr(*a, next_register));
            next_register += 1;
            results.append(&mut compile_expr(*b, next_register));
            results.push(Instr::Divide(
                next_register - 1,
                next_register,
                next_register - 1,
            ))
        }
        Expr::Pow(a, b) => {
            results.append(&mut compile_expr(*a, next_register));
            next_register += 1;
            results.append(&mut compile_expr(*b, next_register));
            results.push(Instr::Power(
                next_register - 1,
                next_register,
                next_register - 1,
            ))
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use chumsky::Parser;
    use vm::opcode::instructions::Instr::*;

    macro_rules! compile_eq {
        ($in:expr => $out:expr) => {
            assert_eq!(compile_expr($in, 0), $out)
        };
    }

    macro_rules! compile_str_eq {
        ($in:expr => $out:expr) => {
            assert_eq!(compile_expr(parser::expr().parse($in).unwrap(), 0), $out)
        };
    }
    #[test]
    fn compile_load() {
        compile_eq!(Expr::Int(2) => vec![Load(0, 2)]);
    }

    #[test]
    fn compile_negate() {
        compile_eq!(Expr::Negate(Box::new(Expr::Int(2))) => vec![Load(0, 2), Load(1, -1), Multiply(0, 1, 0)])
    }

    #[test]
    fn compile_binop() {
        compile_eq!(Expr::Add(Box::new(Expr::Int(2)), Box::new(Expr::Int(3))) => vec![Load(0, 2),Load(1, 3), Add(0, 1, 0) ]);
        compile_eq!(Expr::Sub(Box::new(Expr::Int(2)), Box::new(Expr::Int(3))) => vec![Load(0, 2),Load(1, 3), Subtract(0, 1, 0) ]);
        compile_eq!(Expr::Mul(Box::new(Expr::Int(2)), Box::new(Expr::Int(3))) => vec![Load(0, 2),Load(1, 3), Multiply(0, 1, 0) ]);
        compile_eq!(Expr::Div(Box::new(Expr::Int(2)), Box::new(Expr::Int(3))) => vec![Load(0, 2),Load(1, 3), Divide(0, 1, 0) ]);
        compile_eq!(Expr::Pow(Box::new(Expr::Int(2)), Box::new(Expr::Int(3))) => vec![Load(0, 2),Load(1, 3), Power(0, 1, 0) ]);
    }

    #[test]
    fn compile_nested_binop() {
        compile_str_eq!("2 + 3 - 2" => vec![Load(0, 2),Load(1, 3), Add(0, 1, 0), Load(1, 2), Subtract(0, 1, 0)]);
        compile_str_eq!("2 / 3 * 2" => vec![Load(0, 2),Load(1, 3), Divide(0, 1, 0), Load(1, 2), Multiply(0, 1, 0)])
    }

    #[test]
    fn compile_nested_precedence_binop() {
        compile_str_eq!("2 - (3 * 2)" => vec![Load(0, 2), Load(1, 3), Load(2, 2), Multiply(1, 2, 1), Subtract(0, 1, 0)])
    }
}
