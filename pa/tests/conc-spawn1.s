setframe 0
push Lmain
call
halt
Lmain:
push undef
push 1
push undef
alloc
store 2
var 2
push 0
push _L4
set
var 2
spawn
push tt
store 2
ret
_L4:
push 97
print
push tt
ret
