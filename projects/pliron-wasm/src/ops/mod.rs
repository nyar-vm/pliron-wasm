use pliron::{declare_op, impl_attr};

mod parsable;
mod printable;
mod verify;
mod interface;

use pliron::parsable::{Parsable, StateStream};
use std::fmt::Formatter;
use pliron::context::Context;
use pliron::printable::{Printable, State};
use pliron::error::Result;
use pliron::common_traits::Verify;

include!("defs.rs");