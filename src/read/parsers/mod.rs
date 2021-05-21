mod combinators;
pub mod types;

use crate::types as core_types;
use crate::types::lir::{Span, WithSpan};

use super::types::{Source, Result};
use types::*;

use crate::types::number::read as parse_number;
use crate::types::symbol::read as parse_symbol;

pub type Atom = WithSpan<core_types::Atom>;

fn parse_atom<'a>(source: &Source<'a>) -> Result<'a, Atom> {
    if source.inner.starts_with(char::is_numeric) {
        parse_number.map(core_types::Atom::Number).with_span().parse(source)
    } else {
        parse_symbol.map(core_types::Atom::Symbol).with_span().parse(source)
    }
}

pub type List = WithSpan<Vec<Form>>;

fn parse_list<'a>(source: &Source<'a>) -> Result<'a, List> {
    let parse_open_paren = combinators::literal_parser("(");
    let parse_close_paren = combinators::literal_parser(")");
    
    let parse_forms = parse_form.whitespace_delimited();

    let parse_list = parse_forms.surrounded(parse_open_paren, parse_close_paren)
        .map(|(_, contents, _)| contents);

    let ( WithSpan { value, span }, next ) = parse_list.with_span().parse(source)?;

    let span = Span::new(source.position, span.end);

    Ok((WithSpan { value, span }, next))
}

fn parse_list_contents<'a>(source: &Source<'a>) -> Result<'a, List> {
    parse_form.whitespace_delimited().with_span().parse(source)
}

#[derive(Clone, Debug, PartialEq)]
pub enum Form {
    Atom(Atom),
    List(List)
}

fn parse_form<'a>(source: &Source<'a>) -> Result<'a, Form> {
    let (value, next) = parse_atom.or(parse_list).map(|e| match e {
        Either::Left(atom) => Form::Atom(atom),
        Either::Right(list) => Form::List(list)
    }).parse(source)?;

    Ok((value, next))
}

pub fn read(source: &str) -> Result<List> {
    parse_list_contents(&Source::new(source))
}