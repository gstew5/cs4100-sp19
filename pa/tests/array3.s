setframe 0
push Lmain
call
halt
Lmain:
push undef
push 10
push 0
alloc
store 2
push 1
var 2
alloc
push 0
get
store 2
ret
