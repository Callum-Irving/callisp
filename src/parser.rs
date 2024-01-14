//! Contains parser created using the nom crate.

use crate::ast;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::character::complete::{char, digit1, multispace0, multispace1, satisfy};
use nom::combinator::{cut, map, opt, recognize};
use nom::multi::separated_list0;
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

/// Parse a lisp expression. input must only contain the expression and nothing else (except for
/// whitespace). Used only in REPL.
pub fn parse_complete_expr(input: &str) -> IResult<&str, ast::Ast> {
    let (remaining, ast) = parse_expr(input)?;
    if !remaining.is_empty() {
        IResult::Err(nom::Err::Error(nom::error::Error {
            input,
            code: nom::error::ErrorKind::Complete,
        }))
    } else {
        IResult::Ok((remaining, ast))
    }
}

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
    alt((
        parse_float,
        parse_int,
        parse_string,
        parse_bool,
        parse_symbol,
    ))(input)
}

fn recognize_float_exponent(input: &str) -> IResult<&str, &str> {
    recognize(tuple((
        alt((char('e'), char('E'))),
        opt(alt((char('+'), char('-')))),
        cut(digit1),
    )))(input)
}

fn parse_float(input: &str) -> IResult<&str, ast::Ast> {
    let res: IResult<&str, &str> = recognize(tuple((
        opt(alt((char('+'), char('-')))),
        digit1,
        alt((
            map(
                tuple((char('.'), opt(digit1), opt(recognize_float_exponent))),
                |_| (),
            ),
            map(recognize_float_exponent, |_| ()),
        )),
    )))(input);
    let (remaining, num_str) = res?;
    let num = num_str.parse::<f64>().unwrap();
    Ok((remaining, ast::Ast::Atom(ast::LispAtom::Float(num))))
}

fn parse_int(input: &str) -> IResult<&str, ast::Ast> {
    let (remaining, num_str) = recognize(tuple((opt(alt((char('+'), char('-')))), digit1)))(input)?;
    let num = num_str.parse::<i64>().unwrap();
    Ok((remaining, ast::Ast::Atom(ast::LispAtom::Int(num))))
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
        let expected = ast::Ast::Atom(ast::LispAtom::Int(1));
        assert_eq!(ast, expected);

        let (_, ast) = parse_atom("1E10").expect("parse atom failed");
        let expected = ast::Ast::Atom(ast::LispAtom::Float(1E10));
        assert_eq!(ast, expected);
    }

    #[test]
    fn parse_list_works() {
        let (_, ast) = parse_list("(1 2\n)").expect("parse list failed");
        let expected = ast::Ast::List(vec![
            ast::Ast::Atom(ast::LispAtom::Int(1)),
            ast::Ast::Atom(ast::LispAtom::Int(2)),
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
                ast::Ast::Atom(ast::LispAtom::Int(3)),
            ]),
        ]);
        assert_eq!(ast, expected);
    }
}
