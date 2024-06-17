pub mod execute;
pub mod query;

mod types;
pub use types::*;

use super::*;

// This doesn't like being defined in query.rs
impl From<query::QueryMsg> for QueryMsg {
    fn from(value: query::QueryMsg) -> Self {
        Self::Staking(value)
    }
}

#[path = ""]
#[cfg(test)]
mod test {
    use super::*;
    mod execute_tests;
    mod query_tests;
}
