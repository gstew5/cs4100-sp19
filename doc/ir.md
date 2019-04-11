# Grumpy Intermediate Representation

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
      
//Extended expression types NOT required in PA4:
      (print e)             //Evaluate e to an i32 then print as ASCII by casting to u8
      (spawn eclos)         //Spawn a new thread initialized to run eclos (a heap-allocated closure)
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
 prog ::= fn1 fn2 ... fnM % e                      
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
