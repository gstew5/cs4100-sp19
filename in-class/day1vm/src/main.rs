use std::ops::{Index,IndexMut};
use std::io;

#[derive(Debug,Clone)]
enum Reg {
    ACC,
    RA,
    RB,
    RC
}

#[derive(Debug)]
struct RegBank {
    acc: i32,
    ra: i32,
    rb: i32,
    rc: i32,
}

impl Index<Reg> for RegBank {
    type Output = i32;
    
    fn index<'a>(&'a self, reg: Reg) -> &'a i32 {
        match reg { 
            Reg::ACC => &self.acc,
            Reg::RA => &self.ra,
            Reg::RB => &self.rb,
            Reg::RC => &self.rc,            
        }
    }
}

impl IndexMut<Reg> for RegBank {
    fn index_mut<'a>(&'a mut self, reg: Reg) -> &'a mut i32 {
        match reg { 
            Reg::ACC => &mut self.acc,
            Reg::RA => &mut self.ra,
            Reg::RB => &mut self.rb,
            Reg::RC => &mut self.rc,            
        }
    }
}

#[derive(Debug,Clone)]
enum Instr {
    Const(Reg, i32), //const rdst, i:    r <- i
    Mov(Reg, Reg),   //mov rdst, rsrc:   rdst <- rsrc
    Br(usize),       //br lbl:           branch to lbl if register acc == 1
    Eq(Reg, Reg),    //eq rdst, rsrc:    rdst <- 1 if rsrc == rdst, otherwise 0
    Sub(Reg, Reg),   //sub rdst, rsrc:   rdst <- rdst - rsrc
    Mul(Reg, Reg),   //mul rdst, rsrc:   rdst <= rdst * rsrc
    Hlt,             //hlt:              halt the machine          
}

#[derive(Debug)]
struct State {
    pc: usize,
    rb: RegBank,
}

fn show_state(s: &State) -> String {
    format!("pc={}, acc={}, ra={}, rb={}, rc={}",
            s.pc, s.rb[Reg::ACC], s.rb[Reg::RA], s.rb[Reg::RB], s.rb[Reg::RC])
}

fn run(s: &mut State, prog: &[Instr]) {
    'mainloop:loop { 
        match prog[s.pc].clone() {
            Instr::Const(rdst, i) => {
                s.rb[rdst] = i;
                s.pc = s.pc + 1
            },
            Instr::Mov(rdst, rsrc) => {
                s.rb[rdst] = s.rb[rsrc];
                s.pc = s.pc + 1
            },
            Instr::Br(lbl) => {
                if s.rb[Reg::ACC] == 1 { s.pc = lbl }
                else { s.pc = s.pc + 1 }
            },
            Instr::Eq(rdst, rsrc) => {
                if s.rb[rsrc] == s.rb[rdst.clone()] { s.rb[rdst] = 1 }
                else { s.rb[rdst] = 0 };
                s.pc = s.pc + 1
            },
            Instr::Sub(rdst, rsrc) => {
                s.rb[rdst] = s.rb[rdst.clone()] - s.rb[rsrc];
                s.pc = s.pc + 1
            },
            Instr::Mul(rdst, rsrc) => {
                s.rb[rdst] = s.rb[rdst.clone()] * s.rb[rsrc];                
                s.pc = s.pc + 1
            },
            Instr::Hlt => break 'mainloop
        };
        println!("{}, next instr = {:?}", show_state(&s), prog[s.pc])        
    }
}

use Instr::*;
use Reg::*;

fn init_state() -> State {
    State {
        pc: 0,
        rb: RegBank{acc: 0, ra: 5, rb: 0, rc: 0}
    }
}


#[test]
fn run_const() {
    let mut s = init_state();
    run(&mut s, &[Const(ACC, 1), Hlt]);
    assert_eq!(s.rb[ACC], 1);
}

#[test]
fn run_mov() {
    let mut s = init_state();
    s.rb[ACC] = 0;
    s.rb[RA] = 7;    
    run(&mut s, &[Mov(ACC, RA), Hlt]);
    assert_eq!(s.rb[ACC], 7);
}

#[test]
fn run_br() {
    let mut s = init_state();
    s.rb[RA] = 7;    
    run(&mut s, &[Br(1), Mov(ACC, RA), Hlt]);
    assert_eq!(s.rb[ACC], 7);
}

fn main() {
    let mut init_state = init_state();
    //Assumes RA stores n > 0
    //Result is stored in ACC
    let fact = vec![
        Mov(RC, RA),        
        Const(ACC, 1),
        Eq(ACC, RA),
        Br(11),
        Mov(ACC, RA),
        Const(RB, 1),
        Sub(ACC, RB),
        Mul(RC, ACC),
        Mov(RA, ACC),
        Const(ACC, 1),
        Br(1),
        Mov(ACC, RC),
        Hlt
    ];

    println!("enter an integer > 0:");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("no input");
    let n = buffer.trim().parse::<i32>().unwrap();
    if n <= 0 {
        panic!(format!("requires n = {} > 0", n))
    };
    init_state.rb[RA] = n;
    run(&mut init_state, &fact[..]);
    println!("fact({}) = {}", n, init_state.rb[ACC])
}
