# Grumpy Intermediate Representation (WIP)

The Grumpy compiler desugars Grumpy source code to GrumpyIR, a smaller language that's simpler to optimize and compile. In this chapter, we present GrumpyIR's syntax and semantics, along with a few example programs that illustrate features of the language.

## Syntax

GrumpyIR's syntax is as follows.

We define values as either 32-bit integers or the distinguished Boolean values `true` and `false`.

```
Values 
v ::= n                   //32-bit signed integers
      true | false        //Boolean values
      tt                  //The unit value
```

Larger GrumpyIR programs are composed of expressions, which are wrapped in function definitions, which themselves compose whole programs. We describe the GrumpyIR expression language first. Here's its syntax:

```
Expressions 
e ::= v                     //Values
      x                     //Variables
      (u e)                 //Unary operation u applied to expression e
      (b e1 e2)             //Binary operation b applied to expressions e1 and e2
      (let x e1 e2)         //Let x equal the result of e1 in e2 (in which x may appear free)
      (seq e1 e2)           //Sequential composition (do e1 then e2)
      (alloc esize einit)   //Allocate an array of size esize, initialized at each index to einit
      (set earr eidx e)     //Update array earr at index eidx to the value of e
      (get earr eidx)       //Get the value at index eidx of array earr
      (cond econd e1 e2)    //If econd evaluates to true then e1, else e2
      (funptr f)            //A pointer to function f
      (call e e1 e2 ... eN) //Call function pointer e  
      (f e1 e2 ... eN)      //Call function f
```

The syntax `(f e1 e2 ... eN)`, while supported by a compliant GrumpyIR parser, is purely for convenience. All such calls can be desugared as `(call (funptr f) e1 e2 ... eN)`.

The supported unary and binary operators are the same as in GrumpyVM assembly.

```
Unary Operators
u ::= neg

Binary Operators
b ::= + | * | - | / | < | ==
```

Function definitions are composed of: 
1. The function name (e.g., `f`)
2. The parameters `param1 param2 ... paramN`
3. The return type `ty`
4. The function's body `e`, an expression.

```
Functions 
param ::= (x ty)                                   //Function parameters, annotated with types
   fn ::= (fun f param1 param2 ... paramN -> ty e) //Function definitions
```

Parameters are pairs of an identifier `x` and type `ty` where types are defined as:

```
Types 
ty ::= i32                //32-bit integers
       bool               //Booleans
       unit               //The unit type
       (array ty)         //Arrays of values of type ty
```

Finally, programs compose sequences of functions (0 or more) with a "main" expression `e`, the result of which is the result of the whole program. The function definitions must be separated from the body by a `%`.

```
Programs
 prog ::= fn1 fn2 ... fnM ; e                      
```

### Example 1

Here's an example GrumpyIR program that allocates an array of size `100` (with each index initialized to `7`), updates the array at index `23` to equal `42`, then returns the value at index `23`, which should equal `42`.

```
(fun f (x (array i32)) -> i32 
  (seq (set x 23 42) 
       (get x 23)))
%
(f (alloc 100 7))
```

### Example 2

Here's a slightly larger example program:

```
(fun fib (x i32) -> i32
     (cond (== x 0) 1
           (cond (== x 1) 1
	   	 (+ (fib (- x 1)) (fib (- x 2))))))		 
%
(fib 20)
```

that implements the Fibonnaci sequence recursively following the equations:

```
fib(0) = 1
fib(1) = 1
fib(n | n > 1) = fib(n-1) + fib(n-2)
```

## Compiling IR to Assembly

We'll generate assembly from IR by defining a recursive function, `C[ e ]`, that maps expressions to lists of assembly instructions. To make our lives easier as we define `C`, we'll enforce as a **compilation invariant** (about which more below) that the list of instructions produced by `C[ e ]` leaves the stack unchanged *except* for storing `e`'s result on top.

### Example: Unary Expressions

For example, to compile the expression `(neg false)`, one might first generate an instruction for pushing `false` onto the stack, then a second instruction `Unop(Neg)` for negating `false`, replacing it on top of the stack with `true`. In general, the compilation function for the unary expression case can be defined as:

```
C[ (u e) ] = 
  let instrs = C[ e ]; 
  instrs ++ [Unop(u)]
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
  instrs1 ++ instrs2 ++ [Binop(b)]
```

First we run the instructions for `e1`, then the instructions for `e2`, then perform binary operation `b`. By the **compilation invariant**, `instrs1 ++ instrs2` leaves the stack with `... v1 v2 STACK_TOP` where `v1` and `v2` are `e1`'s and `e2`'s result values respectively. The binary operation instruction `Binop(b)` pops `v2` then `v1`, replacing both values with the result of executing `b` (e.g., `v1 + v2` if `b = Add`). Note that this case, like the unary case of `C`, produces a state that satisfies the compilation invariant:  The original stack is unchanged apart from pushing the result of binary operation `b` applied to `v1` and `v2`.
