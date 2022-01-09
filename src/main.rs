use std::rc::Rc;

use rust_smart_pointers::{
    Chest, CustomSmartPointer,
    List::{Cons, Nil},
    RcList::{Cons as RcCons, Nil as RcNil},
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
    let y = Chest::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    fn hello(name: &str) {
        println!("Hello, {}!", name);
    }
    let m = Chest::new(String::from("World"));
    hello(&m); // Implicit deref coercion
    hello(&(*m)[..]); // what we would have had to write to do the above

    // Drop
    {
        let _c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        let _d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        println!("CustomSmartPointers created.");
    } // Rust automatically calls drop for us when _c, _d lose scope

    // Multiple ownership
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a)); // fails originally
    // let's use our new list type (RcList)
    let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
    let b = RcCons(3, Rc::clone(&a));
    let c = RcCons(4, Rc::clone(&a)); // c now can point to the same reference as b (&a)
    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = {:?}", c);
    println!("count after creating b & c = {}", Rc::strong_count(&a));

    println!("Ending the app");
}
