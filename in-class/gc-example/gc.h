#ifndef __GC_H
#define __GC_H

enum TAG {
  INT,
  PTR
};

typedef struct chunk {
  enum TAG tag;
  //The forward pointer is used to determine
  //whether a particular chunk has already been
  //copied from FROM to TO; if it has,
  //forward_ptr will equal the (nonnull) address
  //of the block's new location in TO space.
  struct chunk* forward_ptr;
  union {
    unsigned int as_int;
    struct chunk* as_ptr;
  } data;
} CHUNK;

void push(CHUNK* root); //push a chunk pointer onto the stack
void pop(void);         //pop a chunk pointer from the stack
CHUNK* new(void);       //allocate a new chunk
void gc(void);          //garbage-collect chunks

#endif 
