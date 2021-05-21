mod types;
mod parsers;

pub type Error = types::Error;

pub fn read(source: &str) -> Result<parsers::List, Error> {
    let (value, _) = parsers::read(source)?;
    Ok(value)
}