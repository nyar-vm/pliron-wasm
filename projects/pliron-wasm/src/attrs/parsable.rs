use pliron::parsable::{Parsable, StateStream};
use super::*;


impl Parsable for I32Attr {
    type Parsed = ();

    fn parse<'a>(state_stream: &mut StateStream<'a>) -> combine::error::ParseResult<Self::Parsed, combine::stream::easy::ParseError<StateStream<'a>>> {
        todo!()
    }
}