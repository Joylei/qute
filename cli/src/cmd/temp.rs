use crate::{ctx::Context, utils::temp_c2f};
use anyhow::Result;
use qute_ctrl::Temperature;

use pico_args::Arguments;

pub fn run(args: &mut Arguments, ctx: &Context) -> Result<()> {
    if ctx.get_opts().help {
        print_help();
        return Ok(());
    }
    let cmd = args.subcommand().ok().flatten().unwrap_or_default();
    // let index: u8 = args
    //     .opt_value_from_str(["-i", "--index"])
    //     .map_err(|e| Error::with_chain(e, "invalid input for index"))?
    //     .unwrap_or(0);
    match cmd.as_str() {
        "cpu" => return process(args, ctx, 0),
        "sys" => return process(args, ctx, 5),
        _ => {}
    }
    print_help();
    Ok(())
}

fn process(_args: &mut Arguments, ctx: &Context, index: u8) -> Result<()> {
    let tag = match index {
        idx if idx < 5 => "cpu",
        5 => "sys",
        _ => return Err(anyhow!("Not supported")),
    };
    let chip = ctx.get_platform()?;
    let val = chip.get_temperature(index)?;
    println!("{} temperature: {:.1} ℃ / {} ℉", tag, val, temp_c2f(val));
    Ok(())
}

fn print_help() {
    println!(
        r"qute temp [OPTIONS] [COMMANDS]

Fetch system/hardware temperature

OPTIONS:
  -h, --help                   Print this help text.

COMMANDS:
  cpu                          Fetch cpu temperature
  sys                           Fetch sys temperature
"
    );
}
