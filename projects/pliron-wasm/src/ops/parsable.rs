
use super::*;


impl Parsable for UnreachableOp {
    type Parsed = ();

    fn parse<'a>(state_stream: &mut StateStream<'a>) -> combine::error::ParseResult<Self::Parsed, combine::stream::easy::ParseError<StateStream<'a>>> {
        todo!()
    }
}