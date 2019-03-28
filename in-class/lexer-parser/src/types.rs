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

pub trait Interp {
    fn interp(&self) -> i32;
}

impl Interp for Binexp {
    fn interp(&self) -> i32 {
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

impl Interp for Exp {
    fn interp(&self) -> i32 {
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

#[derive(Debug,Clone)]
pub enum Instr {
    IPlus,
    ITimes,
    II32(i32),
}

use types::Instr::*;

#[derive(Debug)]
pub struct VM {
    pub stack: Vec<i32>,
    pub instrs: Vec<Instr>,
    pub pc: usize
}

impl VM {
    pub fn init(instrs: &[Instr]) -> VM {
        VM {
            stack: vec![],
            instrs: instrs.to_vec(),
            pc: 0
        }            
    }

    pub fn run(&mut self) -> Option<i32> {
        'mainloop:loop {
            if self.pc >= self.instrs.len() { break 'mainloop };
            match self.instrs[self.pc] {
                IPlus => {
                    let v2 = self.stack.pop().expect("IPlus: missing arg v2");
                    let v1 = self.stack.pop().expect("IPlus: missing arg v1");
                    self.stack.push(v1 + v2)
                },
                ITimes => {
                    let v2 = self.stack.pop().expect("ITimes: missing arg v2");
                    let v1 = self.stack.pop().expect("ITimes: missing arg v1");
                    self.stack.push(v1 * v2)
                },
                II32(i) => {
                    self.stack.push(i)
                }
            };
            self.pc = self.pc + 1
        }
        let res = self.stack[self.stack.len() - 1];
        Some(res)
    }
}
