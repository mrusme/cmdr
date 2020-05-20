cmdr
----

A minimalistic, zero-configuration, single-binary tool built in Rust that runs multiple commands simultaneously and can be used as Docker `ENTRYPOINT`.

## Installation

### Manual

```bash
git clone https://github.com/mrusme/cmdr.git
cd cmdr
cargo install --path .
```

## Usage

```
USAGE:
    cmdr --cmd <COMMAND>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --cmd <COMMAND>...    command to run
```

### Examples

```bash
cmdr --cmd "/bin/sleep 10" --cmd "/bin/ls"
```

```bash
cmdr --cmd "/usr/bin/caddy" --cmd "/usr/bin/mongod"
```
