use crate::ast;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::character::complete::{char, digit1, multispace0, multispace1};
use nom::combinator::{map, map_res};
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::IResult;

pub fn parse_expr(input: &str) -> IResult<&str, ast::Ast> {
    // skip whitespace
    // if first char == '(', call parse_list
    // if first char =='[', parse vec
    // else parse atom

    delimited(multispace0, alt((parse_list, parse_atom)), multispace0)(input)
}

fn parse_list(input: &str) -> IResult<&str, ast::Ast> {
    // whitespace separated expressions
    map(
        // multispace1
        delimited(char('('), separated_list0(tag(" "), parse_atom), char(')')),
        |exprs| ast::Ast::List(exprs),
    )(input)
}

fn parse_atom(input: &str) -> IResult<&str, ast::Ast> {
    alt((parse_num, parse_symbol))(input)
}

fn parse_num(input: &str) -> IResult<&str, ast::Ast> {
    // digit1 + opt(seq('.' digit1))
    map(
        map_res(alt((digit1, multispace1)), |s: &str| s.parse::<f64>()),
        |num| ast::Ast::Atom(ast::LispAtom::Number(num)),
    )(input)
}

fn parse_symbol(input: &str) -> IResult<&str, ast::Ast> {
    // TODO: Make sure first character isn't a digit
    map(take_while(is_symbol_character), |s: &str| {
        ast::Ast::Atom(ast::LispAtom::Symbol(s.to_string()))
    })(input)
}

fn is_symbol_character(c: char) -> bool {
    c != '(' && c != ')' && c != '"' && c != ';' && !c.is_whitespace()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_atom_works() {
        let (_, ast) = parse_atom("1").expect("parse atom failed");
        let expected = ast::Ast::Atom(ast::LispAtom::Number(1.0));
        assert_eq!(ast, expected);
    }

    #[test]
    fn parse_list_works() {
        let (_, ast) = parse_list("(1 2)").expect("parse list failed");
        let expected = ast::Ast::List(vec![
            ast::Ast::Atom(ast::LispAtom::Number(1.0)),
            ast::Ast::Atom(ast::LispAtom::Number(2.0)),
        ]);
        assert_eq!(ast, expected);
    }

    #[test]
    fn parse_expr_works() {
        let (_, ast) = parse_expr("(one two 3)").expect("parse expr failed");
        let expected = ast::Ast::List(vec![
            ast::Ast::Atom(ast::LispAtom::Symbol("one".to_string())),
            ast::Ast::Atom(ast::LispAtom::Symbol("two".to_string())),
            ast::Ast::Atom(ast::LispAtom::Number(3.0)),
        ]);
        assert_eq!(ast, expected);
    }
}
