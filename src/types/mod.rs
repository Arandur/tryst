//mod number;
mod cons;
mod symbol;

//pub use number::*;
pub use cons::*;
pub use symbol::*;

pub type Number = f64;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Atom(Atom),
    Cons(Cons)
}

#[derive(Clone, Debug, PartialEq)]
pub enum Atom {
    Nil,
    Boolean(bool),
    // Byte(u8),
    Character(char),
    Number(Number),
    Symbol(Symbol),
    String(String)
}

#[doc(hidden)]
#[macro_export]
macro_rules! dispatchable {
    ($enum:ident { $variant:ident $( , $rest:ident )* }) => {
        macro_rules! dispatch {
            (match $match:ident { $inner:ident => $ret:expr }) => {
                match $match {
                    $enum::$variant($inner) => $ret,
                    $(
                        $enum::$rest($inner) => $ret,
                    )*
                }
            };
            (fn $func:ident(self) -> $ret:ty) => {
                fn $func(self) -> $ret {
                    dispatch!(match self { val => val.$func().into() })
                }
            };
            (fn $func:ident(&self) -> $ret:ty) => {
                fn $func(&self) -> $ret {
                    dispatch!(match self { val => val.$func().into() })
                }
            };
            (fn $func:ident(&mut self) -> $ret:ty) => {
                fn $func(&mut self) -> $ret {
                    dispatch!(match self { val => val.$func().into() })
                }
            };
        }
    };
}