setframe 0
push Lmain
call
halt
Lmain:
push 0
push Lg
setframe 2
swap
call
ret
Lf:
push 1
push 3
binary /
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
ret
Lg:
push undef
push 1
push 100
push 10
alloc
alloc
store 3
push 15
push Lf
setframe 2
swap
call
pop
var 3
push 0
get
push 50
get
store 3
ret
