# The Grumpy Source Language (WIP)

## Syntax

```
Values
v ::= n                     //Signed integers
      true | false          //Boolean values
      tt                    //The unit value
```

```
Expressions
e ::= v                     //Values
      (u e)                 //Unary operation u applied to expression e
      (b e1 e2)             //Binary operation b applied to expressions e1 and e2
      (let x e1 e2)         //Let x equal the result of e1 in e2 (in which x may appear free)
      (seq e1 e2)           //Sequential composition (do e1 then e2)
      (alloc esize einit)   //Allocate an array of size esize, initialized at each index to einit
      (set earr eidx e)     //Update array earr at index eidx to the value of e
      (get earr eidx)       //Get the value at index eidx of array earr
      (cond econd e1 e2)    //If econd evaluates to true then e1, else e2
      (lam x ty e)          //Anonymous function
      (app e1 e2)           //Apply function e1 to argument e2
```

Unary operations `u` and binary operations `b` are the same as in GrumpyIR.

```
Types
ty ::= i32                  //32-bit signed integers
       bool                 //Booleans
       unit                 //The unit type
       (array ty)           //Arrays of ty
       (-> ty1 ty2)         //Functions from ty1 to ty2
```

## Type System

### Values 

```
---------------------------------------------------------------------------- T-i32
G |-v n : i32

---------------------------------------------------------------------------- T-true
G |-v true : bool

---------------------------------------------------------------------------- T-false
G |-v false : bool
```

### Expressions

```
G |-v v : ty
---------------------------------------------------------------------------- T-val
G |- v : ty

G |- e : bool        
---------------------------------------------------------------------------- T-neg
G |- (neg e) : bool

G |- e1 : i32        G |- e2 : i32        b \in {+, *, -, /} 
---------------------------------------------------------------------------- T-binop-i32
G |- (b e1 e2) : i32

G |- e1 : i32        G |- e2 : i32        b \in {<, ==} 
---------------------------------------------------------------------------- T-binop-bool
G |- (b e1 e2) : bool

G,(x:ty1) |- e2 : ty2 
---------------------------------------------------------------------------- T-let
G |- (let x ty1 e2) : ty2

G |- e1 : unit        G |- e2 : ty2
---------------------------------------------------------------------------- T-seq
G |- (seq e1 e2) : ty2

G |- esize : i32      G |- einit : ty 
---------------------------------------------------------------------------- T-alloc
G |- (alloc esize einit) : (array ty)

G |- earr : (array ty)        G |- eidx : i32        G |- e : ty 
---------------------------------------------------------------------------- T-set
G |- (set earr eidx e) : unit

G |- earr : (array ty)        G |- eidx : i32       
---------------------------------------------------------------------------- T-get
G |- (get earr eidx) : ty

G |- econd : bool     G |- e1 : ty        G |- e2 : ty     
---------------------------------------------------------------------------- T-get
G |- (cond econd e1 e2) : ty

G,(x:ty1) |- e2 : ty2     
---------------------------------------------------------------------------- T-lam
G |- (lam x ty1 e2) : (-> ty1 ty2) 

G |- e1 : (-> ty2 ty)        G |- e2 : ty2     
---------------------------------------------------------------------------- T-lam
G |- (app e1 e2) : ty 
```
