setframe 0
push Lmain
call
halt
Lmain:
push undef
push undef
push undef
push undef
push undef
push 1
push undef
alloc
store 2
var 2
push 0
push _L14
set
var 2
store 6
var 6
store 4
var 4
push 1
push undef
alloc
store 3
var 3
push 0
push _L24
set
var 3
var 4
push 0
get
setframe 3
swap
call
store 5
var 5
push 10
var 5
push 0
get
setframe 3
swap
call
store 2
pop
pop
pop
pop
ret
_L17:
push undef
push undef
var 0
push 1
get
store 5
var 5
var 0
push 1
get
store 4
var 4
var 1
var 4
push 0
get
setframe 3
swap
call
var 5
push 0
get
setframe 3
swap
call
store 4
pop
ret
_L14:
push undef
push 3
push undef
alloc
store 4
var 4
push 0
push _L17
set
var 4
push 2
var 1
set
var 4
push 1
var 1
set
var 4
store 4
ret
_L24:
var 1
push 1
binary +
ret
