pub mod utils;

mod info;
mod manager;
mod plugin;
mod loader;
mod wrapper_loader;

pub mod variable;
pub mod function;

pub use info::*;
pub use manager::*;
pub use plugin::*;
pub use loader::*;
pub use wrapper_loader::*;

pub type Link<T> = std::rc::Rc<std::cell::RefCell<T>>;

#[cfg(feature = "derive")]
extern crate codegen;
#[cfg(feature = "derive")]
pub mod codegen;