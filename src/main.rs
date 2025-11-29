use std::cell::RefCell;
use std::rc::Rc;

use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

use crate::ast::Expr;
use crate::env::{Env, Value};

mod ast;
mod env;

fn main() -> rustyline::Result<()> {
    let env = Rc::new(RefCell::new(Env::default()));
    let mut rl = DefaultEditor::new()?;
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                match Expr::parse(line) {
                    Ok(expr) => match eval(&expr, env.clone()) {
                        Ok(output) => println!("{}", output),
                        Err(err) => println!("eval error: {}", err),
                    },
                    Err(err) => println!("parse error: {}", err),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}

fn eval(expr: &Expr, mut env: Rc<RefCell<Env>>) -> Result<Value, String> {
    match expr {
        Expr::Atom(symbol) => env.borrow().get(symbol),
        Expr::List(exprs) if exprs.is_empty() => Ok(Value::Nil),
        Expr::List(exprs) => {
            let value = eval(&exprs[0], env.clone())?;
            match value {
                Value::Def => {
                    if exprs.len() == 3 {
                        let key = exprs[1].to_string();
                        let val = eval(&exprs[2], env.clone())?;
                        env.borrow_mut().set(&key, val.clone());
                        Ok(val)
                    } else {
                        return Err(format!("def! requires 2 arguments"));
                    }
                }
                Value::Let => {
                    if exprs.len() == 3 {
                        if let Expr::List(keypairs) = &exprs[1]
                            && keypairs.len() % 2 == 0
                        {
                            env = Rc::new(RefCell::new(Env::new(Some(env))));
                            for keypair in keypairs.chunks(2) {
                                let key = keypair[0].to_string();
                                let value = eval(&keypair[1], env.clone())?;
                                env.borrow_mut().set(&key, value);
                            }
                            let value = eval(&exprs[2], env)?;
                            return Ok(value);
                        } else {
                            return Err(format!(
                                "let* expected key-value pairs got '{}'",
                                exprs[1]
                            ));
                        }
                    } else {
                        return Err(format!("let* requires 2 arguments"));
                    }
                }
                Value::Function(f) => {
                    let mut args = vec![];
                    for expr in &exprs[1..] {
                        let value = eval(expr, env.clone())?;
                        args.push(value);
                    }
                    let res = f(&args)?;
                    return Ok(res);
                }
                _ => Err(format!("unknown symbol '{}'", value)),
            }
        }
    }
}
