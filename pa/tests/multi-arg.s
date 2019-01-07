setframe 0
push Lmain
call
halt
Lmain:
push 4
push 3
binary +
push 6
push 5
binary +
push Lf
setframe 3
swap
call
ret
Lf:
var 1
ret
