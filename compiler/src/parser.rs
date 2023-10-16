#![allow(dead_code)]
use chumsky::prelude::*;

use crate::{Ast, Expr};

pub fn expr() -> impl Parser<char, Expr, Error = Simple<char>> {
    recursive(|expr| {
        let int = text::int(10).from_str().unwrapped().map(Expr::Int);

        let atom = int.or(expr.delimited_by(just('('), just(')')));

        let negated = just('-')
            .padded()
            .repeated()
            .then(atom)
            .foldr(|_, rhs| Expr::Negate(Box::new(rhs)));

        let op = |op, f| one_of(op).to(f).padded();

        let expo = negated
            .clone()
            .then(
                op("^", Expr::Pow as fn(_, _) -> _)
                    .then(negated.clone())
                    .repeated(),
            )
            .foldl(|rhs, (op, lhs)| op(Box::new(rhs), Box::new(lhs)));

        let product = expo
            .clone()
            .then(
                op("*", Expr::Mul as fn(_, _) -> _)
                    .or(op("/", Expr::Div as fn(_, _) -> _))
                    .then(expo.clone())
                    .repeated(),
            )
            .foldl(|rhs, (op, lhs)| op(Box::new(rhs), Box::new(lhs)));

        product
            .clone()
            .then(
                op("+", Expr::Add as fn(_, _) -> _)
                    .or(op("-", Expr::Sub as fn(_, _) -> _))
                    .then(product.clone())
                    .repeated(),
            )
            .foldl(|rhs, (op, lhs)| op(Box::new(rhs), Box::new(lhs)))
    })
}

fn parse_decl() -> impl Parser<char, Ast, Error = Simple<char>> {
    let expr = expr();

    text::ident()
        .padded()
        .then_ignore(just("=").padded())
        .then(expr)
        .map(|(ident, expr)| Ast::Let { ident, value: expr })
}

fn multiple_exprs() -> impl Parser<char, Vec<Expr>, Error = Simple<char>> {
    expr().padded().repeated()
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! parse_expr_eq {
        ($in:expr => $out:expr) => {
            assert_eq!(expr().parse($in).unwrap(), $out)
        };
    }

    macro_rules! parse_exprs_eq {
        ($in:expr => $out:expr) => {
            assert_eq!(multiple_exprs().parse($in).unwrap(), $out)
        };
    }

    macro_rules! parse_decl_eq {
        ($in:expr => $out:expr) => {
            assert_eq!(super::parse_decl().parse($in).unwrap(), $out)
        };
    }

    #[test]
    fn parse_one_expr() {
        parse_expr_eq!("1" => Expr::Int(1));
        parse_expr_eq!("23" => Expr::Int(23));
    }

    #[test]
    fn parse_one_negated() {
        parse_expr_eq!("-1" => Expr::Negate(Box::new(Expr::Int(1))))
    }

    #[test]
    fn parse_group_negated() {
        parse_expr_eq!("-(1 + 3)" => Expr::Negate(Box::new(Expr::Add(Box::new(Expr::Int(1)), Box::new(Expr::Int(3))))))
    }

    #[test]
    fn parse_one_binop() {
        parse_expr_eq!("1 + 1" => Expr::Add(Box::new(Expr::Int(1)), Box::new(Expr::Int(1)),));
        parse_expr_eq!("1 - 1" => Expr::Sub(Box::new(Expr::Int(1)), Box::new(Expr::Int(1)),));
        parse_expr_eq!("1 * 1" => Expr::Mul(Box::new(Expr::Int(1)), Box::new(Expr::Int(1)),));
        parse_expr_eq!("1 / 1" => Expr::Div(Box::new(Expr::Int(1)), Box::new(Expr::Int(1)),));
        parse_expr_eq!("1 ^ 1" => Expr::Pow(Box::new(Expr::Int(1)), Box::new(Expr::Int(1)),));
    }

    #[test]
    fn parse_many_int() {
        parse_exprs_eq!(
            "1 2 3" =>
            vec![
                Expr::Int(1),
                Expr::Int(2),
                Expr::Int(3),
            ]
        )
    }

    #[test]
    fn parse_many_binop() {
        parse_exprs_eq!(
            "1 + 3
             2 / 3" =>
            vec![
                Expr::Add(Box::new(Expr::Int(1)), Box::new(Expr::Int(3))),
                Expr::Div(Box::new(Expr::Int(2)), Box::new(Expr::Int(3)))
            ]
        )
    }

    #[test]
    fn parse_nested_binop() {
        parse_exprs_eq!(
            "2 / 8 - 4" =>
            vec![Expr::Sub(Box::new(Expr::Div(Box::new(Expr::Int(2)), Box::new(Expr::Int(8)))), Box::new(Expr::Int(4)))]
        )
    }

    #[test]
    fn parse_bracket() {
        parse_exprs_eq!(
            "2 / (8 - 4)" =>
            vec![Expr::Div(Box::new(Expr::Int(2)), Box::new(Expr::Sub(Box::new(Expr::Int(8)), Box::new(Expr::Int(4)))))]
        )
    }

    #[test]
    fn parse_precedence() {
        parse_exprs_eq!("2 + 4 * 3" => vec![Add(Box::new(Int(2)), Box::new(Mul(Box::new(Int(4)), Box::new(Int(3)))))]);
    }

    #[test]
    fn parse_decl() {
        parse_decl_eq!("x = 2" => Ast::Let { ident: "x".to_string(), value: Expr::Int(2) });
        parse_decl_eq!(" x = 2 + 2 " => Ast::Let { ident: "x".to_string(), value: Expr::Add(Box::new(Expr::Int(2)), Box::new(Expr::Int(2))) })
    }
}
