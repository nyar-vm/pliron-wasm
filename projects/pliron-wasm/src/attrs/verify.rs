
use super::*;

impl Verify for I32Attr {
    fn verify(&self, _: &Context) -> pliron::error::Result<()> {
        Ok(())
    }
}

impl Verify for I64Attr {
    fn verify(&self, _: &Context) -> pliron::error::Result<()> {
        Ok(())
    }
}
impl Verify for F32Attr {
    fn verify(&self, _: &Context) -> pliron::error::Result<()> {
        Ok(())
    }
}
impl Verify for F64Attr {
    fn verify(&self, _: &Context) -> pliron::error::Result<()> {
        Ok(())
    }
}

impl Verify for V128Attr {
    fn verify(&self, _: &Context) -> pliron::error::Result<()> {
        Ok(())
    }
}


impl Verify for FuncRefAttr {
    fn verify(&self, _: &Context) -> pliron::error::Result<()> {
        Ok(())
    }
}


impl Verify for ExternRefAttr {
    fn verify(&self, _: &Context) -> pliron::error::Result<()> {
        Ok(())
    }
}

impl Verify for ValueAttr {
    fn verify(&self, _: &Context) -> pliron::error::Result<()> {
       Ok(())
    }
}