# waymouse
`waymouse` is a command-line program for simulating mouse inputs on wayland.

## Prerequisites
1. For waymouse to work you need to be using a wayland compositor/desktop that implements `zwlr_virtual_pointer_v1`, you can check if your compositor/desktop implements that [here](https://wayland.app/protocols/wlr-virtual-pointer-unstable-v1#compositor-support)
2. You need to have cargo to install waymouse as there are currently no pre-built binaries, cargo can be installed with `curl https://sh.rustup.rs -sSf | sh`
3. `~/.cargo/bin` needs to be in your `$PATH`, this should be done by default if you ran the previous command and installed cargo through rustup. If you installed cargo through your package manager you may need to add `export PATH="$HOME/.cargo/bin:$PATH"` to your .bashrc/.zshrc

## Installation
```
cargo install --git https://github.com/xunuwu/waymouse
```
This command might take a while to run as it needs to compile `waymouse` from source.

## Usage
You can always use `waymouse --help` or `waymouse <SUBCOMMAND> --help` :)
```
# To input negative numbers you need to use --, ex 'waymouse move -- -20 -150'
waymouse move <X> <Y> # These are relative to your current mouse position!
waymouse scroll [--horizontal] <AMOUNT>
waymouse button click [-c <COUNT>] [-d <DELAY_MS>] <left|right|middle>
waymouse button up <left|right|middle>
waymouse button down <left|right|middle>
```
