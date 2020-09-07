use crate::ctx::Context;
use crate::errors::*;
use qute_ctrl::FanControl;

use pico_args::Arguments;

pub fn run(args: &mut Arguments, ctx: &Context) -> Result<()> {
    let cmd = args.subcommand().ok().flatten().unwrap_or_default();
    match cmd.as_str() {
        "pwm" => return process_pwm(args, ctx),
        "speed" => return process_speed(args, ctx),
        "status" => return process_status(args, ctx),
        _ => {}
    }
    print_help();
    return Ok(());
}

fn print_help() {
    println!(
        r"qute fan [OPTIONS] [COMMANDS]
Fan control

OPTIONS:
  -h, --help                 Print this help text.

COMMANDS:
  pwm                        Get or set pwm
  speed                      Get current speed
  status                      Get fan status
"
    );
}

fn process_pwm(args: &mut Arguments, ctx: &Context) -> Result<()> {
    if ctx.get_opts().help {
        println!(
            r"qute fan pwm [OPTIONS]

Get or set fan PWM

OPTIONS:
  -i, --index                 Fan Index. Optional.
      --value                 PWM value [0-255]; if specified, will set PWM speed, otherwise will return the current PWM speed.
  -h, --help                  Print this help text.
"
        );
        return Ok(());
    }
    let index: u8 = args
        .opt_value_from_str(["-i", "--index"])
        .map_err(|e| Error::with_chain(e, "invalid input for index"))?
        .unwrap_or(0);
    let config = ctx.get_config();
    if index >= config.fan.len() as u8 {
        return Err(format!("fan pwm: invalid fan index {}", index).into());
    }
    let chip = ctx.get_platform()?;
    let val: Option<u8> = args
        .opt_value_from_str(["-v", "--value"])
        .map_err(|e| Error::with_chain(e, "invalid input for value"))?;
    if let Some(val) = val {
        //set val
        chip.set_fan_speed(index, val)?;
        println!("√ PWM of fan {} was set to {}", index, val);
    } else {
        //get val
        let val = chip.get_fan_pwm(index)?;
        println!("fan {} pwm: {}", index, val);
    }
    Ok(())
}

fn process_speed(args: &mut Arguments, ctx: &Context) -> Result<()> {
    if ctx.get_opts().help {
        println!(
            r"qute fan speed [OPTIONS]

Get fan speed in RPM

OPTIONS:
  -i, --index                   Fan Index. Optional.
  -h, --help                    Print this help text.
"
        );
        return Ok(());
    }
    let index: u8 = args
        .opt_value_from_str(["-i", "--index"])
        .map_err(|e| Error::with_chain(e, "invalid input for index"))?
        .unwrap_or(0);
    let config = ctx.get_config();
    if index >= config.fan.len() as u8 {
        return Err(format!("fan speed: invalid fan index {}", index).into());
    }
    //get speed
    let chip = ctx.get_platform()?;
    let speed = chip.get_fan_speed(index)?;
    println!("fan {} speed: {} RPM", index, speed);
    Ok(())
}

fn process_status(args: &mut Arguments, ctx: &Context) -> Result<()> {
    if ctx.get_opts().help {
        println!(
            r"qute fan status [OPTIONS]

Get fan status

OPTIONS:
  -i, --index                   Fan Index. Optional.
  -h, --help                    Print this help text.
"
        );
        return Ok(());
    }
    let index: u8 = args
        .opt_value_from_str(["-i", "--index"])
        .map_err(|e| Error::with_chain(e, "invalid input for index"))?
        .unwrap_or(0);
    let config = ctx.get_config();
    if index >= config.fan.len() as u8 {
        return Err(format!("fan status: invalid fan index {}", index).into());
    }
    //get speed
    let chip = ctx.get_platform()?;
    let val = chip.get_fan_status(index)?;
    println!("fan {} status: {}", index, val);
    Ok(())
}
