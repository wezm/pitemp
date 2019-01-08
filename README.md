# pitemp

Small tool to display CPU and GPU temperature on Raspberry Pi

[![builds.sr.ht status](https://builds.sr.ht/~wezm/pitemp.svg)](https://builds.sr.ht/~wezm/pitemp?)
[Issue Tracker](https://todo.sr.ht/~wezm/pitemp)

<img src="https://git.sr.ht/%7Ewezm/pitemp/blob/master/screenshot.png" alt="Screenshot of pitemp in a terminal window" width="466" />

## Compatibility

`pitemp` has been tested on these devices:

* Raspberry Pi 3 B
* Raspberry Pi Zero W

## Installing

### From Binary Release

[Latest Release][release]

`pitemp` is a single small binary. To download the latest release do the following:

    curl -L https://releases.wezm.net/pitemp/pitemp-v0.3.0-arm-unknown-linux-gnueabihf.tar.gz | tar zxf -

The binary should be in your current directory and can be run as follows:

    ./pitemp

Feel free to move it elsewhere (`~/.local/bin` for example).

### From Source

**Note:** You will need the [Rust compiler installed][rust].

    git clone https://git.sr.ht/~wezm/pitemp
    cargo install --path pitemp

## License and Credits

`pitemp` is licensed under the [GPL 2 license][license] and is inspired by
[pitemp script by Vivek Gite][nixcraft] released under GPL v2.x+.

[rust]: https://www.rust-lang.org/en-US/install.html
[nixcraft]: https://www.cyberciti.biz/faq/linux-find-out-raspberry-pi-gpu-and-arm-cpu-temperature-command/
[release]: https://releases.wezm.net/pitemp/
[license]: https://git.sr.ht/%7Ewezm/pitemp/tree/master/LICENSE
