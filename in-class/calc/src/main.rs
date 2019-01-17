use std::io::{self, Write};

enum Binop { Plus, Minus, Times, Div }

struct Binexp {
    op: Binop,
    e1: Exp,
    e2: Exp
}

enum Exp {
    EInt(i32),
    EBinop(Box<Binexp>)
}

use sexp::*;
//Crate sexp: https://docs.rs/sexp/1.1.4/sexp/index.html
use sexp::Sexp::*;
use sexp::Atom::*;
use self::Binop::*;
use self::Exp::*;

fn parse_sexp(s: &Sexp) -> Exp {
    match s {
        Atom(I(i)) => EInt(*i as i32),
        Atom(F(_)) => panic!("Floats unsupported"),
        Atom(S(s)) => panic!(format!("Unrecognized operator {}", s)),
        List(v) => {
            let e1 = parse_sexp(&v[1]);
            let e2 = parse_sexp(&v[2]);            
            match &v[0] {
                Atom(S(s)) => {
                    match s.trim() {
                        "+" => EBinop(Box::new(Binexp{op: Plus, e1: e1, e2: e2})),
                        "-" => EBinop(Box::new(Binexp{op: Minus, e1: e1, e2: e2})),
                        "*" => EBinop(Box::new(Binexp{op: Times, e1: e1, e2: e2})),
                        "/" => EBinop(Box::new(Binexp{op: Div, e1: e1, e2: e2})),
                        _ => panic!(format!("Unrecognized operator {}", s)),
                    }
                },
                _ => panic!(format!("Unexpected SExp {:?}", v[0]))
            }
        }
    }
}

fn parse(s: &str) -> Exp {
    match sexp::parse(s) {
        Ok(se) => parse_sexp(&se),
        Err(err) => panic!(format!("Parse error: {}", err))
    }
}

fn eval(e: &Exp) -> i32 {
    match e {
        EInt(i) => *i,
        EBinop(b) => {
            let n1 = eval(&b.e1);
            let n2 = eval(&b.e2);
            match b.op {
                Plus => n1 + n2,
                Minus => n1 - n2,
                Times => n1 * n2,
                Div => n1 / n2
            }
        },
    }
}

fn eval_string(s: &str) -> i32 {
    eval(&parse(s))
}

fn main() {
    println!("Hello, world!");
    let prog1 = "(+ 3 (+ 4 (* 1 5)))";
    println!("eval[{}] = {}", prog1, eval_string(prog1));
    
    loop {
        print!("> ");
        io::stdout().flush();
        
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("no input");

        println!("{}", buffer.trim());
        println!("result: {}", eval_string(&buffer[..]));
    }
}
