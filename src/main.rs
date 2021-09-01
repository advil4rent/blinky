use gpio_cdev::{Chip, LineRequestFlags};
use quicli::prelude::*;
use std::thread::sleep;
use std::time::{Duration, Instant};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    /// The gpiochip device (e.g. /dev/gpiochip0)
    chip: String,
    /// The offset of the GPIO line for the provided chip
    line: u32,
    /// Period in milliseconds
    period_ms: u64,
    /// Duration over which to blink in milliseconds
    duration_ms: u64,
}

fn do_main(args: Cli) -> std::result::Result<(), gpio_cdev::Error> {
    let mut chip = Chip::new(args.chip)?;

    // NOTE: we set the default value to the desired state so
    // setting it separately is not required
    let handle = chip
        .get_line(args.line)?
        .request(LineRequestFlags::OUTPUT, 1, "blinky")?;

    let duration = Duration::from_millis(args.duration_ms);
    let start_time = Instant::now();
    while start_time.elapsed() < duration {
        sleep(Duration::from_millis(args.period_ms));
        handle.set_value(0)?;
        sleep(Duration::from_millis(args.period_ms));
        handle.set_value(1)?;
    }

    Ok(())
}

fn main() -> CliResult {
    let args = Cli::from_args();
    do_main(args).or_else(|e| {
        error!("{:?}", e);
        Ok(())
    })
}
