setframe 0
push Lmain
call
halt
Lmain:
push undef
push undef
push undef
push 97
store 4
push 2
push undef
alloc
store 2
var 2
push 0
push _L4
set
var 2
push 1
var 4
set
var 2
store 3
var 3
spawn
push tt
store 2
pop
pop
ret
_L4:
var 0
push 1
get
print
push tt
ret
