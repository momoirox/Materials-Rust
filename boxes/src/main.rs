
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    println!("Hello, world!");
    //smart pointers 
    //reference counting owners
    let _list  = Cons(1, Box::new(Cons(2,Box::new(Nil))));

    let x = 5;
    let y = Box::new(x); //has deref trait
    assert_eq!(5,x);
    assert_eq!(5,*y);
}
