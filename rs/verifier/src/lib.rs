#[cfg(all(feature = "no-session", feature = "session"))]
compile_error!("feature \"no-session\" and feature \"session\" cannot be enabled at the same time");

use cfg_if::cfg_if;

cfg_if! {
  if #[cfg(feature = "session")] {
    mod session;
    pub use session::{verify_proof, verify_session};
  } else if #[cfg(feature = "no-session")] {
    mod no_session;
    pub use no_session::verify_proof;
  }
}
