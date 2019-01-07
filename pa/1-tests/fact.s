setframe 0
push Lmain
call
halt
Lmain:
push 5
push Lfact
setframe 2
swap
call
ret
Lfact:
var 0
push 0
binary ==
push _L1
branch
push 1
var 0
binary -
push Lfact
setframe 2
swap
call
var 0
binary *
push true
push _L2
branch
_L1:
push 1
_L2:
ret
