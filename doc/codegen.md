# Code Generation

## Compiling IR to Assembly

We'll generate assembly from IR by defining a recursive function, `C[ e ]`, that maps expressions to programs in assembly language, which are just sequences of assembly instructions or labels as we'll define in the grammar below. To make our lives easier as we define `C`, we'll enforce as a **compilation invariant** (about which more below) that the list of instructions produced by `C[ e ]` leaves the stack unchanged *except* for storing `e`'s result on top.

Let's start, for the purposes of this chapter, with the following simple source expression language, a subset of the [Grumpy IR](ir.md):

```
Values 
v ::== i                    //32-bit signed integers
       true | false         //Booleans 
       tt                   //The "unit" value

Expressions
e ::== v                    //Values  
       (u e)                //Perform unary operation u
       (b e1 e2)            //Perform binary operation b
       (cond econd e1 e2)   //If econd then e1 otherwise e2
       (seq e1 e2)          //Do e1 then e2
       (print e)            //Print the result of e (cast to u8)
```

Our target language will include the following subset of Grumpy assembly: 

```
Instructions
i ::= push v     //Push value v
    | push L     //Push label L
    | pop 
    | unary u
    | binary b
    | branch     //stack = ... Vbool(b) Vloc(target) STACK_TOP: branch to target if b=true

Instructions or Labels
iL ::= i         //An "il" is either an instruction 
     | L:        //or a label "L" followed by a colon ":", as in "Lmain:".         
```

Programs in the target language are sequences of instructions or labels:

```
Program
p ::= [iL1, iL2, ..., iLN]
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

Let's talk a bit more about invariants. In general, an invariant is a property `I` that is *preserved* by some function `f(x)`. That is, if `I(x)` (`I` holds of `x`) before the call to `f`, then `I(f(x))` after (`I` holds of `f`'s result). Importantly, `I` need not hold *during* the call to `f`, only before (by assumption) and after `f` terminates. In other words, `f` is free to break the invariant while it's executing but must restore it afterward.

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

Here's one way to code this pattern up:

```
C[ (cond econd e1 e2) ] = 
  //Compile expressions
  let instrs_cond = C[ econd ];
  let instrs1 = C[ e1 ];
  let instrs2 = C[ e2 ];
    
  //Allocate fresh (unused) labels 
  let _Lthen = fresh_label(); //Prefix "_L" means compiler internal 
  let _Lelse = fresh_label(); 
  let _Lend  = fresh_label();
  
  //Generate code
  instrs_cond ++ 
  [push _Lthen, branch, push true, push _Lelse, branch] ++
  [_Lthen:] ++ instrs1 ++ [push true, push _Lend, branch] ++
  [_Lelse:] ++ instrs2 ++ [push true, push _Lend, branch] ++
  [_Lend:]
```  

We can optimize this pattern slightly, by recognizing that `branch` falls through to the next instruction when a branch isn't taken:

```
  //Generate optimized code
  instrs_cond ++ 
  [push _Lthen, branch] ++
  instrs2 ++ [push true, push _Lend, branch] ++  
  [_Lthen:] ++ instrs1 ++ [push true, push _Lend, branch] ++
  [_Lend:]
```  

## Example: Sequences

Once we know how to compile expressions like `cond`, sequence expressions like `(seq e1 e2)` which first run `e1`, then run `e2` are reasonably straightforward.

```
C[ (seq e1 e2) ] =
  let instrs1 = C[ e1 ];
  let instrs2 = C[ e2 ];
  instrs1 ++
  [pop]
  instrs2
```

The only wrinkle is that -- by the compilation invariant -- `instrs1` will leave `e1`'s result on top of the stack. We need to `pop` this result before we execute `instrs2`. Otherwise, the stack will contain an extra value. 

Why do we pop `e1`'s result instead of using it? When executing the sequential composition of two expressions, the first expression `e1` is typically executed solely for its side-effects. For example, in the expression `(seq (print 65) 7)`, the first expression `(print 65)` prints the character `A` to `stdout` (`65` is the ASCII encoding of `A`). The overall expression returns `7`, the result of the second expression. 

## Let Expressions and Variables

Let's consider an extension of the source and target languages above to support let-bound variables. 

We might extend the source language as:

```
Expressions
e ::== ...           //Everything from before plus:
     | x             //Variables
     | (let x e1 e2) //Let expressions: Let x equal the result of e1 in e2
```

To support variables in the target language, we'll extend our instruction set with: 

```
Instructions 
i ::== ...       //Everything from before plus:
     | var u32    //var i: Push the value at stack position fp+i
     | store u32  //store i: x <- pop(); store x at stack position fp+i
```

How should we compile variables? On a register machine, the compiler has to figure out how to allocate potentially many variables to a fixed set of registers. Since we're targeting a stack machine, we'll instead store all our variables (arguments and local variables) on the stack, starting at a position marked by a special register called the *frame pointer*, or `fp` (note that on a register machine, some locals might also be stored on the stack, if their address is taken in a language like C or if they spilled, i.e., couldn't fit in registers). 

In the GrumpyVM, as a function begins executing its body, the stack looks like this: 

| Stack Position | fp+0 | ... | fp+(N-1) |        |        | (fp+(N-1)+2)+0 | ... | (fp+(N-1)+2)+(M-1) | 
|----------------|------|-----|----------|--------|--------|----------------|-----|--------------------|
| **Value**      | arg1 | ... | argN     | ret_fp | ret_pc | loc_var1       | ... | loc_varM           |

The `N` arguments are at addresses `fp+0` to `fp+(N-1)` while the function's `M` local variables are stored at addresses `(fp+(N-1)+2)+0` to `(fp+(N-1)+2)+(M-1)`. The weird `+2` term is due to storage on the stack, when a GrumpyVM `call` instruction is executed, of the caller's `ret_fp` and `ret_pc`. Note also that in the GrumpyVM, the caller is responsible for pushing the arguments whereas the callee is responsible for making space on the stack for local variables. We'll come back to this point when we talk about functions below.

Given the argument and local variable layout in the figure above, how should we implement `C[ x ]` where `x` is an arbitrary variable? One method is to maintain a mapping, from variables to their indices in the stack frame, telling us the location of `x`'s storage. That way, we can figure out where to find `x` when we need to read or initialize it. In pseudocode, this looks like: 

```
C[ x ] = 
  let i = position of x in current stack frame;
  return value at stack position [fp+i]
```

The last line of this pseudocode we can implement using the `var` instruction: `[var i]`. The `let i = position of x...` is a bit harder. We need to update our `C` function, as motivated above, to take an extra argument, the context mapping variables to their indices. Calling this context `rho`, we now get:

```
C_rho[ x ] = 
  let i = rho(x);
  [var i]
```

Note that compilation may now fail if, for example, `x` was not defined (it's neither a function parameter nor a let-bound local variable) and therefore does not appear in `rho`.

How do variables get placed in `rho`? One way is using `let` expressions. Consider, for example, the expression:

```
(let x <exp> 
     (let y (+ x <exp>)
          (let z (+ y <exp>))))
```

If this expression occurred inside the body of a function, then we as compiler writers could identify that the variables `x`, `y`, and `z` are all variables that require space in the function's stack frame. We could therefore make sure to add the appropriate entries in `rho` before we begin compiling the expression. Another method is to construct the environment `rho` as we go, making sure to keep track of which variables we may need to allocate space for in the stack frame. Either works fine but the first requires multiple passes through the expression.

Assuming for purposes of discussion that we've done this mapping in `rho` already by making a first pass through the expression, let's consider compiling the general form of a `let` expression. The recipe is: 

```
C_rho[ (let x e1 e2) ] =
  let i = rho(x);
  let instrs1 = C_rho[ e1 ];
  let instrs2 = C_rho[ e2 ];
  instrs1 ++ [store i] ++ instrs2
```

That is, we first look up `x`'s storage location `i`, then we compile `e1` and `e2` but making sure to store `e1`'s result at position `fp+i` in the stack frame before we begin executing `e2`. That way, if `e2` looks up `x`, it will find the appropriate value (`e1`'s result).

As an example, consider what happens when we compile `(let x 1 x)`.

```
C_rho[ (let x 1 x) ] = 
  let i = rho(x);
  let instrs1 = C_rho[ 1 ] (= [push 1]);
  let instrs2 = C_rho[ x ] (= [var i]);
  instrs1 ++ [store i] ++ instrs2 (= [push 1, store i, var i])
```

The resulting code first pushes `1`, then pops `1` storing it at stack position `fp+i`, then looks up the value at exactly the same position, pushing it onto the stack. We end up with the stack containing `1`. 

## Functions

As our final extension, let's consider what happens when we add functions to our source and target languages. We'll extend our source-language syntax as follows:

```
Expressions
e ::== ...                    //Everything from before plus:
     | (funptr f)             //The location of function f
     | (call ef e1 e2 ... eN) //Call ef (which should evaluate to a function pointer) on e1, e2, ..., eN
```

Likewise, we'll extend our target language's syntax to support calls and returns as follows:

```
Instructions 
i ::== ...        //Everything from before plus:
     | call       //pre_stack  = ... varg1 varg2 ... vargN Vloc(caller_fp) Vloc(target) STACK_TOP
                  //post_stack = ... varg1 varg2 ... vargN Vloc(caller_fp) Vloc(cur_pc) STACK_TOP
     | ret        //pre_stack  = ... varg1 varg2 ... vargN Vloc(ret_fp) Vloc(ret_pc) vret STACK_TOP        
                  //post_stack = ... vret STACK_TOP
     | setframe i //Push current fp, set fp to stack_len - i - 1
     | swap       //Swap the topmost two values on the stack
```

We'll use the two additional instructions `setframe` and `swap` instructions in our implementation of `call` below.

### Function Pointers

But first, how do we compile a `funptr` expression? It becomes a push of the location associated with the function:

```
C_rho[ (funptr f) ] = [push Lf]
```

If functions are defined at most once, then from a given function name `f` we can also build an appropriate corresponding fresh label `Lf`.

### Function Calls

Compiling function calls is a bit trickier:

```
C_rho[ (call ef e1 e2 ... eN) ] = 
  let instrs1 = C_rho[ e1 ];
  let instrs2 = C_rho[ e2 ];
  ...
  let instrsN = C_rho[ eN ];
  let instrs_ef = C_rho[ ef ];
  instrs1 ++ instrs2 ++ ... ++ instrsN ++ instrs_ef ++ [setframe (N+1), swap, call]
```  

First we compile instructions for the arguments `e1` through `eN`. Then we compile code for the function pointer itself, `ef`. Finally, we string things together by first evaluating (in our generated code) the arguments, then the function pointer, then `setframe` followed by a `swap` and `call`.

The `setframe (N+1)` instruction updates the frame pointer to point `N+1` values into the stack, at the value of `e1` that gets pushed by executing `instrs1`. The slightly weird `+1` deals with the fact that `instrs_ef` just pushed an additional value, the function pointer about to be called. Instruction `setframe` pushes`ret_fp`, the previous frame pointer, on the stack but `call` expects the function pointer on top so we execute a `swap` before `call` to switch the order of the two arguments. 

```
Question for self-study: What's wrong with setting the frame pointer *before* executing instrs_ef?
```

### Function Definitions (WIP)

Function in Grumpy have form: 

```
   fn ::= (fun f param1 param2 ... paramN -> ty e) //Function definitions
```

where `param`s are pairs of strings `x` along with their expected types `ty`.

How to compile such a function definition?

The first step is to determine the set of let-bound local variables in the body of the function `e`. Then we can construct an overall compilation environment `rho` for the function body that includes both the function's parameters and its let-bound variables.

We define the let-bound variables of an expression `e` by:

```
bvs[ x ] = []
bvs[ (let x e1 e2) ] = {x} \cup bvs[ e1 ] \cup bvs[ e2 ]
```

where by `\cup` we mean the union of two sets.

The bound variables of the other expression types are defined similarly, e.g.:

```
bvs[ (cond e e1 e2) ] = bvs[ e ] \cup bvs[ e1 ] \cup bvs[ e2 ]
bvs[ (seq e1 e2) ] = bvs[ e1 ] \cup bvs[ e2 ]
...
```

We make the simplifying assumption that there is no let-bound variable shadowing (such shadowing can be detected during type checking). 

Once we know the let-bound variables of the body of a function `e`, we can construct the compilation environment for the function definition as a map from variable names to their storage locations, as indices `i` from the frame pointer: 

| **Variable**   | arg1 | ... | argN     | let_var1       | ... | let_varM           |
|----------------|------|-----|----------|----------------|-----|--------------------|
| **rho**        | 0    | ... | N-1      | (N-1)+2        | ... | (N-1)+2+(M-1)      | 

Variable `x` is stored at `fp + rho(x)`. For example, variable `let_var1` is stored at stack position `fp+(N-1)+2` where `N` is the number of function arguments. The `+2` gap between the arguments `argN` and `let_var1` is to account for the `ret_fp` and `ret_pc` values that are stored on the stack by the caller. 

When a function begins executing, it needs to allocate space on the stack for its let-bound variables (by the GrumpyVM calling convention, the caller pushes the arguments and its return frame pointer and program counter). In pseudocode, this process is:

```
C[ (fun f param1 param2 ... paramN -> ty e) ] = 
  let mut instrs = [];
  //Calculate let-bound variables of function body e.
  let let_vars = bvs[ e ];
  //INTRO: Allocate callee stack frame (where let-bound variables will be stored).
  for x in let_vars:
    instrs.push( push Vundef );
  //Build compilation environment rho
  let rho = compilation environment for function, as described above;
  //Compile e, referencing rho
  instrs.append( C_rho[ e ] );
  //OUTRO: Pop let-bound variables; store return value on top of stack  
  ...
  instrs.append( ... )
  //Return
  instrs.push( ret )
```

The final missing piece is the `OUTRO`, the code that deallocates the function's let-bound variables and then has the function return. The main idea is store the return value at the appropriate slot on the stack (as expected by the `ret` instruction), then simply to pop the let-bound variables.
