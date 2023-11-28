

mod parsable;
mod printable;
mod verify;
mod interface;

use pliron::{declare_op};
use std::fmt::Formatter;
use pliron::context::Context;
use pliron::printable::{Printable, State};
use pliron::error::Result;
use pliron::common_traits::Verify;

include!("defs.rs");