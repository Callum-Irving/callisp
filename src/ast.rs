/// AST
pub type Ast = Vec<Expr>;

pub enum Atom {
    Symbol(String),
    Number(f64),
}

pub type List = Vec<Expr>;

pub enum Expr {
    Atom(Atom),
    List(List),
}
