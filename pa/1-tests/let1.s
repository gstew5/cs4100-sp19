setframe 0
push Lmain
call
halt
Lmain:
push undef
push 1
store 2
push 2
var 2
binary +
store 2
ret
