use crate::ctx::Context as PlatformContext;
use anyhow::{Context, Result};
use qute_ctrl::{EupControl, SwitchState};

use pico_args::Arguments;

pub fn run(args: &mut Arguments, ctx: &PlatformContext) -> Result<()> {
    if ctx.get_opts().help {
        print_help();
        return Ok(());
    }
    let mode: Option<SwitchState> = args
        .opt_value_from_str(["-m", "--mode"])
        .with_context(|| "invalid input for mode")?;

    if let Some(mode) = mode {
        let chip = ctx.get_platform()?;
        chip.set_eup_state(mode)?;
        println!("√ eup was set to {}", mode);
        return Ok(());
    } else {
        let chip = ctx.get_platform()?;
        let mode = chip.get_eup_state()?;
        println!("eup state: {}", mode);
    }
    return Ok(());
}

fn print_help() {
    println!(
        r"qute eup

Turn EuP on/off. when EuP is disabled, the power consumption is slightly higher than 1W when NAS is powered off; when Eup is enabled, WOL will not work.

USAGE: qute eup [OPTIONS]

OPTIONS:
  -m, --mode [on|off]                EuP mode

DEFAULT:
  Get current EuP mode
"
    );
}
