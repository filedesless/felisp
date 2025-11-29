use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

use crate::ast::Expr;

pub type Callback = Rc<dyn Fn(&[Value]) -> Result<Value, String>>;

#[derive(Clone)]
pub enum Value {
    // nil, ()
    Nil,
    True,
    False,
    // special forms
    Def,
    Let,
    Do,
    If,
    Fn,
    Quote,
    // data types
    Number(i64),
    Quoted(Expr),
    Function(Callback),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::True => write!(f, "true"),
            Value::False => write!(f, "false"),
            Value::Def => write!(f, "def!"),
            Value::Let => write!(f, "let*"),
            Value::If => write!(f, "if"),
            Value::Do => write!(f, "do"),
            Value::Quote => write!(f, "quote"),
            Value::Fn => write!(f, "fn*"),
            Value::Number(n) => write!(f, "{}", n),
            Value::Quoted(expr) => write!(f, "{}", expr),
            Value::Function(_) => write!(f, "<fun>"),
        }
    }
}

pub struct Env {
    data: RefCell<HashMap<String, Value>>,
    outer: Option<Rc<Env>>,
}

impl Env {
    pub fn new(outer: Option<Rc<Env>>) -> Self {
        Self {
            data: RefCell::new(HashMap::new()),
            outer,
        }
    }
    pub fn get(&self, symbol: &str) -> Result<Value, String> {
        let val = match symbol {
            "nil" => Value::Nil,
            "true" => Value::True,
            "false" => Value::False,
            "def!" => Value::Def,
            "let*" => Value::Let,
            "do" => Value::Do,
            "if" => Value::If,
            "quote" => Value::Quote,
            "fn*" => Value::Fn,
            _ => {
                if let Ok(n) = symbol.parse::<i64>() {
                    Value::Number(n)
                } else {
                    self.get_from_map(symbol)?
                }
            }
        };
        Ok(val)
    }

    fn get_from_map(&self, symbol: &str) -> Result<Value, String> {
        if let Some(value) = self.data.borrow().get(symbol) {
            return Ok(value.clone());
        }
        if let Some(outer) = &self.outer {
            return outer.get_from_map(symbol);
        }
        Err(format!("unknown symbol '{}'", symbol))
    }

    pub fn set(&self, symbol: &str, value: Value) {
        self.data.borrow_mut().insert(symbol.to_string(), value);
    }
}

impl Default for Env {
    fn default() -> Self {
        Self {
            data: RefCell::new(HashMap::from([
                ("+".to_string(), Value::Function(Rc::new(add))),
                ("-".to_string(), Value::Function(Rc::new(sub))),
                ("*".to_string(), Value::Function(Rc::new(mul))),
                ("<=".to_string(), Value::Function(Rc::new(leq))),
            ])),
            outer: None,
        }
    }
}

fn add(args: &[Value]) -> Result<Value, String> {
    let mut total = 0;
    for arg in args {
        if let Value::Number(n) = arg {
            total += n;
        } else {
            return Err(format!("invalid type expected Number but got '{}'", arg));
        }
    }
    Ok(Value::Number(total))
}

fn sub(args: &[Value]) -> Result<Value, String> {
    if let Value::Number(x) = args[0] {
        if let Value::Number(y) = args[1] {
            return Ok(Value::Number(x - y));
        }
    }
    Err(format!(
        "invalid type expected Numbers but got '{}, {}'",
        args[0], args[1]
    ))
}

fn mul(args: &[Value]) -> Result<Value, String> {
    let mut total = 1;
    for arg in args {
        if let Value::Number(n) = arg {
            total *= n;
        } else {
            return Err(format!("invalid type expected Number but got '{}'", arg));
        }
    }
    Ok(Value::Number(total))
}

fn leq(args: &[Value]) -> Result<Value, String> {
    if let Value::Number(x) = args[0] {
        if let Value::Number(y) = args[1] {
            return Ok(if x <= y { Value::True } else { Value::False });
        }
    }
    Err(format!(
        "invalid type expected Numbers but got '{}, {}'",
        args[0], args[1]
    ))
}
