# x11_edid_auto

Automatically set monitors for X server with monitor EDIDs.  
Sometimes my monitors switch DP1 and DP2 names when returning from sleep.  
This program is developed to fix that specific problem, but feel free to fork and modify it to fix also yours.

## Description

This Rust project automatically configures monitors for the X server based on monitor EDIDs in given .toml file.

## Installation

To build and install this project, you need to have Rust installed.  
You can then use `cargo` to build the project:

```sh
cargo install --path .
```
## Usage
To use this program, you need to provide a configuration file as an argument:
```sh
x11_edid_auto config.toml
```
The configuration file should be in TOML format and contain a list of monitor groups.  
Example can be found in [example.toml](example.toml).  
EIDS in this file are of format:  
`<MANUFACTURER>:<PRODUCT>:<SERIAL>`
Program will use first group of EIDS that has all of the monitors connected and disables rest of the connected monitors.  
If there is multiple monitors in one group, program will set first as leftmost and primary, then just puts others right of the previous one.  
`[MONITOR1][MONITOR2][MONITOR3]`

### Examples

#### Working
Working example, with only EDIDs changed.
```sh
x11_edid_auto config.toml 
```
=>
```sh
Getting monitors...
Monitor groups...
2. monitor group had all of it's monitors ["AAAA:AAAA:AAAAAAAA", "BBBB:BBBB:BBBBBBBB"] present!
Monitor:
         edid: "AAAA:AAAA:AAAAAAAA"
         name: "DP1"
         crtc: 62
crtc_info:
         x: 0
         y: 0
         width: 2560
         height: 1440
         mode: 1089
         rotation: ROTATE0
CRTC config is already set!
Monitor:
         edid: "BBBB:BBBB:BBBBBBBB"
         name: "DP2"
         crtc: 63
crtc_info:
         x: 2560
         y: 0
         width: 2560
         height: 1440
         mode: 1089
         rotation: ROTATE0
CRTC config is already set!
Disabling unused monitors...
Monitor:
         edid: "CCCC:CCCC:CCCCCCCC"
         name: "eDP1"
         crtc: 0
Monitor is already disabled!
Done!
```

### Listing monitors.
Just run for config file that does not contain any of your monitors.  
[example.toml](example.toml) is great for this purpose.  
```sh
x11_edid_auto example.toml
```
=>
```sh
Getting monitors...
Monitor groups...
2. monitor group did not have all of it's monitors ["XXXX:XXXX:XXXXXXXX", "YYYY:YYYY:YYYYYYYY"] present!
3. monitor group did not have all of it's monitors ["XXXX:XXXX:XXXXXXXX"] present!
No monitor group with all of it's monitors present found!
Available monitors:
Monitor:
         edid: "AAAA:AAAA:AAAAAAAA"
         name: "DP1"
         crtc: 62
Monitor:
         edid: "BBBB:BBBB:BBBBBBBB"
         name: "DP2"
         crtc: 63
Monitor:
         edid: "CCCC:CCCC:CCCCCCCC"
         name: "eDP1"
         crtc: 0
```

### My own usage
I'm using [i3](https://i3wm.org/) window manager.  
I have following at the end of my `~/.config/i3/config`-file.  
```
exec_always --no-startup-id /usr/local/bin/x11_edid_auto /etc/x11_edid_auto.toml
```
So when [i3](https://i3wm.org/) first starts or when I reload [i3](https://i3wm.org/) it will run this script.  
I update my `/usr/local/bin/x11_edid_auto` and `/etc/x11_edid_auto.toml` files by running [make](https://www.gnu.org/software/make/).  
```sh
make install
```

## License
This project is licensed under the Apache-2.0 License. See the [LICENSE.txt](LISENSE.txt) file for details.
