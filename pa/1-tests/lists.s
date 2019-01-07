setframe 0
push Lmain
call
halt
Lmain:
push 1
push 2
push 3
push tt
push Lnil
setframe 2
swap
call
push Lcons
setframe 3
swap
call
push Lcons
setframe 3
swap
call
push Lcons
setframe 3
swap
call
push Ladd
setframe 2
swap
call
ret
Lnil:
push undef
push 2
push false
alloc
store 3
var 3
push 1
push tt
set
var 3
store 3
ret
Lcons:
push undef
push undef
push 2
push true
alloc
store 5
var 5
push 1
push 2
push undef
alloc
store 4
var 4
push 0
var 0
set
var 4
push 1
var 1
set
var 4
set
var 5
store 4
pop
ret
Ladd:
push undef
push undef
push undef
var 0
store 5
var 5
push 0
get
push _L9
branch
var 5
push 1
get
store 4
push 0
push true
push _L10
branch
_L9:
var 5
push 1
get
store 3
var 3
push 1
get
push Ladd
setframe 2
swap
call
var 3
push 0
get
binary +
_L10:
store 3
pop
pop
ret
