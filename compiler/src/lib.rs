type Int = u64;

#[derive(Debug, Eq, Clone, PartialEq)]
pub enum Ast {
    Int(Int),
    Negate(Box<Ast>),

    Add(Box<Ast>, Box<Ast>),
    Sub(Box<Ast>, Box<Ast>),
    Mul(Box<Ast>, Box<Ast>),
    Div(Box<Ast>, Box<Ast>),
    Pow(Box<Ast>, Box<Ast>),
}
