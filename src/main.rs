use std::{cell::RefCell, rc::Rc};

use rust_smart_pointers::{
    Chest, CustomSmartPointer,
    List::{Cons, Nil},
    RcList::{Cons as RcCons, Nil as RcNil},
    RcRefCellList::{Cons as RcRefCellCons, Nil as RcRefCellNil},
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

    // RefCell<T> allows mutable borrows checked at runtime
    // Mutating the value inside an immutable value is the interior mutability pattern
    // We created RcRefCellList to handle interior mutability pattern
    // let x = 5;
    // let y = &mut x; // fails
    let x = Rc::new(RefCell::new(5));
    let a = Rc::new(RcRefCellCons(Rc::clone(&x), Rc::new(RcRefCellNil)));
    println!("a = {:?}", a);
    let b = RcRefCellCons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = RcRefCellCons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    println!("b = {:?}", b);
    println!("c = {:?}", c);
    // change the inner value of x although it is immutable, we can deref and borrow_mut() to change the inner value
    *x.borrow_mut() += 100;
    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = {:?}", c);
    // Note: this is a single thread implementation. We would use Mutex<T> to do this across threads Ch. 16

    println!("Ending the app");
}
