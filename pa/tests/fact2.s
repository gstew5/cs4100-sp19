setframe 0
push Lmain
call
halt
Lmain:
push 12
push Lfact
setframe 2
swap
call
ret
Lfact:
push undef
push undef
push 100
push 0
alloc
store 4
var 4
push 0
var 0
set
var 4
push 0
get
store 3
var 3
push 0
binary ==
push _L1
branch
push 1
var 3
binary -
push Lfact
setframe 2
swap
call
var 3
binary *
push true
push _L2
branch
_L1:
push 1
_L2:
store 3
pop
ret
