use types::*;
use types::Binop::*;
use types::Exp::*;
use types::Instr::*;

pub fn compile(e: &Exp) -> Vec<Instr> {
    //INVARIANT: e's result left on top of stack
    match e {
        EI32(i) => vec![II32(*i)],
        EBinop(b) => {
            let mut is_lhs = compile(&b.lhs);
            let mut is_rhs = compile(&b.rhs);
            let mut is_op =
                match b.op.clone() {
                    BPlus => vec![IPlus],
                    BTimes => vec![ITimes]
                };
            let mut is = vec![];
            is.append(&mut is_lhs);
            is.append(&mut is_rhs);
            is.append(&mut is_op);
            is
        }
    }
}
