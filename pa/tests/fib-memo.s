setframe 0
push Lmain
call
halt
Lmain:
push undef
push 20
store 2
var 2
push 1
var 2
binary +
push 0
alloc
push 1
var 2
binary +
push false
alloc
push Lfib
setframe 4
swap
call
store 2
ret
Lfib:
push undef
var 0
push 0
binary ==
push _L7
branch
var 0
push 1
binary ==
push _L5
branch
var 2
push 2
var 0
binary -
get
push _L3
branch
push 2
var 0
binary -
var 1
var 2
push Lfib
setframe 4
swap
call
push true
push _L4
branch
_L3:
var 1
push 2
var 0
binary -
get
_L4:
var 2
push 1
var 0
binary -
get
push _L1
branch
push 1
var 0
binary -
var 1
var 2
push Lfib
setframe 4
swap
call
push true
push _L2
branch
_L1:
var 1
push 1
var 0
binary -
get
_L2:
binary +
push true
push _L6
branch
_L5:
push 1
_L6:
push true
push _L8
branch
_L7:
push 1
_L8:
store 5
var 1
var 0
var 5
set
var 2
var 0
push true
set
var 5
store 5
ret
