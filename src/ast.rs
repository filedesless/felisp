use std::{fmt, iter::Peekable, str::Chars};

#[derive(Debug)]
pub enum Expr {
    Atom(String),
    List(Vec<Expr>),
}

impl Expr {
    pub fn parse(source: String) -> Result<Expr, String> {
        let mut chars = source.chars().peekable();
        let expr = parse_expression(&mut chars)?;
        if !chars.peek().is_none() {
            return Err("Unexpected EOF".to_string());
        }
        Ok(expr)
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Atom(s) => write!(f, "{}", s),
            Expr::List(exprs) => {
                write!(f, "(")?;
                let n = exprs.len();
                for expr in &exprs[..n - 1] {
                    write!(f, "{} ", expr)?;
                }
                write!(f, "{})", exprs[n - 1])
            }
        }
    }
}

fn parse_expression(chars: &mut Peekable<Chars>) -> Result<Expr, String> {
    while let Some(_) = chars.next_if(|&c| c.is_whitespace()) {}
    let out = match chars.peek() {
        Some('(') => parse_list(chars),
        _ => parse_atom(chars),
    };
    while let Some(_) = chars.next_if(|&c| c.is_whitespace()) {}
    out
}

fn parse_atom(chars: &mut Peekable<Chars>) -> Result<Expr, String> {
    let mut result = String::new();
    while let Some(c) = chars.next_if(|&c| c != ')' && c.is_ascii_graphic()) {
        result.push(c);
    }
    if result.is_empty() {
        return Err("empty atom".to_string());
    }
    Ok(Expr::Atom(result))
}

fn parse_list(chars: &mut Peekable<Chars>) -> Result<Expr, String> {
    chars.next_if_eq(&'(').ok_or("parse_list expected '('")?;
    let mut result = vec![];
    while chars.peek().is_some_and(|&c| c != ')') {
        let expr = parse_expression(chars)?;
        result.push(expr);
    }
    chars.next_if_eq(&')').ok_or("parse_list expected ')'")?;

    Ok(Expr::List(result))
}
