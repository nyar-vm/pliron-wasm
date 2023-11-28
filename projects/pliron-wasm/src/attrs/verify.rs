use pliron::common_traits::Verify;
use super::*;

impl Verify for I32Attr {
    fn verify(&self, ctx: &Context) -> pliron::error::Result<()> {
        todo!()
    }
}

impl Verify for FuncrefAttr {
    fn verify(&self, ctx: &Context) -> pliron::error::Result<()> {
        todo!()
    }
}