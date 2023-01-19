use std::io;

mod ast;
mod parser;

// Evaluating:
//
// If literal:
// Evaluates to itself
//
// If list:
// Look up first item in list in environment
// If function, run function
// If special form, run special form

fn read() -> io::Result<ast::Ast> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    let buf = buf.trim_end().to_string();

    // TODO: Process input
    let expr = parser::parse_expr(&buf);

    Ok(expr.expect("Parse failed").1)
}

fn eval(input: ast::Ast) -> String {
    // input

    todo!()
}

fn print(input: String) {
    println!("{}", input);
}

fn main() {
    loop {
        let input = read().unwrap();
        let result = eval(input);
        print(result);
    }
}
