use std::io::{self, Write};

use crate::ast::Expr;

mod ast;

fn read() -> Result<Expr, String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    Expr::parse(buffer)
}

fn eval(x: Expr) -> Expr {
    x
}

fn main() -> io::Result<()> {
    loop {
        print!("> ");
        let _ = io::stdout().flush();
        match read() {
            Ok(input) => {
                let output = eval(input);
                println!("{}", output);
            }
            Err(s) => println!("[ERROR] {}", s),
        }
    }
}
