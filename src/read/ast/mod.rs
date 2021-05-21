#[derive(Clone, Debug, PartialEq)]
pub enum Atom {
    Symbol(Symbol),
    Number(Number)
}

pub type Symbol = String;
pub type Number = f64;