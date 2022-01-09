use std::ops::Deref;

pub enum List {
    //Boxes provide only the indirection and heap allocation
    Cons(i32, Box<List>),
    Nil,
}

pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// to be able to deref a type of MyBox we must implement Deref
impl<T> Deref for MyBox<T> {
    type Target = T; // defines an associated type for the Deref trait to use

    fn deref(&self) -> &Self::Target {
        &self.0 // returns a reference to the value we want to access
    }
}
