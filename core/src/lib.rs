mod piece;
mod board;

// Re-export primary API components
pub use self::primary_api::*;

// Include macro definitions
#[macro_use]
mod macros {
    // Define your macros here
}

// Common traits used across your crate
pub mod traits {
    // Define your traits here
}

// Utility functions used across your crate
pub mod utils {
    // Define your utility functions here
}

// Primary API components
mod primary_api {
    // Define your primary API components here
}