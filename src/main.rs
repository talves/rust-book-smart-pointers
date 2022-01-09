use rust_smart_pointers::{
    Chest, CustomSmartPointer,
    List::{Cons, Nil},
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
    }

    println!("Ending the app");
}
