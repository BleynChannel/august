pub mod utils;
pub mod context;

mod info;
mod manager;
mod plugin;
mod loader;

pub mod variable;
pub mod function;

use std::sync::Arc;

pub use info::*;
pub use manager::*;
pub use plugin::*;
pub use context::*;
pub use loader::*;

use function::{Function, Request};

pub type Registry = Vec<Arc<Function>>;
pub type Requests = Vec<Request>;

#[cfg(feature = "derive")]
extern crate codegen;
#[cfg(feature = "derive")]
pub mod codegen;