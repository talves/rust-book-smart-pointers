use std::ops::Deref;

use rust_smart_pointers::{
    List::{Cons, Nil},
    MyBox,
};

fn main() {
    let a = 5;
    let c: &str = "cat🤣";
    let b = Box::new((a, c));
    println!("b = {:?}", b);

    // cons list
    let _list = Box::new(Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))))));
    // println!("cons list = (:?)", list);

    // dereference operator, *
    // can only be used on types that have the Deref trait
    let x = 5;
    let y = &x;
    let z = Box::new(x); // makes a copied value of x, then references it

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
