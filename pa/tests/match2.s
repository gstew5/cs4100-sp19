setframe 0
push Lmain
call
halt
Lmain:
push undef
push undef
push undef
push undef
push undef
push 2
push true
alloc
store 2
var 2
push 1
push 3
set
var 2
store 3
var 3
store 6
var 6
push 0
get
push _L5
branch
var 6
push 1
get
store 5
push 3
push 0
binary +
push true
push _L6
branch
_L5:
var 6
push 1
get
store 4
push 1
var 4
binary +
_L6:
store 2
pop
pop
pop
pop
ret
