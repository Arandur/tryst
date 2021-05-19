use super::Value;

use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct Cons {
    car: Rc<Value>, 
    cdr: Rc<Value>
}

pub fn car(cons: &Cons) -> &Rc<Value> { &cons.car }
pub fn cdr(cons: &Cons) -> &Rc<Value> { &cons.cdr }