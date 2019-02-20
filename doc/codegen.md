# Code Generation

## Compiling IR to Assembly

We'll generate assembly from IR by defining a recursive function, `C[ e ]`, that maps expressions to lists of assembly instructions. To make our lives easier as we define `C`, we'll enforce as a **compilation invariant** (about which more below) that the list of instructions produced by `C[ e ]` leaves the stack unchanged *except* for storing `e`'s result on top.

Let's start, for the purposes of this chapter, with the following simple source expression language, a subset of the [Grumpy IR](ir.md):

```
Values 
v ::== i                    //32-bit signed integers
       true | false         //Booleans 
       tt                   //The "unit" value

e ::== v                    //Values  
       x                    //Variables
       (u e)                //Perform unary operation u
       (b e1 e2)            //Perform binary operation b
       (cond econd e1 e2)   //If econd then e1 otherwise e2
       (let x e1 e2)        //Let x equal the result of e1 in e2
       (print e)            //Print the result of e (cast to u8)
```

Our target language will include the following subset of Grumpy assembly: 

```
Instructions
i ::= push v     //Push a value
    | push l     //Push a label
    | pop 
    | unary u
    | binary b
    | var u32    //var i: Push the value at stack position fp+i
    | store u32  //store i: x <- pop(); store x at stack position fp+i
    | branch     //stack = ... Vbool(b) Vloc(target) STACK_TOP: branch to target if b=true
```
### Example: Values

Let's say we're tasked with compiling a value `v`, as in `C[ v ]`. What should the result be? We can simply push `v` onto the stack!

```
C[ v ] = [push v]
```

Note that this satisfies the **compilation invariant** as described above: it leaves the stack unchanged except for storing `v`'s result (which in this case is just `v` itself) on top of the stack.

### Example: Unary Expressions

Let's consider a more complicated expression like `(neg false)`. To compile `(neg false)`, one might first generate an instruction for pushing `false` onto the stack, then a second instruction `unary neg` for negating `false`, replacing it on top of the stack with `true`. This satisfies the compilation invariant.

To generalize a bit, the compilation function for the unary expression case can be defined as:

```
C[ (u e) ] = 
  let instrs = C[ e ]; 
  instrs ++ [unary u]
```

where `u` is an arbitrary unary expression and `e` is its argument expression. We first recursively generate a list of instructions for `e` (`let instrs = C[ e ]`), then produce as the result of `C[ (u e) ]` the new instruction list that first performs `instrs` (leaving `e`'s result on top of the stack by our **compilation invariant** for `C`), then finally performs `Unop(u)`. We use the notation `instrs ++ [Unop(u)]` to indicate the instruction list that appends the singleton list `[Unop(u)]` to the end of `instrs`. 

### Compilation Invariants

Let's talk a bit more about invariants. In general, an invariant is a property `I` that is *preserved* by some function `f(x)`. That is, if `I(x)` (`I` holds of `x`) before the call to `f`, then `I(f(x))` after (`I` holds of `f`'s result). Importantly, `I` need not hold *during* the call to `f`, only before (by assumption) and after `f` terminates. In other words, `f` is free to break the invariant while it's executing but must restore it afterwward.

An invariant of a compilation function, which we've called a *compilation invariant* above, is a property that the compilation function `C` may *assume*, for example of recursive calls to compile subexpressions of `e`, but also must *guarantee* of its result `C[ e ]`.

For example, in the compilation of unary expressions `C[ (u e) ]` above, we assumed that the recursive call to `C[ e ]` returned a list of instructions that satisfied our **compilation invariant** (it leaves the stack unchanged except for pushing `e`'s result). But to respect the invariant, we also must guarantee that the overall list of instructions `instrs.push(Unop(u))` satisfies the invariant (in this case it does because `Unop(u)` pops `e`'s result, then pushes the negated Boolean value).

### Example: Binary Expressions

Consider, as a second example, the compilation of binary expressions `(b e1 e2)`.

```
C[ (b e1 e2) ] =
  let instrs1 = C[ e1 ];
  let instrs2 = C[ e2 ];
  instrs1 ++ instrs2 ++ [binary b]
```

First we run the instructions for `e1`, then the instructions for `e2`, then perform binary operation `b`. By the **compilation invariant**, `instrs1 ++ instrs2` leaves the stack with `... v1 v2 STACK_TOP` where `v1` and `v2` are `e1`'s and `e2`'s result values respectively. The binary operation instruction `Binop(b)` pops `v2` then `v1`, replacing both values with the result of executing `b` (e.g., `v1 + v2` if `b = Add`). Note that this case, like the unary case of `C`, produces a state that satisfies the compilation invariant:  The original stack is unchanged apart from pushing the result of binary operation `b` applied to `v1` and `v2`.

### Example: Conditional Expressions

Here's where things get a bit more complicated. Let's say we're given the expression `(cond econd e1 e2)`. How should this be compiled? We can start by compiling `econd`, `e1`, and `e2` separately, leaving three sequences of instructions `instrs_cond`, `instrs1`, and `instrs2` corresponding respectively to the three expressions. But then how do we connect these instruction sequences to one another?

One way is to generate two fresh labels, one for the "then" branch (`instrs1`) and a second for the "else" branch (`instrs2`). Then we could run `instrs_cond` to determine which branch to jump to. 

Graphically, the situation looks like this:

```
                        /---------------\
                        |  instrs_cond  |
                        \---------------/
                                /\
                              /    \
                  /-----------\    /-----------\ 
                  | _Lthen:   |    | _Lelse:   |
                  |  instrs1  |    |  instrs2  |
                  \-----------/    \-----------/
                              \    /
                                \/
                        /---------------\
                        |    _Lend:     |
                        \---------------/
```

The `instrs_cond` block can jump to either `_Lthen` or `_Lelse` depending on whether `instrs_cond` evaluates to `true` or `false`. Both the "then" and the "else" blocks jump, when they're done executing, to a common block labeled `_Lend` that merges their control flow.
