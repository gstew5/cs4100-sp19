use lexer::{LexerState};
use lexer::Tok::*;
use types::*;
use types::Exp::*;
use types::Binop::*;

/* GRAMMAR:

   <start> ::= <exp> $
     <exp> ::== <i32>
              | <exp> + <exp>
              | <exp> * <exp>

   RE-FACTORED GRAMMAR:

 1.        <exp> ::== <term> <exp-rest> $
 2.   <exp-rest> ::== + <term> <exp-rest>
 3.                 | <empty-string>
 
 4.       <term> ::== <factor> <term-rest>
 5.  <term-rest> ::== * <factor> <term-rest>
 6.                 | <empty-string>

 7.     <factor> ::== <i32>     

    FIRST/FOLLOW SETS FOR REFACTORED GRAMMAR:

    SYMBOL     FIRST       FOLLOW
    ----------------------------
    <exp>      | <i32>     | 
    <exp-rest> | +         | $
    <term>     | <i32>     | +, $
    <term-rest>| *         | +, $
    <factor>   | <i32>     | *

    PREDICTIVE PARSING TABLE: 
 
                 <i32>      |      +     |      *     |     $   
    --------------------------------------------------------------
         <exp> | 1          |            |            |   
    <exp-rest> |            | 2          |            | 3  
        <term> | 4          |            |            |   
   <term-rest> |            | 6          | 5          | 6  
      <factor> | 7          |            |            |   
     
*/

macro_rules! parse_err {
    ( $l:expr, $err:expr ) => {
        Err(format!("{} at {}:{} in '{}'",
                    $err, $l.info.line_no, $l.info.col_no, $l.rest))
    };
}

fn parse_exp(l: &mut LexerState) -> Result<Exp,String> {
    match l.peek().expect("exp: expected a token") {
        I32(_) => {
            let t = parse_term(l)?;
            let erest = parse_erest(l)?;
            l.eat(DOLLAR);
            Ok(EBinop(Box::new(Binexp{op: BPlus, lhs: t, rhs: erest})))
        },
        tok => parse_err!(l, format!("exp: unexpected token {:?}", tok))
    }
}

fn parse_erest(l: &mut LexerState) -> Result<Exp,String> {
    match l.peek().expect("erest: expected a token") {
        PLUS => {
            l.eat(PLUS);
            let t = parse_term(l)?;
            let erest = parse_erest(l)?;
            Ok(EBinop(Box::new(Binexp{op: BPlus, lhs: t, rhs: erest})))
        },
        DOLLAR => {
            Ok(EI32(0)) //The unit for +
        },
        tok => parse_err!(l, format!("erest: unexpected token {:?}", tok))
    }
}

fn parse_term(l: &mut LexerState) -> Result<Exp,String> {
    match l.peek().expect("term: expected a token") {
        I32(_) => {
            let f = parse_factor(l)?;
            let trest = parse_trest(l)?;
            Ok(EBinop(Box::new(Binexp{op: BTimes, lhs: f, rhs: trest})))
        },
        tok => parse_err!(l, format!("term: unexpected token {:?}", tok))
    }
}

fn parse_trest(l: &mut LexerState) -> Result<Exp,String> {
    match l.peek().expect("trest: expected a token") {
        TIMES => {
            l.eat(TIMES);
            let f = parse_factor(l)?;
            let trest = parse_trest(l)?;
            Ok(EBinop(Box::new(Binexp{op: BTimes, lhs: f, rhs: trest})))
        },
        PLUS => Ok(EI32(1)), //The unit for *
        DOLLAR => Ok(EI32(1)), //The unit for *
        tok => parse_err!(l, format!("erest: unexpected token {:?}", tok))
    }
}

fn parse_factor(l: &mut LexerState) -> Result<Exp,String> {
    match l.peek().expect("factor: expected a token") {
        I32(i) => {
            l.eat(I32(i));
            Ok(EI32(i))
        },
        tok => parse_err!(l, format!("term: unexpected token {:?}", tok))
    }
}

pub fn parse(s: &str) -> Result<Exp,String> {
    let mut l = LexerState::new(s);
    parse_exp(&mut l)
}
