setframe 0
push Lmain
call
halt
Lmain:
push 200
push Lf
setframe 2
swap
call
ret
Lf:
push undef
push 0
var 0
binary ==
push _L1
branch
push 100
push false
alloc
pop
push 1
var 0
alloc
store 3
push 1
var 3
push 0
get
binary -
push Lf
setframe 2
swap
call
push true
push _L2
branch
_L1:
push 3
_L2:
store 3
ret
