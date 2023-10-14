use vm::opcode::instructions::Instr;

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
        }
        Expr::Sub(_, _) => todo!(),
        Expr::Mul(_, _) => todo!(),
        Expr::Div(_, _) => todo!(),
        Expr::Pow(_, _) => todo!(),
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use vm::opcode::instructions::Instr;

    macro_rules! compile_eq {
        ($in:expr => $out:expr) => {
            assert_eq!(compile_expr($in, 0), $out)
        };
    }
    #[test]
    fn compile_simple() {
        compile_eq!(Expr::Int(2) => vec![Instr::Load(0, 2)]);
        compile_eq!(Expr::Negate(Box::new(Expr::Int(2))) => vec![Instr::Load(0, 2), Instr::Load(1, -1), Instr::Multiply(0, 1, 0)])
    }
}
