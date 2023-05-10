pub mod error;
mod info;
mod manager;
mod plugin;
mod loader;

pub use info::*;
pub use manager::*;
pub use plugin::*;
pub use loader::*;

pub type Link<T> = std::rc::Rc<std::cell::RefCell<T>>;