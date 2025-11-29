use std::rc::Rc;

use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

use crate::ast::Expr;
use crate::env::{Callback, Env, Value};

mod ast;
mod env;

fn main() -> rustyline::Result<()> {
    let env = Rc::new(Env::default());
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

fn eval(expr: &Expr, env: Rc<Env>) -> Result<Value, String> {
    match expr {
        Expr::Atom(symbol) => env.get(symbol),
        Expr::List(exprs) if exprs.is_empty() => Ok(Value::Nil),
        Expr::List(exprs) => {
            let first_value = eval(&exprs[0], env.clone())?;
            match first_value {
                Value::Def => eval_def(&env, exprs),
                Value::Let => eval_let(&env, exprs),
                Value::Do => eval_do(&env, exprs),
                Value::If => eval_if(&env, exprs),
                Value::Quote => eval_quote(&env, exprs),
                Value::Fn => eval_fn(&env, exprs),
                Value::Function(f) => eval_function(env, exprs, f),
                _ => Err(format!("unknown symbol '{}'", first_value)),
            }
        }
    }
}

/// updates the current env with (evaluated) expr bound to symbol
/// (def! symbol expr)
fn eval_def(env: &Rc<Env>, exprs: &Vec<Expr>) -> Result<Value, String> {
    if exprs.len() != 3 {
        return Err(format!("def! requires 2 arguments"));
    }

    let key = exprs[1].to_string();
    let val = eval(&exprs[2], env.clone())?;
    env.set(&key, val.clone());
    Ok(val)
}

/// evaluates expr in a new env with (evaluated) vi bound to (symbol) ki
/// (let* (k1 v1 k2 v2 ...) expr)
fn eval_let(env: &Rc<Env>, exprs: &Vec<Expr>) -> Result<Value, String> {
    if exprs.len() != 3 {
        return Err(format!("let* requires 2 arguments"));
    }
    match &exprs[1] {
        Expr::List(keypairs) if keypairs.len() % 2 == 0 => {
            let env = Rc::new(Env::new(Some(env.clone())));
            for keypair in keypairs.chunks(2) {
                let key = keypair[0].to_string();
                let val = eval(&keypair[1], env.clone())?;
                env.set(&key, val.clone());
            }
            Ok(eval(&exprs[2], env)?)
        }
        _ => Err(format!("let* expected key-value pairs got '{}'", exprs[1])),
    }
}

/// evaluates all arguments sequentially, returning the last
/// (do 1 2 3) -> 3
fn eval_do(env: &Rc<Env>, exprs: &Vec<Expr>) -> Result<Value, String> {
    let mut output = Value::Nil;
    for expr in &exprs[1..] {
        output = eval(expr, env.clone())?;
    }
    Ok(output)
}

/// (if cond then else?) -> evaluates cond
/// if it is nil or false, evaluates and returns else (nil if absent)
/// otherwise evaluates and returns then
fn eval_if(env: &Rc<Env>, exprs: &Vec<Expr>) -> Result<Value, String> {
    if exprs.len() <= 2 {
        return Err(format!("if* requires at least 2 arguments"));
    }
    match eval(&exprs[1], env.clone())? {
        Value::Nil | Value::False if exprs.len() == 3 => Ok(Value::Nil),
        Value::Nil | Value::False if exprs.len() == 4 => Ok(eval(&exprs[3], env.clone())?),
        _ if exprs.len() <= 4 => Ok(eval(&exprs[2], env.clone())?),
        _ => Err(format!("if* requires at most 3 arguments")),
    }
}

/// (f x1 x2 ...) -> (apply f x1 x2 ...)
fn eval_function(env: Rc<Env>, exprs: &Vec<Expr>, f: Callback) -> Result<Value, String> {
    let mut args = vec![];
    for expr in &exprs[1..] {
        let value = eval(expr, env.clone())?;
        args.push(value);
    }
    Ok(f(&args)?)
}

/// (quote a b) -> (a b)
fn eval_quote(_env: &Env, exprs: &[Expr]) -> Result<Value, String> {
    Ok(Value::Quoted(Expr::List(Vec::from(&exprs[1..]))))
}

/// returns a lambda that once called, evaluates the body with the given arguments
/// (fn* (a) a) -> `<fun>`
/// ((fn* (a b) (+ a b)) 2 3) -> 5
fn eval_fn(env: &Rc<Env>, exprs: &Vec<Expr>) -> Result<Value, String> {
    if exprs.len() != 3 {
        return Err(format!("fn* requires 2 arguments"));
    }
    if let Expr::List(bindings) = &exprs[1] {
        let cloned_env = env.clone();
        let cloned_bindings = bindings.clone();
        let cloned_body = exprs[2].clone();
        let cb: Callback = Rc::new(move |args| {
            if args.len() != cloned_bindings.len() {
                return Err(format!(
                    "fn required {} args but given {}",
                    cloned_bindings.len(),
                    args.len()
                ));
            }
            let env = Rc::new(Env::new(Some(cloned_env.clone())));
            for (binding, arg) in cloned_bindings.iter().zip(args) {
                let key = binding.to_string();
                env.set(&key, arg.clone());
            }
            let output = eval(&cloned_body, env)?;
            Ok(output)
        });
        Ok(Value::Function(cb))
    } else {
        Err(format!("fn* expected bindings got '{}'", exprs[1]))
    }
}
