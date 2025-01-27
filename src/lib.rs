//! Library for accessing the [Untis](https://www.untis.at) JSON-RPC API.
//!
//! The core of this crate is the `untis::Client` struct. You can log in using `untis::Client::login()`.
//!
//! ## API
//! This client uses the public Untis JSON-RPC API, which only has read-only, limited access.
//!
//! ## Examples
//! ```rust
//! #[tokio::main]
//! async fn main() -> Result<(), untis::Error> {
//!   let results = untis::schools::search("School Name").await?;
//!   let school = match results.first() {
//!     None => {
//!       println!("No school found");
//!       return Ok(());
//!     },
//!     Some(v) => v
//!   };
//!
//!   let mut client = school.client_login("username", "password").await?;
//!
//!   let timetable = client.own_timetable_current_week().await?;
//!
//!   // profit
//!
//!   Ok(())
//! }
//! ```
//! For more examples, see the `examples/` directory.

mod client;
mod datetime;
mod error;
mod params;
mod resources;

pub mod jsonrpc;
pub mod schools;

pub use client::Client;
pub use datetime::*;
pub use error::Error;
pub use resources::*;
