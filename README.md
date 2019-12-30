# validjson
Simple JSON schema validator based on valico and serde_json that can be run from console.

## Install
```
cargo install validjson
```

## Usage
```
USAGE:
    validjson [FLAGS] <SCHEMA> [CONTENT]...

FLAGS:
    -h, --help       Prints help information
    -i               Ignore unknown keywords in schema validation
    -V, --version    Prints version information

ARGS:
    <SCHEMA>        JSON Schema file
    <CONTENT>...    JSON content file[s]
```
