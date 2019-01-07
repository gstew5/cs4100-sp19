setframe 0
push Lmain
call
halt
Lmain:
push undef
push undef
push undef
push 4
store 3
push 2
push undef
alloc
store 2
var 2
push 0
push _L6
set
var 2
push 1
var 3
set
var 2
store 4
var 4
push 3
var 4
push 0
get
setframe 3
swap
call
store 2
pop
pop
ret
_L6:
var 0
push 1
get
var 1
binary +
ret
