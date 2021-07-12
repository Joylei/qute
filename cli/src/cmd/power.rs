use crate::ctx::Context as PlatformContext;
use anyhow::{Context, Result};
use pico_args::Arguments;
use qute_ctrl::{Power, PowerRecoveryMode};

pub fn run(args: &mut Arguments, ctx: &PlatformContext) -> Result<()> {
    if ctx.get_opts().help {
        print_help();
        return Ok(());
    }
    let chip = ctx.get_platform()?;
    let mode: Option<PowerRecoveryMode> = args
        .opt_value_from_str(["-m", "--mode"])
        .with_context(|| "invalid input for mode")?;
    if let Some(mode) = mode {
        chip.set_power_recovery_mode(mode)?;
        println!("√ power recovery mode was set to {}", mode);
    } else {
        let mode = chip.get_power_recovery_mode()?;
        println!("power recovery mode: {} [{}]", mode, mode.desc());
    }
    Ok(())
}

fn print_help() {
    println!(
        r"qute power [OPTIONS]

Get or set power recovery mode

OPTIONS:
  -m, --mode [last|on|off]      Power recovery mode. optional
                                  last: keep previous power state
                                  on: turn on the NAS automatically
                                  off: keep the NAS turned off
  -h, --help                    Print this help text.
"
    );
}
