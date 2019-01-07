setframe 0
push Lmain
call
halt
Lmain:
push 3
push Lf
setframe 2
swap
call
ret
Lf:
push undef
var 0
store 3
var 3
var 3
binary +
store 3
ret
