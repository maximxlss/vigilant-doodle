#![warn(clippy::all, rust_2018_idioms)]

mod tasks;
pub use tasks::Tasks;

mod notes;
pub use notes::Notes;

mod base;
pub use base::VigilantDoodle;

pub mod utils;
