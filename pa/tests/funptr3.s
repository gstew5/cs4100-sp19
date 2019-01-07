setframe 0
push Lmain
call
halt
Lmain:
push undef
push false
push _L1
branch
push Lg
push true
push _L2
branch
_L1:
push Lf
_L2:
store 2
push 3
var 2
setframe 2
swap
call
store 2
ret
Lf:
var 0
ret
Lg:
push 1
var 0
binary +
ret
