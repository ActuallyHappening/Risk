use risk::app_tracing;
use risk::prelude::*;

fn main() -> color_eyre::Result<()> {
	app_tracing::install_tracing("")?;
	trace!("Started tracing");

	Ok(())
}
