use List::{Cons, Nil};
use RcList::{RcCons, RcNil};
use std::rc::Rc;

pub fn run() {
	let _list = Cons(1, List::cons(2, List::cons(3, List::nil())));

	println!("{}", 1);

    let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
    println!("count @ a = {}", Rc::strong_count(&a));
    let b = RcCons(3, Rc::clone(&a));
    println!("count @ b = {}", Rc::strong_count(&a));
    {
        let c = RcCons(4, Rc::clone(&a));
        println!("count @ c = {}", Rc::strong_count(&a));
    }
    println!("count after c = {}", Rc::strong_count(&a));
}

enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

impl<T> List<T> {
    fn cons(value: T, next: Box<List<T>>) -> Box<List<T>> {
        Box::new(Cons(value, next))
    }

    fn nil() -> Box<List<T>> {
        Box::new(Nil)
    }
}

enum RcList<T> {
    RcCons(T, Rc<RcList<T>>),
    RcNil,
}