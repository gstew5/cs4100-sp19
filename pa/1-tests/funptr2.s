setframe 0
push Lmain
call
halt
Lmain:
push undef
push 3
push Lf
store 2
var 2
setframe 2
swap
call
store 2
ret
Lf:
var 0
ret
