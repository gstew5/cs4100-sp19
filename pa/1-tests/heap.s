setframe 0
push Lmain
call
halt
Lmain:
push 200
push Lf
setframe 2
swap
call
ret
Lf:
push 1
push 3
binary /
var 0
binary ==
push _L1
branch
push 100
push false
alloc
pop
push 1
var 0
binary -
push Lf
setframe 2
swap
call
push true
push _L2
branch
_L1:
push 3
_L2:
ret
