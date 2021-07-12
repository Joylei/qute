#![feature(never_type)]

//#![deny(missing_docs, warnings)]
#[macro_use]
extern crate log;
extern crate qute_ctrl;
extern crate stderrlog;
#[macro_use]
extern crate anyhow;

pub(crate) mod cmd;
pub(crate) mod config;
pub(crate) mod ctx;
pub(crate) mod utils;

use crate::config::{Config, FanConfig};
use crate::ctx::{Context as PlatformContext, Options};
use anyhow::{Context, Result};
use pico_args::Arguments;
use std::alloc::System;

#[global_allocator]
static A: System = System;

pub(crate) const APP_NAME: &str = "qute";
pub(crate) const APP_VERSION: &str = "0.1";

fn main() -> Result<()> {
    let mut args = Arguments::from_env();
    let opts = Options {
        quiet: args.contains(["-q", "--quiet"]),
        verbose: args
            .opt_value_from_str(["-v", "--verbose"])
            .with_context(|| "invalid value for verbose")?
            .unwrap_or(0),
        help: args.contains(["-h", "--help"]),
    };
    stderrlog::new()
        .module(module_path!())
        .module("qute_hal")
        .quiet(opts.quiet)
        .verbosity(opts.verbose)
        //.timestamp(opt.ts.unwrap_or(stderrlog::Timestamp::Off))
        .init()
        .unwrap();

    let config = Config {
        fan: vec![FanConfig {
            max_speed: 1700,
            min_speed: 0,
        }],
    };
    let ctx = PlatformContext::new(config, opts);
    run(&mut args, &ctx)
}

fn run(args: &mut Arguments, ctx: &PlatformContext) -> Result<()> {
    //check version
    if args.contains(["-V", "--version"]) {
        println!("version: {}", APP_VERSION);
        return Ok(());
    }
    //check sub command
    let text = args.subcommand().ok().flatten().unwrap_or_default();
    match text.as_str() {
        "eup" => return cmd::eup::run(args, ctx),
        "fan" => return cmd::fan::run(args, ctx),
        "power" => return cmd::power::run(args, ctx),
        "temp" => return cmd::temp::run(args, ctx),
        "monitor" => return cmd::monitor::run(args, ctx),
        _ => {}
    }
    // no sub command
    println!(
        r"{} v{}

QNAP device control. Use AT YOUR OWN RISK!!!

USAGE: {} [OPTIONS] [COMMANDS]

OPTIONS:
  -V, --version                 Show version number
  -h, --help                      Show help message
  -v, --verbose [level:N]   Show verbose messages
  -q, --quiet                     Silence all output

COMMANDS:
  eup                                get or set Eup mode
  fan                                 get or set fan speed
  power                            get or set power recovery mode
  temp                              get temperature
  monitor                         auto adjust fan speed based on temperatures
",
        APP_NAME, APP_VERSION, APP_NAME
    );
    Ok(())
}
