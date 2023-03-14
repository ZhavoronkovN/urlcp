# urlcp
## Copying files from url

## Usage:
```
urlcp [OPTIONS] --url <URL> --output <OUTPUT>
```

## Options:
```
  -u, --url <URL>        public url pointing to file
  -o, --output <OUTPUT>  output path with file name and extension
  -r, --rewrite          rewrite file if exists
  -h, --help             Print help
  -V, --version          Print version
```

## Development

Refer to the officail Rust website to install Rust and Cargo - https://www.rust-lang.org/learn/get-started

Example command to install Rust and Cargo - `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

To build the app, run `rustup target add x86_64-pc-windows-gnu`
