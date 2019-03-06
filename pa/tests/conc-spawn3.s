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
push undef
push 1
push undef
alloc
store 2
var 2
push 0
push _L9
set
var 2
store 7
push 97
store 6
push 98
store 5
var 7
store 3
var 3
var 6
var 3
push 0
get
setframe 3
swap
call
spawn
var 7
store 4
var 4
var 5
var 4
push 0
get
setframe 3
swap
call
spawn
push tt
store 2
pop
pop
pop
pop
pop
ret
_L12:
var 0
push 1
get
print
push tt
ret
_L9:
push undef
push 2
push undef
alloc
store 4
var 4
push 0
push _L12
set
var 4
push 1
var 1
set
var 4
store 4
ret
