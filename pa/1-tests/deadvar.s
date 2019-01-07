setframe 0
push Lmain
call
halt
Lmain:
push undef
push undef
push false
push _L1
branch
push 7
push true
push _L2
branch
_L1:
push 3
store 3
push 4
store 2
var 2
_L2:
store 2
pop
ret
