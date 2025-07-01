//! Test that repository derive fails on non-struct types

use sqlx_repository::prelude::*;

#[derive(Debug, Clone, Repository)]
#[repository(table = "users")]
pub enum User {  // This should fail - Repository only works on structs
    Active,
    Inactive,
}

fn main() {}