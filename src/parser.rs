//! Contains parser created using the nom crate.

use crate::ast;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::character::complete::{char, multispace0, multispace1, satisfy};
use nom::combinator::{map, map_res, recognize};
use nom::multi::separated_list0;
use nom::number::complete::recognize_float;
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

/// Parse a lisp expression.
pub fn parse_expr(input: &str) -> IResult<&str, ast::Ast> {
    // skip whitespace
    // if first char == '(', call parse_list
    // if first char =='[', parse vec
    // else parse atom

    preceded(multispace0, alt((parse_list, parse_atom)))(input)
}

fn parse_list(input: &str) -> IResult<&str, ast::Ast> {
    // whitespace separated expressions
    map(
        delimited(
            terminated(char('('), multispace0),
            separated_list0(multispace1, parse_expr),
            preceded(multispace0, char(')')),
        ),
        ast::Ast::List,
    )(input)
}

fn parse_atom(input: &str) -> IResult<&str, ast::Ast> {
    alt((parse_num, parse_string, parse_bool, parse_symbol))(input)
}

fn parse_num(input: &str) -> IResult<&str, ast::Ast> {
    // TODO: Use some sort of multi-precision number instead of f64
    map(
        map_res(recognize_float, |s: &str| s.parse::<f64>()),
        |num| ast::Ast::Atom(ast::LispAtom::Number(num)),
    )(input)
}

fn parse_string(input: &str) -> IResult<&str, ast::Ast> {
    map(
        delimited(char('"'), take_while(|c| c != '"'), char('"')),
        |s: &str| ast::Ast::Atom(ast::LispAtom::String(s.to_string())),
    )(input)
}

fn parse_bool(input: &str) -> IResult<&str, ast::Ast> {
    map(
        alt((map(tag("true"), |_| true), map(tag("false"), |_| false))),
        |b: bool| ast::Ast::Atom(ast::LispAtom::Bool(b)),
    )(input)
}

fn parse_symbol(input: &str) -> IResult<&str, ast::Ast> {
    map(
        recognize(tuple((
            satisfy(|c| is_symbol_character(c) && !c.is_ascii_digit()),
            take_while(is_symbol_character),
        ))),
        |s: &str| ast::Ast::Atom(ast::LispAtom::Symbol(s.to_string())),
    )(input)
}

fn is_symbol_character(c: char) -> bool {
    c != '(' && c != ')' && c != '"' && c != ';' && !c.is_whitespace()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_symbol_works() {
        parse_symbol("1").expect_err("parsed '1' as symbol");

        let (_, ast) = parse_symbol("a123-five?").expect("parse symbol failed");
        let expected = ast::Ast::Atom(ast::LispAtom::Symbol("a123-five?".to_string()));
        assert_eq!(ast, expected);
    }

    #[test]
    fn parse_atom_works() {
        let (_, ast) = parse_atom("1").expect("parse atom failed");
        let expected = ast::Ast::Atom(ast::LispAtom::Number(1.0));
        assert_eq!(ast, expected);

        let (_, ast) = parse_atom("1E10").expect("parse atom failed");
        let expected = ast::Ast::Atom(ast::LispAtom::Number(1E10));
        assert_eq!(ast, expected);
    }

    #[test]
    fn parse_list_works() {
        let (_, ast) = parse_list("(1 2\n)").expect("parse list failed");
        let expected = ast::Ast::List(vec![
            ast::Ast::Atom(ast::LispAtom::Number(1.0)),
            ast::Ast::Atom(ast::LispAtom::Number(2.0)),
        ]);
        assert_eq!(ast, expected);
    }

    #[test]
    fn parse_expr_works() {
        let (_, ast) = parse_expr(" (one   two (f\n3)\n)").expect("parse expr failed");
        let expected = ast::Ast::List(vec![
            ast::Ast::Atom(ast::LispAtom::Symbol("one".to_string())),
            ast::Ast::Atom(ast::LispAtom::Symbol("two".to_string())),
            ast::Ast::List(vec![
                ast::Ast::Atom(ast::LispAtom::Symbol("f".to_string())),
                ast::Ast::Atom(ast::LispAtom::Number(3.0)),
            ]),
        ]);
        assert_eq!(ast, expected);
    }
}
