setframe 0
push Lmain
call
halt
Lmain:
push 20
push Lfib
setframe 2
swap
call
ret
Lfib:
push 0
var 0
binary ==
push _L3
branch
push 1
var 0
binary ==
push _L1
branch
push 2
var 0
binary -
push Lfib
setframe 2
swap
call
push 1
var 0
binary -
push Lfib
setframe 2
swap
call
binary +
push true
push _L2
branch
_L1:
push 1
_L2:
push true
push _L4
branch
_L3:
push 1
_L4:
ret
