use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

pub type Callback = Rc<dyn Fn(&[Value]) -> Result<Value, String>>;

#[derive(Clone)]
pub enum Value {
    Nil,
    Def,
    Let,
    Number(u64),
    Function(Callback),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Def => write!(f, "def!"),
            Value::Let => write!(f, "let*"),
            Value::Number(n) => write!(f, "{}", n),
            Value::Function(_) => write!(f, "<fun>"),
        }
    }
}

pub struct Env {
    data: HashMap<String, Value>,
    outer: Option<Rc<RefCell<Env>>>,
}

impl Env {
    pub fn new(outer: Option<Rc<RefCell<Env>>>) -> Self {
        Self {
            data: HashMap::new(),
            outer,
        }
    }
    pub fn get(&self, symbol: &str) -> Result<Value, String> {
        if symbol == "nil" {
            return Ok(Value::Nil);
        }
        if symbol == "def!" {
            return Ok(Value::Def);
        }
        if symbol == "let*" {
            return Ok(Value::Let);
        }
        if let Ok(n) = symbol.parse::<u64>() {
            return Ok(Value::Number(n));
        }
        self.get_from_map(symbol)
    }

    fn get_from_map(&self, symbol: &str) -> Result<Value, String> {
        if let Some(value) = self.data.get(symbol) {
            return Ok(value.clone());
        }
        if let Some(outer) = &self.outer {
            return outer.borrow().get_from_map(symbol);
        }
        Err(format!("unknown symbol '{}'", symbol))
    }

    pub fn set(&mut self, symbol: &str, value: Value) {
        self.data.insert(symbol.to_string(), value);
    }
}

impl Default for Env {
    fn default() -> Self {
        Self {
            data: HashMap::from([
                ("nil".to_string(), Value::Nil),
                ("+".to_string(), Value::Function(Rc::new(add))),
                ("*".to_string(), Value::Function(Rc::new(mul))),
            ]),
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
