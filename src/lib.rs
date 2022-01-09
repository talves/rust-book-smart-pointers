use std::ops::Deref;

pub enum List {
    //Boxes provide only the indirection and heap allocation
    Cons(i32, Box<List>),
    Nil,
}

pub struct Chest<T>(T);

impl<T> Chest<T> {
    pub fn new(x: T) -> Chest<T> {
        Chest(x)
    }
}

// to be able to deref a type of MyBox we must implement Deref
impl<T> Deref for Chest<T> {
    type Target = T; // defines an associated type for the Deref trait to use

    fn deref(&self) -> &Self::Target {
        &self.0 // returns a reference to the value we want to access
    }
}

// implements Drop
pub struct CustomSmartPointer {
    pub data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
