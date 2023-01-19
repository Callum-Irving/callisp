#[derive(Debug, PartialEq)]
pub enum Ast {
    Atom(LispAtom),
    List(Vec<Ast>),
}

#[derive(Debug, PartialEq)]
pub enum LispAtom {
    Symbol(String),
    String(String),
    Number(f64),
}
