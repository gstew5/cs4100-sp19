extern crate regex;
use std::fs;
use std::env;

#[allow(dead_code)]
mod lexer;
use lexer::{LexerState,Tok};

#[allow(dead_code)]
mod types;
use types::{Interp,VM};

#[allow(dead_code)]
mod parser;
use parser::{parse};

#[allow(dead_code)]
mod compile;
use compile::{compile};

fn main() -> Result<(), String> {
    let file = env::args().last().expect("cargo run file");
    let buf = fs::read_to_string(&file).expect(&format!("main: couldn't read {}", file));
    println!("tokens are:");
    let mut l = LexerState::new(&buf);
    loop {
        if let Some(tok) = l.next() {
            println!("{:?}", tok);            
            match tok {
                Tok::DOLLAR => break,
                _ => {}
            }
        } else { break }
    }

    match parse(&buf) {
        Ok(e) => {
            println!("expression is: {}", e.to_string());
            println!("result is: {}", e.interp().to_string());
            let instrs = compile(&e);
            println!("instructions are: {:?}", instrs);
            let mut vm = VM::init(&instrs);
            let res = vm.run();
            println!("VM result is: {:?}", res);
            Ok(())
        },
        Err(err) => Err(err)            
    }
}
