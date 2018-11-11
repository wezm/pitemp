// Display the ARM CPU and GPU temperature of Raspberry Pi 2/3
// Inspired by pitemp script by Vivek Gite <www.cyberciti.biz> under GPL v2.x+

extern crate trivial_colours;

use std::process::Command;
use std::{fs, io};
use trivial_colours::{Colour, Reset};

fn main() -> Result<(), io::Error> {
    let cpu_temp = read_cpu_temp()?;
    let gpu_temp = read_gpu_temp()?;

    output_temp("CPU", cpu_temp);
    output_temp("GPU", gpu_temp);

    Ok(())
}

fn read_cpu_temp() -> Result<f64, io::Error> {
    fs::read_to_string("/sys/class/thermal/thermal_zone0/temp")?
        .trim()
        .parse::<i32>()
        .map(|temp| temp as f64 / 1000.)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
}

fn read_gpu_temp() -> Result<f64, io::Error> {
    let output = Command::new("/opt/vc/bin/vcgencmd")
        .arg("measure_temp")
        .output()?;

    if output.status.success() {
        String::from_utf8(output.stdout)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
            .and_then(parse_vcgencmd_output)
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "vcgencmd did not run successfully",
        ))
    }
}

fn output_temp(label: &str, temp: f64) {
    println!("{}: {}{:.1}°C{}", label, colour(temp), temp, Reset);
}

fn colour(value: f64) -> Colour {
    if value > 80. {
        Colour::Red
    } else if value > 60. {
        Colour::Yellow
    } else {
        Colour::Green
    }
}

fn parse_vcgencmd_output(output: String) -> Result<f64, io::Error> {
    let parts: Vec<_> = output.splitn(2, '=').collect();

    match parts.as_slice() {
        &["temp", temp] => temp
            .chars()
            .filter(|c| c.is_digit(10) || *c == '.')
            .collect::<String>()
            .parse::<f64>()
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err)),
        _ => Err(io::Error::new(
            io::ErrorKind::Other,
            "unexpected output from vcgencmd",
        )),
    }
}

#[test]
fn test_parse_vcgencmd_output() {
    let result = parse_vcgencmd_output("temp=54.3'C".to_string()).unwrap();
    assert!(result > 54.29 && result < 54.31)
}

#[test]
fn test_parse_invalid_vcgencmd_output() {
    let result = parse_vcgencmd_output("error=1 error_msg=\"Command not registered\"".to_string());
    assert!(result.is_err(), "result is not error")
}
