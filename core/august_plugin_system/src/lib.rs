pub mod utils;
pub mod context;

mod info;
mod manager;
mod plugin;
mod builder;
mod loader;
mod wrapper_loader;

pub mod variable;
pub mod function;

use std::sync::Arc;

pub use info::*;
pub use manager::*;
pub use plugin::*;
pub use builder::*;
pub use loader::*;
pub use wrapper_loader::*;

use function::{Function, Request};

pub type Link<T> = std::rc::Rc<std::cell::RefCell<T>>;
pub type Registry = Vec<Arc<Function>>;
pub type Requests = Vec<Request>;

#[cfg(feature = "derive")]
extern crate codegen;
#[cfg(feature = "derive")]
pub mod codegen;