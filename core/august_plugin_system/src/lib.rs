pub mod context;
pub mod utils;

mod info;
mod loader;
mod manager;
mod plugin;

pub mod function;
pub mod variable;

use std::sync::Arc;

pub use context::*;
pub use info::*;
pub use loader::*;
pub use manager::*;
pub use plugin::*;

use function::{Request, Function};

pub type Registry<T> = Vec<Arc<dyn Function<Output = T>>>;
pub type Requests = Vec<Request>;

#[cfg(feature = "derive")]
extern crate codegen;
#[cfg(feature = "derive")]
pub mod codegen;
