use std::collections::HashMap;

use super::*;

pub mod execute;
pub mod query;
pub mod sudo;

mod types;
pub use types::*;

#[path = ""]
#[cfg(test)]
mod test {
    use super::*;
    mod execute_tests;
    mod query_tests;
}
