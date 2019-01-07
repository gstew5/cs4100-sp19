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
var 0
ret
