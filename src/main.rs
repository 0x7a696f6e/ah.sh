use ah::{cli, config, log, util};

fn main() {
    config::init();
    cli::complete_dynamic();

    log::with_logging(|| {
        cli::run()?;
        Ok(())
    })
    .unwrap_or_else(util::exit_with_error)
}
