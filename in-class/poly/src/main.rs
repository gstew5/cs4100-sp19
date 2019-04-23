#[derive(Clone)]
enum List<T> {
    Nil,
    Cons(T, Box<List<T>>)
}

use List::{Nil, Cons};

fn use_list() {
    let l1: List<i32> = Cons(1, Box::new(Cons(2, Box::new(Nil))));
    let _l2: List<List<i32>> = Cons(l1.clone(),
                                   Box::new(Cons(l1, Box::new(Nil))));         
}

fn append<T>(l1: List<T>, l2: List<T>) -> List<T> {
    match l1 {
        Nil => l2,
        Cons(f, r) => Cons(f, Box::new(append(*r, l2)))
    }
}

//Expansion/Monomorphization
#[derive(Clone)]
enum IntList {
    Nil,
    Cons(i32, Box<IntList>)
}

#[derive(Clone)]
enum IntListList {
    Nil,
    Cons(IntList, Box<IntListList>)
}

fn use_intlist() {
    let l1: IntList =
        IntList::Cons(1, Box::new(IntList::Cons(2, Box::new(IntList::Nil))));
    let _l2: IntListList =
        IntListList::Cons(
            l1.clone(),
            Box::new(IntListList::Cons(
                l1,
                Box::new(IntListList::Nil))));         
}

//Tagging
fn swap(p: (usize, usize)) -> (usize, usize) {
    (p.1, p.0)
}

fn use_tags() {
    let a_char: u8;
    let b_char: u8 = 4;
    let c_char: u8 = 5;

    //Tag the characters to distinguish them from pointers
    let a: usize;
    let b: usize = ((b_char << 1) | 1) as usize;
    let c: usize = ((c_char << 1) | 1) as usize;
    
    //Add by untagging, then adding, then re-tagging
    a = (((b >> 1) + (c >> 1)) << 1) | 1;
    a_char = (a >> 1) as u8;

    let c = swap(swap((a, c))).1;
    let c_char = c >> 1;

    println!("a_char = {}, b_char = {}", a_char, c_char);
}

fn main() {
    use_list();
    use_intlist();
    use_tags();
}
