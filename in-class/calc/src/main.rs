use std::io::{self, Write};

enum Binop { Plus, Minus, Times, Div }

struct Binexp {
    op: Binop,
    e1: Exp,
    e2: Exp
}

enum Exp {
    EInt(i32),
    ERand, //ERand(n): Random integer 1 (with prob 1/2), 0 (with prob 1/2)
    EBinop(Box<Binexp>)
}

use sexp::*;
//Crate sexp: https://docs.rs/sexp/1.1.4/sexp/index.html
use sexp::Sexp::*;
use sexp::Atom::*;
use self::Binop::*;
use self::Exp::*;

fn parse_sexp(s: &Sexp) -> Result<Exp,String> {
    match s {
        Atom(I(i)) => Ok(EInt(*i as i32)),
        Atom(F(_)) => Err("Floats unsupported".to_string()),
        Atom(S(s)) => {
            match s.trim() {
                "rand" => Ok(ERand),
                _ => Err(format!("Unrecognized operator {}", s))
            }
        },
        List(v) => {
            let e1 = parse_sexp(&v[1])?;
            let e2 = parse_sexp(&v[2])?;            
            match &v[0] {
                Atom(S(s)) => {
                    match s.trim() {
                        "+" => Ok(EBinop(Box::new(Binexp{op: Plus, e1: e1, e2: e2}))),
                        "-" => Ok(EBinop(Box::new(Binexp{op: Minus, e1: e1, e2: e2}))),
                        "*" => Ok(EBinop(Box::new(Binexp{op: Times, e1: e1, e2: e2}))),
                        "/" => Ok(EBinop(Box::new(Binexp{op: Div, e1: e1, e2: e2}))),
                        _ => Err(format!("Unrecognized operator {}", s)),
                    }
                },
                _ => Err(format!("Unexpected SExp {:?}", v[0]))
            }
        }
    }
}

fn parse(s: &str) -> Result<Exp,String> {
    match sexp::parse(s) {
        Ok(se) => parse_sexp(&se),
        Err(err) => Err(format!("Parse error: {}", err))
    }
}

fn eval(e: &Exp) -> i32 {
    match e {
        EInt(i) => *i,
        ERand => {
            let f = rand::random::<f64>(); //random f64 in [0,1]
            if f > 0.5 { 1 }
            else { 0 }
        },
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

fn eval_string(s: &str) -> Result<i32,String> {
    /*match parse(s) {
        Ok(e) => Ok(eval(&e)),
        Err(err) => Err(err) }*/
    let e = parse(s)?;
    Ok(eval(&e))
}

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().expect("Couldn't flush stdout");
        
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("no input");

        println!("{}", buffer.trim());
        match eval_string(&buffer[..]) {
            Ok(r) => println!("result: {:?}", r),
            Err(err) => println!("error: {}", err)
        }
    }
}
