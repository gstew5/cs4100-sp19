# Tail Recursion

Some recursive functions, those that are *tail recursive*, can be optimized to use constant stack space. This optimization is especially important in functional languages like Grumpy that make heavy use of functions and recursion to replace traditional loops. We start this section with a few definitions, then explain how tail recursive functions can be optimized.

## Tail Calls

A function *f* makes a *tail call* to a function *g* if the call to *g* is the last thing *f* does before returning (the call is in *tail position*). A function is tail recursive if the only recursive calls it makes are tail calls.   

Consider as an example the following GrumpyIR program that computes the sum of the numbers from 1 to *n*, the *n*th triangular number:

```
(fun trian (x i32) -> i32
  (cond (== x 0) 0
        (+ x (trian (- x 1)))))
```

This function makes a recursive call `(trian (- x 1))` but not in tail position: the result of the call is added to `x` to produce the function's overall result.

The fact that work is done after the recursive call is reflected in the corresponding Grumpy assembly code: 

```
Ltrian:
  push 0
  var 0
  binary ==
  push _L1
  branch
  push 1
  var 0
  binary -
  push Ltrian
  setframe 2
  swap
  call         //Recursive call to Ltrian
  var 0        //Push x
  binary +     //Add x to result of recursive call
  push true
  push _L2
  branch       //Branch to ret
_L1:
  push 0
_L2:
  ret
```

The function can be rewritten, however, to make it tail recursive: 

```
(fun trian2 (acc i32) (x i32) -> i32
  (cond (== x 0)
        acc
        (trian2 (+ x acc) (- x 1))))
```

The general pattern is to use an accumulator `acc` to store the incremental results of the computation, then to pass `acc` as an argument to recursive calls. Function `trian2` behaves the same as `trian` (assuming `acc=0` initially) but is now tail recursive, as reflected in the assembly code: 

```
_Ltrian2:
  ...
  push Ltrian2
  setframe 3
  swap
  call         //Recursive call to Ltrian2
  push true
  push _L2
  branch       //Branch to ret
  ...
  _L2:
  ret
```

Note that `trian2` returns directly after making its recursive call (for the purposes of tail call optimization, we can think of the jump to `_L2` followed by `ret` as a single return operation).

## Optimizing Tail Recursion 

Tail-recursive functions can be optimized to use constant stack space. Why is this? 

For a typical non-tail-recursive function, on every recursive call we push a new stack frame to store the local state of the recursive call. When the recursive call returns and its stack frame is popped, the caller uses the return value and its own local state (stored in its own stack frame) to continue executing. In the first `trian` function above for example, we need to remember the value of `x` in order to execute the addition that occurs after the recursive call `(trian (- x 1))`.

For tail-recursive functions, we no longer care about the caller's state. This is because the recursive call is the last thing the function does before returning; it could not possibly perform a computation with an argument like `x`. 

The tail recursion optimization exploits this fact (that the caller's state is dead at the point of the tail-recursive call) to have the recursive callee *reuse* the caller's state, thus turning the recursive call into a loop. Applying this transformation to `trian2` above we get:

### Before Tail Recursion Optimization:

```
(fun trian2 (acc i32) (x i32) -> i32
  (cond (== x 0)
        acc
        (trian2 (+ x acc) (- x 1))))
```

### After:

```
(fun trian2 (acc i32) (x i32) -> i32
  (seq 
    (while (neg (== x 0)
        (seq 
          (set acc (+ acc x))  //acc += x
          (set x (- x 1))))    //x -= 1
    acc))
```

In the **After** version, we used some non-GrumpyIR syntax such as `(set x (- x 1))` to describe mutable updates to `trian2`'s arguments `acc` and `x`. In Grumpy assembly, we could implement these updates using the `store` instruction:

```
Ltrian2:       //var 0 stores acc, var 1 stores x    
  push 0       
  var 1
  binary ==    //If x == 0, jump to _L1
  push _L1
  branch
  var 0        //Else:
  var 1
  binary +
  store 0      //acc += x 
  push 1
  var 1
  binary - 
  store 1      //x -= 1
  push true
  push Ltrian2 
  branch       //Jump to Ltrian2
  push true
  push _L2
  branch
_L1:
  var 0
_L2:
  ret
```

You might worry about mutably updating `trian2`'s arguments `x` and `acc`. This is OK; all the arguments (modified or not) will be popped by `ret` when the function returns.
