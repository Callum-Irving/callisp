//! Tokenizes input.

use std::fmt::Debug;

pub struct Token {
    loc: Location,   // where the token is in the file
    kind: TokenKind, // the type of token and type-specific info
}

impl Token {
    pub fn new(line: usize, column: usize, kind: TokenKind) -> Self {
        Self {
            loc: Location { line, column },
            kind,
        }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} : {:?}", self.kind, self.loc)
    }
}

pub struct Location {
    line: usize,
    column: usize,
}

impl Location {
    fn new(line: usize, column: usize) -> Self {
        Location { line, column }
    }
}

impl Debug for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.line, self.column)
    }
}

#[allow(dead_code)]
pub enum TokenKind {
    LParen,             // (
    RParen,             // )
    Plus,               // +
    Minus,              // -
    Times,              // *
    Divide,             // /
    SingleQuote,        // '
    Dot,                // .
    Newline,            // \n or \r\n
    String(String),     // string literal
    Integer(isize),     // an integer literal
    Float(f32),         // a float literal (number ending in 'f')
    Double(f64),        // a double literal (number ending in 'd')
    Boolean(bool),      // 'true' or 'false'
    Identifier(String), // a sequence of characters
    Invalid,            // an invalid token
}

impl Debug for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LParen => write!(f, "("),
            Self::RParen => write!(f, ")"),
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Times => write!(f, "*"),
            Self::Divide => write!(f, "/"),
            Self::SingleQuote => write!(f, "'"),
            Self::Dot => write!(f, "."),
            Self::Newline => write!(f, "newline"),
            Self::String(s) => write!(f, "String({})", s),
            Self::Integer(num) => write!(f, "{}", num),
            Self::Float(num) => write!(f, "{}", num),
            Self::Double(num) => write!(f, "{}", num),
            Self::Boolean(b) => write!(f, "{}", b),
            Self::Identifier(ident) => write!(f, "Ident({})", ident),
            Self::Invalid => write!(f, "invalid"),
        }
    }
}

#[allow(dead_code)]
enum LexerState {
    Whitespace, // consuming whitespace
    Comment,    // consuming a comment
    String,     // consuming a string literal
    Number,     // consuming a number literal
    Identifier, // consuming an identifier or boolean
    Newline,    // consuming \r\n
}

// TODO: Implement error trait
#[allow(dead_code)]
#[derive(Debug)]
pub struct LexError {
    loc: Location,
    kind: LexErrKind,
}

impl LexError {
    fn new(loc: Location, kind: LexErrKind) -> Self {
        LexError { loc, kind }
    }
}

#[derive(Debug)]
enum LexErrKind {
    UnclosedString,
    UnexpectedChar(char),
}

fn is_ident_char(c: char) -> bool {
    !['(', ')', '"', ';'].contains(&c) && !c.is_whitespace()
}

#[allow(dead_code)]
/// Turn string intro vector of tokens.
pub fn tokenize(input: &str) -> Result<Vec<Token>, LexError> {
    // NOTE: Python doesn't keep the whole program in memory, but rather reopens the file when it
    // needs to show an error.

    let mut chars = input.chars().enumerate().peekable();

    // let mut cur = 0; // cursor position
    let mut line = 0; // line number
    let mut bol = 0; // beginning of line

    let mut tokens = vec![];

    'outer: while let Some((cur, c)) = chars.next() {
        match c {
            '(' => tokens.push(Token::new(line, cur - bol, TokenKind::LParen)),
            ')' => tokens.push(Token::new(line, cur - bol, TokenKind::RParen)),
            '\'' => tokens.push(Token::new(line, cur - bol, TokenKind::SingleQuote)),
            '.' => tokens.push(Token::new(line, cur - bol, TokenKind::Dot)),
            '+' => tokens.push(Token::new(line, cur - bol, TokenKind::Plus)),
            '-' => tokens.push(Token::new(line, cur - bol, TokenKind::Minus)),
            '*' => tokens.push(Token::new(line, cur - bol, TokenKind::Times)),
            '/' => tokens.push(Token::new(line, cur - bol, TokenKind::Divide)),
            '\n' => {
                tokens.push(Token::new(line, cur - bol, TokenKind::Newline));
                line += 1;
                bol = cur;
            }
            '"' => {
                // take until unescaped quote
                let mut string = String::new();
                let mut lines_in_str = 0;
                let mut new_bol = bol;

                for (cur2, c) in chars.by_ref() {
                    match c {
                        '"' => {
                            tokens.push(Token::new(line, cur - bol, TokenKind::String(string)));
                            line += lines_in_str;
                            bol = new_bol;
                            continue 'outer;
                        }
                        '\\' => todo!("handle escaped strings"),
                        '\n' => {
                            string.push(c);
                            lines_in_str += 1;
                            new_bol = cur2; // TODO: Rename cur2
                        }
                        _ => {
                            string.push(c);
                        }
                    }
                }
                return Err(LexError::new(
                    Location::new(line, cur - bol),
                    LexErrKind::UnclosedString,
                ));
            }
            '0'..='9' => todo!("tokenize numbers"),
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut ident = c.to_string();
                while let Some(&(_, c)) = chars.peek() {
                    if is_ident_char(c) {
                        chars.next();
                        ident.push(c);
                    } else {
                        break;
                    }
                }

                match ident.as_str() {
                    "true" => tokens.push(Token::new(line, cur - bol, TokenKind::Boolean(true))),
                    "false" => tokens.push(Token::new(line, cur - bol, TokenKind::Boolean(false))),
                    _ => tokens.push(Token::new(line, cur - bol, TokenKind::Identifier(ident))),
                }
            }
            _ if c.is_whitespace() => (),
            _ => {
                return Err(LexError::new(
                    Location::new(line, cur - bol),
                    LexErrKind::UnexpectedChar(c),
                ));
            }
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_s_expr() {
        let input = "((+ a _ab_c) \"string literal\")";
        let toks = tokenize(input).expect("Lexer failed");
        for token in toks {
            println!("{:?}", token);
        }
    }
}
