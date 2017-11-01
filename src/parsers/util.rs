use pom::parser::*;
use std::fmt::*;
use pom::{Input, Train};


pub trait ExpectParser<'a, I, O> where I: 'static, O: 'static {
    fn expect(self, msg: &'static str) -> Parser<'a, I, O>;
}


impl<'a, I: 'static, O: 'static> ExpectParser<'a, I, O> for Parser <'a, I, O> {
    fn expect(self, msg: &'static str) -> Parser<'a, I, O>
        where I: 'static
    {
        Parser::new(move |input: &mut Input<I>| {
            Ok(self.parse(input).expect(msg))
        })
    }
}

pub fn boundary<'a, I>(p: Parser<'a, u8, I>) -> Parser<'a, u8, I>
    where I: 'static
{
    whitespace() * p - whitespace()
}

pub fn sym_s<'a>(symbol: u8) -> Parser<'a, u8, u8> {
    boundary(sym(symbol))
}
pub fn seq_s<'a,  T>(train: &'static T) -> Parser<'a, u8, Vec<u8>>
    where T: Train<u8> + ::std::marker::Sized
{
    boundary(seq(train))
}
pub fn one_of_s<'a, S>(symbols: &'static S) -> Parser<'a, u8, u8>
    where S: ::pom::set::Set<u8> + ::std::marker::Sized
{
    one_of_skip(symbols, whitespace)
}

pub fn whitespace() -> Parser<'static, u8, ()> {
    (one_of(b" \t\n\r").discard()
        |(seq(b"//")
            * none_of(b"\n").repeat(..)
            * (sym(b'\n').discard() | end()
        )
    ))
        .repeat(0..).discard()
}

pub fn one_of_skip<'a, I, S>(symbols: &'static S, whitespace: fn() -> Parser<'a, I, ()>)  -> Parser<'a, I, I>
    where I: Copy + PartialEq + Display + Debug + 'static,
          S: ::pom::set::Set<I> + ::std::marker::Sized
{
    whitespace() * one_of(symbols) - whitespace()
}


pub fn flatten<T>(mut vec: Vec<Vec<T>>) -> Vec<T> {
    let mut result = Vec::new();

    vec.drain(..).for_each(|mut v|v.drain(..).for_each(|v_inner|result.push(v_inner)));

    result
}

pub fn push_identity<T>(pair: (T, Vec<T>)) -> Vec<T> {
    let (first, mut vec) = pair;
    vec.push(first);
    vec
}