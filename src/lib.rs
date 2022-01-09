use std::{
    cell::RefCell,
    ops::Deref,
    rc::{Rc, Weak},
};

pub enum List {
    //Boxes provide only the indirection and heap allocation
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
pub enum RcList {
    //Rc provides for a reference count to keep track of multiple references to our List
    Cons(i32, Rc<RcList>),
    Nil,
}

#[derive(Debug)]
pub enum RcRefCellList {
    Cons(Rc<RefCell<i32>>, Rc<RcRefCellList>),
    Nil,
}

// for demonstration of a reference cycle that could leak memory
#[derive(Debug)]
pub enum RefCycleList {
    Cons(i32, RefCell<Rc<RefCycleList>>),
    Nil,
}

impl RefCycleList {
    pub fn tail(&self) -> Option<&RefCell<Rc<RefCycleList>>> {
        match self {
            crate::RefCycleList::Cons(_, item) => Some(item),
            crate::RefCycleList::Nil => None,
        }
    }
}

// Using Weak<T> to avoid reference cycle
#[derive(Debug)]
pub struct Node {
    pub value: i32,
    pub parent: RefCell<Weak<Node>>,
    pub children: RefCell<Vec<Rc<Node>>>,
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
        //cleanup code would usually go here, but we are using this to show what happens on drop
        println!("Dropping CustomSmartPointer with data `{}`💀!", self.data);
    }
}
