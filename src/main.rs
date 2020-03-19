#![allow(unused)]
use std::io;
use std::io::BufRead;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;

// mod lexer;

fn main() {
    let stdin = io::stdin();
    println!("Welcome to rustql!");

    loop {
        let line = read_terminal_line().unwrap_or(String::from(""));

        if line.is_empty() { continue; }

        let tks = lexer::lex(line);

        for t in  tks {
            println!("{}", t);
        }
    }

}

fn read_terminal_line() -> Result<String, ()> {
    let stdin = stdin();
    let mut line = String::new();

    print!("# ");
    stdout().flush().unwrap();

    loop {
        stdin.lock().read_line(&mut line).map_err(|_| ())?;
        if line.len() < 2 {
            return Err(());
        }

        if line.as_bytes()[line.len() - 2] != b'\\' {
            break;
        }

        // Read multiline statements
        line.pop().unwrap();
        line.pop().unwrap();
        line.push('\n');

        print!("| ");
        stdout().flush().unwrap();
    }
    Ok(line)
}
