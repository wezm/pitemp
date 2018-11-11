# pitemp

Small tool to display CPU and GPU temperature on Raspberry Pi

[![Build Status](https://api.cirrus-ci.com/github/wezm/pitemp.svg)](https://cirrus-ci.com/github/wezm/pitemp)

<img src="https://raw.githubusercontent.com/wezm/pitemp/master/screenshot.png" alt="Screenshot of pitemp in a terminal window" width="466" />

## Installing

### From Binary Release

`pitemp` is a single small binary. Downloads are available from the
[GitHub Releases][releases]. To download the latest release do the following:

    curl -L https://github.com/wezm/pitemp/releases/download/v0.2.1/pitemp-v0.2.1-arm-linux-gnueabihf.tar.gz | tar zxf -

The binary should be in your current directory and can my run as follows:

    ./pitemp

Feel free to move it elsewhere (`~/.local/bin` for example).

### From Source

**Note:** You will need the [Rust compiler installed][rust].

    git clone https://github.com/wezm/pitemp.git
    cargo install --path pitemp

## License and Credits

`pitemp` is licensed under the [GPL 2 license][license] and is inspired by
[pitemp script by Vivek Gite][nixcraft] released under GPL v2.x+.

[rust]: https://www.rust-lang.org/en-US/install.html
[nixcraft]: https://www.cyberciti.biz/faq/linux-find-out-raspberry-pi-gpu-and-arm-cpu-temperature-command/
[releases]: https://github.com/wezm/pitemp/releases/latest
[license]: https://github.com/wezm/pitemp/blob/master/LICENSE
