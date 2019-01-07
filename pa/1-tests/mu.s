setframe 0
push Lmain
call
halt
Lmain:
push 1
ret
Llen:
push undef
push undef
push undef
var 0
store 5
var 5
push 0
get
push _L3
branch
var 5
push 1
get
store 4
push 0
push true
push _L4
branch
_L3:
var 5
push 1
get
store 3
var 3
push 1
get
push Llen
setframe 2
swap
call
push 1
binary +
_L4:
store 3
pop
pop
ret
