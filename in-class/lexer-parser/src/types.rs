use std::string::{ToString};

/********************************************
 * Expression language
 ********************************************/

/*
trait ToString {
    fn show(&self) -> String;
}
*/
    
#[derive(Debug,Clone)]
pub enum Binop {
    BPlus,
    BTimes,
}

use types::Binop::*;

impl ToString for Binop {
    fn to_string(&self) -> String {
        match self {
            BPlus => "+".to_string(),
            BTimes => "*".to_string()
        }
    }
}

#[derive(Debug,Clone)]
pub struct Binexp {
    pub op: Binop,
    pub lhs: Exp,
    pub rhs: Exp
}

impl Binexp {
    pub fn interp(&self) -> i32 {
        match self.op {
            BPlus => self.lhs.interp() + self.rhs.interp(),
            BTimes => self.lhs.interp() * self.rhs.interp(),
        }
    }
}

impl ToString for Binexp {
    fn to_string(&self) -> String {
        format!("({} {} {})", self.lhs.to_string(), self.op.to_string(), self.rhs.to_string())
    }
}

#[derive(Debug,Clone)]
pub enum Exp {
    EI32(i32),
    EBinop(Box<Binexp>),
}

use types::Exp::*;

impl Exp {
    pub fn interp(&self) -> i32 {
        match self {
            EI32(i) => *i,
            EBinop(b) => b.interp()
        }
    }
}

impl ToString for Exp {
    fn to_string(&self) -> String {
        match self {
            EI32(i) => i.to_string(),
            EBinop(b) => b.to_string()
        }
    }
}
