#![allow(dead_code)]
use chumsky::prelude::*;
use compiler::Ast;

fn parse_once() -> impl Parser<char, Ast, Error = Simple<char>> {
    recursive(|expr| {
        let int = text::int(10).from_str().unwrapped().map(Ast::Int);

        let atom = int.or(expr.delimited_by(just('('), just(')')));

        let negated = just('-')
            .padded()
            .repeated()
            .then(atom)
            .foldr(|_, rhs| Ast::Negate(Box::new(rhs)));

        let expr = negated
            .clone()
            .then(one_of("+-*/^").padded().then(negated).repeated())
            .foldl(|rhs, (op, lhs)| match op {
                '+' => Ast::Add(Box::new(rhs), Box::new(lhs)),
                '-' => Ast::Sub(Box::new(rhs), Box::new(lhs)),
                '*' => Ast::Mul(Box::new(rhs), Box::new(lhs)),
                '/' => Ast::Div(Box::new(rhs), Box::new(lhs)),
                '^' => Ast::Pow(Box::new(rhs), Box::new(lhs)),
                _ => unreachable!(),
            });

        expr
    })
}

fn parse_many() -> impl Parser<char, Vec<Ast>, Error = Simple<char>> {
    parse_once().padded().repeated()
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! parse_once_eq {
        ($in:expr => $out:expr) => {
            assert_eq!(parse_once().parse($in).unwrap(), $out)
        };
    }

    macro_rules! parse_many_eq {
        ($in:expr => $out:expr) => {
            assert_eq!(parse_many().parse($in).unwrap(), $out)
        };
    }

    #[test]
    fn parse_one_expr() {
        parse_once_eq!("1" => Ast::Int(1));
        parse_once_eq!("23" => Ast::Int(23));
    }

    #[test]
    fn parse_one_negated() {
        parse_once_eq!("-1" => Ast::Negate(Box::new(Ast::Int(1))))
    }

    #[test]
    fn parse_group_negated() {
        parse_once_eq!("-(1 + 3)" => Ast::Negate(Box::new(Ast::Add(Box::new(Ast::Int(1)), Box::new(Ast::Int(3))))))
    }

    #[test]
    fn parse_one_binop() {
        parse_once_eq!("1 + 1" => Ast::Add(Box::new(Ast::Int(1)), Box::new(Ast::Int(1)),));
        parse_once_eq!("1 - 1" => Ast::Sub(Box::new(Ast::Int(1)), Box::new(Ast::Int(1)),));
        parse_once_eq!("1 * 1" => Ast::Mul(Box::new(Ast::Int(1)), Box::new(Ast::Int(1)),));
        parse_once_eq!("1 / 1" => Ast::Div(Box::new(Ast::Int(1)), Box::new(Ast::Int(1)),));
        parse_once_eq!("1 ^ 1" => Ast::Pow(Box::new(Ast::Int(1)), Box::new(Ast::Int(1)),));
    }

    #[test]
    fn parse_many_int() {
        parse_many_eq!(
            "1 2 3" =>
            vec![
                Ast::Int(1),
                Ast::Int(2),
                Ast::Int(3),
            ]
        )
    }

    #[test]
    fn parse_many_binop() {
        parse_many_eq!(
            "1 + 3
             2 / 3" =>
            vec![
                Ast::Add(Box::new(Ast::Int(1)), Box::new(Ast::Int(3))),
                Ast::Div(Box::new(Ast::Int(2)), Box::new(Ast::Int(3)))
            ]
        )
    }

    #[test]
    fn parse_nested_binop() {
        parse_many_eq!(
            "2 / 8 - 4" =>
            vec![Ast::Sub(Box::new(Ast::Div(Box::new(Ast::Int(2)), Box::new(Ast::Int(8)))), Box::new(Ast::Int(4)))]
        )
    }

    #[test]
    fn parse_bracket() {
        parse_many_eq!(
            "2 / (8 - 4)" =>
            vec![Ast::Div(Box::new(Ast::Int(2)), Box::new(Ast::Sub(Box::new(Ast::Int(8)), Box::new(Ast::Int(4)))))]
        )
    }
}
