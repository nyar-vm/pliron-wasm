mod printable;
mod parsable;
mod verify;
mod interface;


use std::fmt::Formatter;
use pliron::{ impl_attr};
use pliron::context::Context;
use pliron::printable::{Printable, State};
use pliron::attribute::Attribute;
use pliron::common_traits::Verify;

include!("defs.rs");