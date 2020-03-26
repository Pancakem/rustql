use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Clone, Copy, Debug)]
struct Cursor {
    pointer: u8,
    loc: Location,
}

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
    Unknown,
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

type Lexer = fn(source: String, cursor: &mut Cursor) -> Result<Token, String>;

pub fn lex(source: String) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut cursor = Cursor {
        pointer: 0u8,
        loc: Location {
            line: 0u8,
            col: 0u8,
        },
    };

    'outer: while cursor.pointer < source.len() as u8 {
        let lexers: Vec<Lexer> = vec![
            lex_keyword,
            lex_symbol,
            lex_string,
            lex_numeric,
            lex_identifier,
        ];
        for l in lexers {
            match l(source.clone(), &mut cursor) {
                Ok(x) => {
                    tokens.push(x);
                }
                Err(e) => {
                    println!("{}", e);
                    break 'end;
                }
            };
            continue 'outer;
        }

        let mut hint: String;
        if tokens.len() > 0 {
            hint.push_str(" after ");
            hint.push_str(&tokens[tokens.len() - 1].value[..]);
        }

        'end: return Err(format!("Unable to lex tokens{}, at  {}:{}", hint, cursor.loc.line, cursor.loc.col);

    }
    Ok(tokens)
}

fn lex_string(source: String, cursor: &mut Cursor) -> Result<Token, String> {
    Err("to do".to_string())
}

fn lex_identifier(source: String, cursor: &mut Cursor) -> Result<Token, String> {
    Err("to do".to_string())
}

fn lex_numeric(source: String, cursor: &mut Cursor) -> Result<Token, String> {
    let mut period_found = false;
    let mut exp_marker_found = false;

    let start_loc = cursor.loc;
    let start_pos = cursor.pointer;
    while cursor.pointer < source.len() as u8 {
        let mut opt_c = source.chars().nth(cursor.pointer as usize);
        cursor.loc.col += 1;
        let c = match opt_c {
            None => ' ',
            Some(x) => x,
        };

        let is_period = c == '.';
        // check exponential
        let is_exp = c == 'e';

        // must start with digit or period
        if start_pos == cursor.pointer {
            if !c.is_digit(10) && !is_period {
                return Err("unexpected token".to_string());
            }

            period_found = is_period;
            continue;
        }

        if is_period {
            if period_found {
                return Err("unexpected token".to_string());
            }

            period_found = false;
            continue;
        }

        if is_exp {
            if exp_marker_found {
                return Err("unexpected token".to_string());
            }

            // no periods allowed after exponential
            period_found = true;
            exp_marker_found = true;

            // exponential must be followed by digits
            if cursor.pointer == (source.len() - 1) as u8 {
                return Err("unexpected token".to_string());
            }

            opt_c = source.chars().nth((cursor.pointer + 1) as usize);
            let c_next = match opt_c {
                None => ' ',
                Some(x) => x,
            };

            if c_next == '-' || c_next == '+' {
                cursor.pointer += 1;
                cursor.loc.col += 1;
            }

            continue;
        }
        if !c.is_digit(10) {
            return Err("unexpected token".to_string());
        }
        cursor.pointer += 1;
    }
    return Ok(Token {
        value: source[start_pos as usize..cursor.pointer as usize].to_string(),
        location: start_loc,
        kind: TokenKind::Numeric,
    });
}

fn lex_symbol(source: String, cursor: &mut Cursor) -> Result<Token, String> {
    let mut opt_c = source.chars().nth(cursor.pointer as usize);
    let start_loc = cursor.loc;
    cursor.loc.col += 1;
    cursor.pointer += 1;
    let c = match opt_c {
        None => ' ',
        Some(x) => x,
    };

    if c == '\n' {
        cursor.loc.line += 1;
        cursor.loc.col = 0;
    } else if c == ';' || c == ',' || c == '(' || c == ')' || c == '\'' || c == '=' {
    } else if c == ' ' {
        return Ok(empty_token());
    } else if c == '*' {
        return Err("unexpected token".to_string());
    } else {
        return Err("unknown token".to_string());
    }

    Ok(Token {
        value: c.to_string(),
        location: cursor.loc,
        kind: TokenKind::Symbol,
    })
}

fn lex_keyword(source: String, cursor: &mut Cursor) -> Result<Token, String> {
    let s_slice: &str = &source[..].to_ascii_lowercase();

    match s_slice {
        "select" | "from" | "as" | "table" | "create" | "insert" | "into" | "values" | "int"
        | "text" | "where" => true,

        _ => false,
    };

    Err("to do".to_string())
}

fn empty_token() -> Token {
    Token {
        kind: TokenKind::Unknown,
        value: "".to_string(),
        location: Location {
            line: 0u8,
            col: 0u8,
        },
    }
}
