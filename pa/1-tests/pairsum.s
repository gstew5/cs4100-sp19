setframe 0
push Lmain
call
halt
Lmain:
push undef
push 2
push true
alloc
store 2
var 2
push 1
push 27
set
var 2
push Lf
setframe 2
swap
call
store 2
ret
Lf:
push undef
push undef
push undef
var 0
store 5
var 5
push 0
get
push _L5
branch
var 5
push 1
get
store 4
push 0
push true
push _L6
branch
_L5:
var 5
push 1
get
store 3
var 3
var 3
binary +
_L6:
store 3
pop
pop
ret
