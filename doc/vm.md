# Grumpy Virtual Machine

A virtual machine (VM) is a language and associated interpreter that together abstract the details of a computer's hardware instruction set. The languages that VMs interpret are typically low level and assembly-like. VMs are useful compiler targets because they are independent of any particular computer's hardware. They are also often slow with respect to running code directly on the hardware, precisely because they are hardware independent.

To get from VM to hardware, one compiles the virtual machine interpreter using a compiler that generates assembly code appropriate for the platform on which the VM is run (e.g., x86). To run code on the VM, the compiled virtual machine binary is executed on a compact in-memory representation of a program in the language supported by the VM.

In this chapter, we present GrumpyVM, a virtual machine for a low-level stack-based language. We explain the language and its semantics first. Then we describe the architecture of our GrumpyVM interpreter, implemented in Rust.

## Machine Model

The GrumpyVM implements a machine model with the following major components:

* Flag `halt` indicating whether the machine has halted
* Register `pc` containing the current program counter, a `u32`
* Register `fp` containing the current frame pointer, a `u32`
* A stack of values `Val`, with maximum size `STACK_SIZE` 
* A heap of values `Val`, with maximum size `HEAP_SIZE`
* The program to be executed, `program`, a list of instructions

We implement the machine model as the Rust datatype:

```
#[derive(Debug,Clone)]
pub struct State {
    pub halt: bool, //Has the machine halted?
    pub pc: u32, //The current program counter, a 32-bit unsigned integer
    pub fp: u32, //The current frame pointer
    pub stack: Vec<Val>, //The stack, with maximum size STACK_SIZE
    pub heap: Vec<Val>, //The heap
    pub program: Vec<Instr> //The program being executed, a list of instructions
}
```

Values are either undefined, unit, 32-bit integers, booleans, locations, object sizes, or addresses, as implemented by: 

```
#[derive(Debug,Clone,PartialEq)]
pub enum Val {
    //Value types that may appear in GrumpyVM programs:
    Vunit,          //The unit value
    Vi32(i32),      //32-bit signed integers
    Vbool(bool),    //Booleans
    Vloc(u32),      //Stack or instruction locations
    Vundef,         //The undefined value
    
    //Value types that are used internally by the language implementation, and may not appear in GrumpyVM programs:
    Vsize(i32),     //Metadata for heap objects that span multiple values
    Vaddr(Address), //Pointers to heap locations
}
```

Addresses are implemented as `usize`:

```
type Address = usize;
```

the type of pointer-sized unsigned integers appropriate for the machine on which the VM will be run.

## Language

The GrumpyVM language consists of the following instructions. We describe the instructions at a high level first, then go into detail.

```
#[derive(Debug,Clone)]
pub enum Instr {
    Push(Val),     //Push(v): Push value v onto the stack
    Pop,           //Pop a value from the stack, discarding it
    Peek(u32),     //Peek(i): Push onto the stack the ith value from the top
    Unary(Unop),   //Unary(u): Apply u to the top value on the stack
    Binary(Binop), //Binary(b): Apply b to the top two values on the stack, replacing them with the result
    Swap,          //Swap the top two values
    Alloc,         //Allocate an array on the heap
    Set,           //Write to a heap-allocated array
    Get,           //Read from a heap-allocated array
    Var(u32),      //Var(i): Get the value at stack position fp+i
    Store(u32),    //Store(i): Store a value at stack position fp+i
    SetFrame(u32), //SetFrame(i): Set fp = s.stack.len() - i
    Call,          //Function call
    Ret,           //Function return
    Branch,        //Conditional jump
    Halt           //Halt the machine
}
```

The unary operators are: 

```
#[derive(Debug,Clone)]
pub enum Unop {
    Neg, //Boolean negation
}
```

while the binary operators are: 

```
#[derive(Debug,Clone)]
pub enum Binop {
    Add, //i32 addition
    Mul, //i32 multiplication
    Sub, //i32 subtraction
    Div, //i32 division (raises an error on divide by zero)
    Lt,  //Returns true if one i32 is less than another, otherwise false
    Eq,  //Returns true if one i32 is equal another, otherwise false
}
```

### Execution Loop

GrumpyVM's `mainloop` performs the following operations in order:

1. Check whether `halt == true`; if so, exit `mainloop`.
2. Increment the `pc`.
3. Check whether `pc - 1` is greater than or equal to the program length; if so, raise an error.
4. Execute the instruction at address `pc - 1`.
5. Loop to 1.

We implement this execution loop as the Rust function `exec` that takes as its second argument a mutable pointer to the VM state. The first argument, `d: &Debug`, is an immutable `DEBUG` flag.

```
pub fn exec(d: &Debug, s: &mut State) {
    'mainloop:loop {
        if s.halt { break 'mainloop }
        let pc = s.pc;
        s.pc = pc + 1;
        if pc >= s.program.len() {
            panic!("exec: pc out of bounds")
        }
        let i = &s.program[pc].clone();
        instr(i, s);
    }
    match d {
        Debug::DEBUG => {
            println!("{:?}", s)
        },
        Debug::NODEBUG => ()
    }
}
```

## Instruction Bytecode Format

An implementation of the GrumpyVM operates on a stream of variable-size bytecode instructions encoded according to the tables below. Every GrumpyVM bytecode file begins with a big-endian u32 encoding of the number of instructions in the program, followed by the binary encoding of each instruction as given below.

### Bytecode Representation of Values

GrumpyVM uses the following variable-size representation of values.

| Value | Bytecode |
| ----- | -------- | 
| Vunit | 0b00000000 | 
| Vi32(i:i32) | 0b00000001 byte3(i) byte2(i) byte1(i) byte0(i) (big-endian, two's complement) | 
| Vbool(true) | 0b00000010 | 
| Vbool(false)| 0b00000011 |
| Vloc(i:u32) | 0b00000100 byte3(i) byte2(i) byte1(i) byte0(i) (big-endian) |
| Vundef      | 0b00000101 |

The other value types (`Vsize`, `Vaddr`) may not appear in user programs. They therefore have no binary representation.

### Bytecode Representation of Unary Operators

| Unary Operator | Bytecode | 
| -------------- | -------- | 
| Neg            | 0b00000000 |

### Bytecode Representation of Binary Operators 

| Binary Operator | Bytecode   | 
| --------------- | ---------- | 
| Add             | 0b00000000 |
| Mul             | 0b00000001 |
| Sub             | 0b00000010 |
| Div             | 0b00000011 |
| Lt              | 0b00000100 |
| Eq              | 0b00000101 |

### Bytecode Representation of Instructions 

| Instruction | Bytecode | 
| ----------- | -------- | 
| Push(v)     | 0b00000000 Bytecode(v) | 
| Pop         | 0b00000001 |
| Peek(i:u32) | 0b00000010 byte3(i) byte2(i) byte1(i) byte0(i) (big-endian) |
| Unary(u)    | 0b00000011 Bytecode(u) |
| Binary(b)   | 0b00000100 Bytecode(b) | 
| Swap        | 0b00000101 |
| Alloc       | 0b00000110 | 
| Set         | 0b00000111 | 
| Get         | 0b00001000 | 
| Var(i:u32)  | 0b00001001 byte3(i) byte2(i) byte1(i) byte0(i) (big-endian) |
| Store(i:u32)| 0b00001010 byte3(i) byte2(i) byte1(i) byte0(i) (big-endian) |
| SetFrame(i:u32)| 0b00001011 byte3(i) byte2(i) byte1(i) byte0(i) (big-endian) |
| Call        | 0b00001100 |
| Ret         | 0b00001101 |
| Branch      | 0b00001110 |
| Halt        | 0b00001111 |

## Instructions

### Push(Val)

`Push(v)` pushes value `v` onto the stack. It leaves other elements of the VM's state unchanged.

Pre-state:

| stack | 
| ----- | 
| ... STACK_TOP |

Post-state: 

| stack | 
| ----- | 
| ... v STACK_TOP | 

### Pop

`Pop` a value from the top of the stack, discarding it.

Pre-state:

| stack | 
| ----- | 
| ... v STACK_TOP | 

Post-state:

| stack | 
| ----- | 
| ... STACK_TOP | 

### Peek(i)

Copy the value at the `i`th position from the top of the stack onto the top of the stack.

Pre-state:

| stack |
| ----- |
| vi ... v1 v0 STACK_TOP |

Post-state:

| stack |
| ----- |
| vi ... v1 v0 vi STACK_TOP |

### Unary(u)

GrumpyVM supports the following unary operators:

```
#[derive(Debug,Clone)]
pub enum Unop {
    Neg, //Negate a boolean value
}
```

`Unary(u)` applies the unary operation `u` to the top value on the stack, replacing it with the result.

Pre-state:

| stack |
| ----- |
| v STACK_TOP |

Post-state:

| stack |
| ----- |
| `[[u]]`(v) STACK_TOP |

where `[[u]]` is the interpretation of unary operator `u` as a function over values. For example: 

```
[[Neg]](Vbool(true)) = Vbool(false)
[[Neg]](Vbool(false)) = Vbool(true)
[[Neg]](_) = error
```

The VM must raise an error when a unary operation is applied to a value of the wrong type.

### Binary(b)

GrumpyVM supports the following binary operators: 

```
#[derive(Debug,Clone)]
pub enum Binop {
    Add, //Add two 32-bit integers
    Mul, //Multiply two 32-bit integers 
    Sub, //Subtract two 32-bit integers
    Div, //Divide two 32-bit integers
    Lt,  //Return true if one 32-bit integer is less than another, otherwise false
    Eq,  //Return true if one 32-bit integer equals another, otherwise false
}
```

`Binary(b)` applies the binary operator `b` to the top two values on the stack, replacing them with the result.

Pre-state:

| stack |
| ----- |
| v2 v1 STACK_TOP |

Post-state:

| stack |
| ----- |
| `[[b]]`(v1, v2) STACK_TOP |

where `[[b]]` is the interpretation of the binary operator `b` as a function over values. For example: 

```
[[Add]](Vi32(n1), Vi32(n2)) = Vi32(n1 + n2)
 [[Eq]](Vi32(n1), Vi32(n2)) = Vbool(n1 == n2)
...
```

### Swap

Swap the top two values on the stack.

Pre-state:

| stack |
| ----- |
| v2 v1 STACK_TOP |

Post-state:

| stack |
| ----- |
| v1 v2 STACK_TOP |

### Alloc

Allocate a new array on the heap, initialized to `size` copies of value `vinit`. Return on the top of the stack the address of the beginning of the array.

Pre-state:

| stack | heap |
| ----- | ---- | 
| ... Vi32(size) vinit STACK_TOP | ... HEAP_END | 

Post-state:

| stack | heap |
| ----- | ---- | 
| ... Vaddr(array_start) STACK_TOP | ... Vi32(size) vinit_1 vinit_2 ... vinit_size HEAP_END |
|                                  | ... ^array_start |

### Set

Store value `v` at heap address `base+idx+1` (element `idx` of the array beginning at `base`). Raise an error if `idx` is out of range for the array at address `base`.

Pre-state: 

| stack | heap |
| ----- | ---- |
| ... Vaddr(base) Vi32(idx) v STACK_TOP | ... v_old ... HEAP_END |
|                                       | ... ^base+idx+1 ... HEAP_END |

Post-state: 

| stack | heap |
| ----- | ---- |
| ... STACK_TOP | ... v ... HEAP_END |
|               | ... ^base+idx+1 ... HEAP_END |

### Get

Push the value contained at heap address `base+idx+1` (element `idx` of the array beginning at `base`). Raise an error if `idx` is out of range for the array at address `base`.

Pre-state: 

| stack | heap |
| ----- | ---- |
| ... Vaddr(base) Vi32(idx) STACK_TOP | ... v ... HEAP_END |
|                                     | ... ^base+idx+1 ... HEAP_END |

Post-state: 

| stack | heap |
| ----- | ---- |
| ... v STACK_TOP | ... v ... HEAP_END |
|                 | ... ^base+idx+1 ... HEAP_END |

### Var(i:u32)

Push onto the stack the value at stack address `fp+i`, or raise an error if `fp+i` is out of range.

Pre-state: 

| stack | 
| ----- | 
| ... v ... STACK_TOP |
| ... ^fp+i ... STACK_TOP |

Post-state: 

| stack | 
| ----- | 
| ... v ... v STACK_TOP |
| ... ^fp+i ... STACK_TOP |

### Store(i:u32)

Overwrite the value at stack address `fp+i` with value `vnew`, or raise an error if `fp+i` is out of range.

Pre-state: 

| stack | 
| ----- | 
| ... vold ... vnew STACK_TOP |
| ... ^fp+i ... STACK_TOP |

Post-state:

| stack | 
| ----- | 
| ... vnew ... STACK_TOP |
| ... ^fp+i ... STACK_TOP |

### SetFrame(i:u32)

1. Push the current frame pointer `cur_fp` onto the stack.
2. Set the frame pointer to `stack_len - i - 1`, where `stack_len` is the length of the stack after 1.

Pre-state: 

| fp | stack | 
| -- | ----- | 
| cur_fp | ... STACK_TOP | 

Post-state: 

| fp | stack | 
| -- | ----- | 
| stack_len - i - 1 | ... Vloc(cur_fp) STACK_TOP |

### Call

Call function at address `target` with arguments `varg1 varg2 ... vargN`. 
`cur_fp` is the frame pointer of the caller, which must be stored on the stack before the call instruction is executed. 

The result of `Call` is to: 
1. Pop the `target` address.
2. Push `cur_pc` onto the stack (points to the instruction directly after the call). 
3. Set the machine's `pc` register to `target`. 

`Call` raises an error if `target` is an invalid instruction.

Pre-state: 

| pc | fp | stack | 
| -- | -- | ----- | 
| cur_pc | cur_fp | ... varg1 varg2 ... vargN Vloc(caller_fp) Vloc(target) STACK_TOP |
|        |        | ... ^cur_fp ... STACK_TOP |

Post-state: 

| pc | fp | stack | 
| -- | -- | ----- | 
| target | cur_fp | ... varg1 varg2 ... vargN Vloc(caller_fp) Vloc(cur_pc) STACK_TOP |

### Ret

Function call return. 

1. Restore the caller's program counter `ret_pc` and frame pointer `ret_fp`. 
2. Pop arguments `varg1 varg2 ... vargN`. 
3. Push the return value `vret`, which is assumed to be the top value on the stack right before the `Ret` instruction is executed. 

Pre-state: 

| pc | fp | stack | 
| -- | -- | ----- | 
| cur_pc | cur_fp | ... varg1 varg2 ... vargN Vloc(ret_fp) Vloc(ret_pc) vret STACK_TOP |
|        |        | ... ^cur_fp ... STACK_TOP |

Post-state: 

| pc | fp | stack | 
| -- | -- | ----- | 
| ret_pc | ret_fp | ... vret STACK_TOP |

### Branch

Branch to address `target` if `b == true`. Raise an error if `target` is an invalid instruction location.

Pre-state: 

| pc | stack | 
| -- | ----- | 
| cur_pc | ... Vbool(b) Vloc(target) STACK_TOP | 

Post-state (`b == true`):

| pc | stack | 
| -- | ----- | 
| target | ... STACK_TOP | 

### Halt

Unconditionally halt the machine.

Pre-state: 

| halt | 
| ---- | 
|      | 

Post-state: 

| halt | 
| ---- | 
| true | 
