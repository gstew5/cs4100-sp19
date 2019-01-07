setframe 0
push Lmain
call
halt
Lmain:
push undef
push 2
push undef
alloc
store 2
var 2
push 0
push false
set
var 2
push 1
push 1
set
var 2
push 0
get
store 2
ret
