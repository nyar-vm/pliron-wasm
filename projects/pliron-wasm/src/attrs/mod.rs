mod printable;
mod parsable;

use std::fmt::Formatter;
use pliron::{declare_op, impl_attr};
use pliron::context::Context;
use pliron::printable::{Printable, State};

impl_attr!(I32Attr, "i32", "wasm");
pub struct I32Attr {
    value: i32
}
impl_attr!(FuncrefAttr, "funcref", "wasm");
pub struct FuncrefAttr {

}

