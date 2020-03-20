use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Clone, Copy, Debug)]
struct Location {
    line: u8,
    col: u8,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum TokenKind {
    Keyword,
    Symbol,
    Identifier,
    String,
    Numeric,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Token {
    value: String,
    kind: TokenKind,
    location: Location,
}

pub fn equal(t: Token, other: Token) -> bool {
    t.value == other.value && t.kind == other.kind
}

pub fn lex(source: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut col = 0;
    let mut ln = 1;
    let mut tok = Token {
        value: "".to_string(),
        kind: TokenKind::String,
        location: Location {
            line: 0u8,
            col: 0u8,
        },
    };
    // let mut string_token: String;

    for c in source.chars() {
        tok.location.line = ln;
        if c == '\n' {
            ln = ln + 1;
            col = 0;
            continue;
        } else if c == ';' || c == ',' || c == '(' || c == ')' {
            col = col + 1;
            tok.value.push(c);
            tok.location.col = col;
        } else {
            col = col + 1;
            tok.value.push(c);
            tok.location.col = col;
            tok.location.line = ln;
        }

        finalize(&mut tok);

        tokens.push(tok.clone());
    }

    return tokens;
}

fn finalize(t: &mut Token) -> bool {
    if finalize_symbol(t) {
        return true;
    } else if finalize_keyword(t) {
        return true;
    } else if finalize_numeric(t) {
        return true;
    }
    false
}

fn finalize_string(t: &mut Token) -> bool {
    if t.value.len() == 0 {
        return false;
    }
    // what??????
    t.kind = TokenKind::String;
    true
}

fn finalize_identifier(t: &mut Token) -> bool {
    {
        t.kind = TokenKind::Identifier;
        true
    }
}

fn finalize_numeric(t: &mut Token) -> bool {
    if t.value.len() == 0 {
        return false;
    }

    let mut period_found = false;
    let mut exp_marker_found = false;

    let mut i: usize = 0;
    while i < t.value.len() {
        let mut opt_c = t.value.chars().nth(i);
        let c = match opt_c {
            None => ' ',
            Some(x) => x,
        };

        let is_period = c == '.';
        // check exponential
        let is_exp = c == 'e';

        // must start with digit or period
        if i == 0 {
            if !c.is_digit(10) && !is_period {
                return false;
            }

            period_found = is_period;
            i += 1;
            continue;
        }

        if is_period {
            if period_found {
                return false;
            }

            period_found = false;
            i += 1;
            continue;
        }

        if is_exp {
            if exp_marker_found {
                return false;
            }

            // no periods allowed after exponential
            period_found = true;
            exp_marker_found = true;

            // exponential must be followed by digits
            if i == t.value.len() - 1 {
                return false;
            }

            opt_c = t.value.chars().nth(i + 1);
            let c_next = match opt_c {
                None => ' ',
                Some(x) => x,
            };

            if c_next == '-' || c_next == '+' {
                i += 1;
            }

            i += 1;
            continue;
        }
        if !c.is_digit(10) {
            return false;
        }
        i += 1;
    }

    t.kind = TokenKind::Numeric;
    return true;
}

fn finalize_symbol(t: &mut Token) -> bool {
    let s_slice: &str = &t.value[..];
    match s_slice {
        "*" | ";" | "(" | ")" => {
            t.kind = TokenKind::Symbol;
            true
        }
        _ => false,
    }
}

fn finalize_keyword(t: &mut Token) -> bool {
    let s_slice: &str = &t.value[..].to_ascii_lowercase();

    match s_slice {
        "select" | "from" | "as" | "table" | "create" | "insert" | "into" | "values" | "int"
        | "text" => {
            t.kind = TokenKind::Keyword;
            true
        }
        _ => false,
    }
}
