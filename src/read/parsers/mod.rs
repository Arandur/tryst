mod combinators;
pub mod types;

use super::types as read_types;
use super::types::*;
use types::*;
use super::ast;

pub type Atom = WithSpan<ast::Atom>;

fn parse_atom<'a>(source: &Source<'a>) -> Result<'a, Atom> {
    if source.inner.starts_with(char::is_numeric) {
        parse_number.map(ast::Atom::Number).with_span().parse(source)
    } else {
        parse_symbol.map(ast::Atom::Symbol).with_span().parse(source)
    }
}

fn parse_number<'a>(source: &Source<'a>) -> Result<'a, ast::Number> {
    let Source { inner, position } = source;

    let end = inner.find(|c: char| c.is_whitespace() || c == ')').unwrap_or_else(|| inner.len());
    let ( number, next ) = inner.split_at(end);
    let next = Source { inner: next, position: position.advance(number) };

    number.parse()
        .map(|n| (n, next))
        .map_err(|_| Error {
            kind: ErrorKind::Expected("a number"),
            position: *position
        })
}

fn parse_symbol<'a>(source: &Source<'a>) -> Result<'a, ast::Symbol> {
    let Source { inner, position } = source;

    // Easier to enumerate the characters a symbol can't start with!
    if inner.starts_with('(') {
        return Err(Error {
            kind: ErrorKind::Expected("a symbol"),
            position: *position
        });
    }

    let end = inner.find(|c: char| c.is_whitespace() || c == ')').unwrap_or_else(|| inner.len());

    if end == 0 {
        return Err(Error {
            kind: ErrorKind::Expected("a symbol"),
            position: *position
        })
    }

    let ( symbol, next ) = inner.split_at(end);
    let next = Source { inner: next, position: position.advance(symbol) };

    Ok((symbol.to_string(), next))
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

pub fn read<'a>(source: &'a str) -> Result<'a, List> {
    parse_list_contents(&Source::new(source))
}