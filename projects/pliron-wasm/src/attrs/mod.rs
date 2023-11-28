mod printable;
mod parsable;
mod verify;
mod interfaces;

use pliron::parsable::{Parsable, StateStream};
use std::fmt::Formatter;
use pliron::{declare_op, impl_attr};
use pliron::context::Context;
use pliron::printable::{Printable, State};

include!("defs.rs");