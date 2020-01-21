
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

fn eval(e: Exp) {
    match e {
        EInt(i) => i,
        ERand => ...,
        EBinop(b) => ...,
    }
}
