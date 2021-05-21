/*
 * An atom is anything which is not a list. This is an open class;
 * we're starting out with only numbers and symbols, but the definition
 * can and will grow as we add more types.
 */

/*
 * Each type of atom gets its own module.
 */
pub mod number;
pub mod symbol;

pub use number::Number;
pub use symbol::Symbol;

#[derive(Clone, Debug, PartialEq)]
pub enum Atom {
    Number(Number),
    Symbol(Symbol)
}