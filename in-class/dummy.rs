struct Dummy { a: i32, b: i32 }

fn take(res: Box<Dummy>) { //ownership of res "moved" to take
} //res deallocated here

fn foo() {
    let mut res: Box<Dummy>;
    res = Box::new(Dummy {a: 0, b: 0});
    //take(res);
    res.a = 2048;
}

fn main() {
    foo()
}
