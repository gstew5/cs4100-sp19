setframe 0
push Lmain
call
halt
Lmain:
push 100
push 7
alloc
push Lf
setframe 2
swap
call
ret
Lf:
var 0
push 23
push 42
set
var 0
push 23
get
ret
