use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
#[logos(skip r" \t\n\f+")]
pub enum Token<'a> {
    #[token("+")]
    Add,
    #[token("-")]
    Sub,
    #[token("*")]
    Mul,
    #[token("/")]
    Div,
    #[token("^")]
    Pow,
    #[token("=")]
    Eq,

    #[token("(")]
    LParen,
    #[token(")")]
    RParen,

    #[regex(r"([1-9])?[0-9]+")]
    Int(&'a str),
}
