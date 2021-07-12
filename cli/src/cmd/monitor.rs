use crate::ctx::Context as PlatformContext;
use anyhow::Result;
use chrono::prelude::*;
use pico_args::Arguments;
use qute_ctrl::{platform::Platform, FanControl, Temperature};
use std::{str::FromStr, thread::sleep, time::Duration};

pub fn run(args: &mut Arguments, ctx: &PlatformContext) -> Result<()> {
    if ctx.get_opts().help {
        print_help();
        return Ok(());
    }
    let min_temp: f32 = args.opt_value_from_str("--min")?.unwrap_or(5.0);
    let max_temp: f32 = args.opt_value_from_str("--max")?.unwrap_or(50.0);
    let method = args.opt_value_from_str("--method")?.unwrap_or_default();
    if min_temp >= max_temp {
        //todo
    }
    run_forever(ctx, method, min_temp, max_temp)?;
}

fn run_forever(ctx: &PlatformContext, method: Method, min_temp: f32, max_temp: f32) -> Result<!> {
    let chip = ctx.get_platform()?;
    let mut last_pwm = None;
    loop {
        let temp = get_max_temp(&chip)?;
        let temp = temp.min(max_temp).max(min_temp);
        trace!("max temperature: {} ℃", temp);
        let pwm = method.apply(min_temp, max_temp, temp);
        if Some(pwm) != last_pwm {
            last_pwm = Some(pwm);
            let index = 0;
            chip.set_fan_speed(index, pwm)?;
            let dt = Local::now();
            println!(
                "{}\t√ PWM of fan {} was set to {}",
                dt.format("%Y-%m-%d %H:%M:%S"),
                index,
                pwm
            );
        } else {
            let dt = Local::now();
            println!("{}\tPWM unchanged", dt.format("%Y-%m-%d %H:%M:%S"),);
        }

        sleep(Duration::from_secs(5));
    }
}

enum Method {
    Linear,
    Eager,
    Step,
}

impl Method {
    fn apply(&self, min_temp: f32, max_temp: f32, cur_temp: f32) -> u8 {
        let ratio = (cur_temp - min_temp) / (max_temp - min_temp);
        let ratio = match self {
            Method::Linear => ratio,
            Method::Eager => 1.0 - (1.0 - ratio).powi(2),
            Method::Step => {
                let ratio = (ratio * 10.0).floor() / 9.0;
                ratio.min(1.0)
            }
        };
        let pwm = (255.0 * ratio) as u8;
        pwm
    }
}

impl FromStr for Method {
    type Err = anyhow::Error;
    fn from_str(src: &str) -> std::result::Result<Self, Self::Err> {
        let src = src.to_lowercase();
        if src == "linear" {
            return Ok(Method::Linear);
        }
        if src == "eager" {
            return Ok(Method::Eager);
        }
        if src == "step" {
            return Ok(Method::Step);
        }
        Err(anyhow!("invalid method"))
    }
}

impl Default for Method {
    fn default() -> Self {
        Method::Linear
    }
}

fn get_max_temp(chip: &Platform) -> Result<f32> {
    let max = get_cpu_temp(chip)?;
    let max = disk::get_all_disk_temp()?
        .into_iter()
        .fold(max, |acc, (_, temp)| temp.max(acc));
    Ok(max)
}

fn get_cpu_temp(chip: &Platform) -> Result<f32> {
    let temp = chip.get_temperature(0)?;
    Ok(temp)
}

fn print_help() {
    println!(
        r"qute monitor [OPTIONS]

automatically adjust fan speed based on cpu and hdd temperatures.

Note: requires smartctl to be installed

OPTIONS:
  -h, --help                   Print this help text.
  --min                         Minimal temperature, default 5 ℃
  --max                        Maximum temperature, default 50 ℃
  --method                  Available options: Linear | Eager | Step
                                    - Linear: linearly adjust pwm based on temperatures
                                    - Eager:  higher temperatures, faster fan speed
                                    - Step: level based
"
    );
}

mod disk {
    use anyhow::{Context, Result};
    use std::{
        path::{Path, PathBuf},
        process::Command,
    };

    pub fn get_all_disk_temp() -> Result<Vec<(PathBuf, f32)>> {
        let devices = list_disk()?;
        let mut res = vec![];
        for dev in devices {
            let temp = get_disk_temp(&dev)?;
            res.push((dev, temp));
        }
        Ok(res)
    }

    fn get_disk_temp<P: AsRef<Path>>(path: P) -> Result<f32> {
        let output = Command::new("smartctl")
            .arg("-A")
            .arg(path.as_ref().display().to_string())
            .arg("-j")
            .output()
            .with_context(|| "require smartctl to be installed")?;
        let text = String::from_utf8(output.stdout).unwrap();
        let obj = json::parse(&text)?;
        let value: f32 = obj["temperature"]["current"].as_f32().unwrap();
        Ok(value)
    }

    fn list_disk() -> Result<Vec<PathBuf>> {
        let output = Command::new("ls").arg("/sys/block").output()?;
        let text = String::from_utf8(output.stdout).unwrap();

        let devices = text
            .split('\n')
            .filter_map(|dev| {
                let link = PathBuf::from("/sys/block").join(dev);
                link.read_link().ok().and_then(|path| {
                    let path_str = path.to_str().unwrap();
                    if is_disk(path_str) {
                        Some(PathBuf::from("/dev").join(dev))
                    } else {
                        None
                    }
                })
            })
            .collect();
        Ok(devices)
    }

    fn is_disk(line: &str) -> bool {
        if !line.contains("devices/pci") {
            return false;
        }
        if line.contains("usb") {
            return false;
        }
        true
    }
}
