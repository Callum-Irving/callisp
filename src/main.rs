use std::io;

mod ast;
mod parser;

fn read() -> io::Result<String> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    let buf = buf.trim_end().to_string();

    // TODO: Process input

    Ok(buf)
}

fn eval(input: String) -> String {
    input
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
