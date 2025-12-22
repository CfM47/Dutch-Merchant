mod models;
pub use models::*;

mod generator;
pub use generator::generate_instance;

/// The methods defined here expect valid inputs
mod matrix;

#[cfg(test)]
mod matrix_test;

#[cfg(test)]
mod generator_test;
