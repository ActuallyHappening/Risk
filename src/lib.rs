#[path = "tracing.rs"]
pub mod app_tracing;

pub mod prelude {
	pub use tracing::{debug, error, info, trace, warn};
}
