#include <stdio.h>
#include <stdlib.h>
#include <inttypes.h>
#include "gc.h"

#define STACK_SIZE 512
#define HEAP_SIZE 1024

static CHUNK* STACK[STACK_SIZE];
static CHUNK** stack_ptr = STACK+STACK_SIZE-1;

static CHUNK FROM_HEAP[HEAP_SIZE];
static CHUNK TO_HEAP[HEAP_SIZE];
static CHUNK* heap = FROM_HEAP;
static CHUNK* heap_ptr = FROM_HEAP; //next free chunk

static uintptr_t stack_size(void)
{
  return (uintptr_t)(STACK+STACK_SIZE-1 - stack_ptr);
}

static void print_chunk(CHUNK* p)
{
  if (p->tag == INT) {
    printf("%lx: CHUNK: tag=%d, data=%u, ",
	   (uintptr_t)p, p->tag, p->data.as_int);
  } else if (p->tag == PTR) {
    printf("%lx: CHUNK: tag=%d, data=%lx, ",
	   (uintptr_t)p, p->tag, (uintptr_t)p->data.as_ptr);
  } else {
    printf("bad chunk tag\n");
    exit (-4);
  }
  printf("forward_ptr: %lx\n", (uintptr_t)p->forward_ptr);
  return;
}

static void debug(void)
{
  CHUNK* cur = heap;

  printf("\n*** Debug\n");
  printf("heap = %lx\n", (uintptr_t)heap);
  while (cur < heap_ptr) {
    print_chunk(cur);
    cur++;
  }
  printf("heap+heap_ptr\n");
  printf("stack = %lu chunk(s)\n", stack_size());
}

void push(CHUNK* root)
{
  if (stack_size() >= STACK_SIZE) {
    printf("stack overflow\n");
    exit(-1);
  }
  *stack_ptr = root;
  stack_ptr--;
  return;  
}

void pop(void)
{
  if (stack_size() <= 0) {
    printf("stack underflow\n");
    exit (-2);
  }
  stack_ptr++;
  return;
}

CHUNK* new(void)
{
  if (heap_ptr >= heap+HEAP_SIZE) {
    printf("not enough memory\n");
    exit (-3);
  }
  CHUNK* ptr = heap_ptr;
  ptr->forward_ptr = NULL;
  heap_ptr++;
  return ptr;
}

void copy(CHUNK* from, CHUNK* to)
{
  to->tag = from->tag;
  to->forward_ptr = NULL;
  if (from->tag == INT) {
    to->data.as_int = from->data.as_int;
  } else if (from->tag == PTR) {
    to->data.as_ptr = from->data.as_ptr;
  } else {
    printf("bad chunk tag\n");
    exit (-4);
  }
  //Don't forget to leave a forwarding address!
  from->forward_ptr = to;
  return;
}

void gc(void)
{
  //scan = address of first block in the new TO space
  CHUNK* scan = (heap==FROM_HEAP) ? TO_HEAP : FROM_HEAP;
  CHUNK* next = scan; //next free chunk in new TO space
  //cur = address of last chunk in stack  
  CHUNK** cur = STACK+STACK_SIZE-1; 

  int ncopied_roots = 0;
  int ncopied_blocks = 0;
 
  printf("\n*** Starting GC\n");
  
  //Copy the roots
  printf("Copying roots\n");
  while (cur > stack_ptr) {
    copy(*cur, next);
    //Don't forget to update pointer!
    *cur = next;
    ncopied_roots++;
    print_chunk(*cur);
    cur--;
    next++;
  }

  printf("Scanning chunks\n");  
  while (scan < next) { //Terminate when scan == next.
    if (scan->tag == PTR) {
      //Special-case on whether the object pointed to
      //has been copied yet:
      if (scan->data.as_ptr->forward_ptr == NULL) {
	copy(scan->data.as_ptr, next);
	ncopied_blocks++;
	//make sure to update pointer
	scan->data.as_ptr = next;	
	//only increment next if we actually copied a block
	next++;
      } else { //was already copied
	scan->data.as_ptr = scan->data.as_ptr->forward_ptr;
      }
      print_chunk(scan);      
    }
    scan++;
  }
  printf("Copied roots = %d, copied blocks = %d\n",
	 ncopied_roots, ncopied_blocks);

  heap = (heap==FROM_HEAP) ? TO_HEAP : FROM_HEAP;
  heap_ptr = next;
  return;
}

int main(void)
{
  CHUNK* c1 = new();
  CHUNK* c2 = new();  

  /* c1->c2
     c2->c2 */
  
  printf("\nc1 points to c2\n");
  c1->tag = PTR;
  c1->data.as_ptr = c2;
  c2->tag = PTR;
  c2->data.as_ptr = c2;
  debug();

  push(c1);

  gc();
  debug();

  /* printf("\npush c1\n"); */
  /* push(c1); */

  /* gc();   */
  /* debug(); */

  /* printf("\npop c1\n"); */
  /* pop(); */
  /* debug(); */
  /* gc(); */
  /* debug(); */

  /* printf("\nclear stack, allocate 20 blocks, gc\n");     */
  /* for (int i = 0; i < 10; i++) { */
  /*   c = new(); */
  /*   d = new(); */
  /*   c->tag = PTR; */
  /*   c->data.as_ptr = d; */
  /*   push(c); */
  /* } */
  /* debug(); */
  /* gc(); */
  /* debug(); */
  /* gc(); */
  /* debug(); */

  /* printf("\ntry gc'ing A LOT\n"); */
  /* for (int i = 0; i < 10000; i++) { */
  /*   gc(); */
  /* } */
  /* debug(); */
  return 0;
}

  
