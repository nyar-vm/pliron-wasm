mod printable;
mod parsable;
mod verify;
mod interface;

use pliron::parsable::{Parsable, StateStream};
use std::fmt::Formatter;
use pliron::{declare_op, impl_attr};
use pliron::context::Context;
use pliron::printable::{Printable, State};
use pliron::attribute::Attribute;

include!("defs.rs");