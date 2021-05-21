mod ast;
mod types;
mod parsers;

type ReadError = parsers::types::Error;

pub fn read(source: &str) -> Result<parsers::List, ReadError> {
    let (value, _) = parsers::read(source)?;
    Ok(value)
}