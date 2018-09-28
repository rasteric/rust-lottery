# :moneybag: lottery - random lottery suggestions

This command-line tool draws lottery numbers in a given range. It is written in Rust.

# Usage

`lottery [OPTIONS] <drawings> <upper_value>`

where `drawings` is the number of drawings and `upper_value` is the highest number (inclusive). By default numbers are drawn from 1 to `upper_value`.

OPTIONS:

    -l, --lower_value <lower_value>    the lowest number to draw from (inclusive), default=1


    -h, --help       Prints help information
    -V, --version    Prints version information

# Installation

Currently there are no build scripts but you may `cargo build` to build an executable for your system.
