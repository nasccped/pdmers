<div align="center">

[![pdmers logo](./images/crate-logo.svg)](#)

[![crates io](https://img.shields.io/crates/v/pdmers.svg)](https://crates.io/crates/pdmers)
[![license: MIT](https://img.shields.io/badge/license-MIT-blue?)](#)

</div>

Download **pdmers** and merge it all in a fancy way 🎉🎉🎉

> [!IMPORTANT]
>
> This is a short overview. Get more info at
> [official repository](https://github.com/nasccped/pdmers).

## Download

Use the following command to download from cargo packages registry:

```sh
cargo install pdmers
```

## Usage

You should use the `--input` flag (alias: `-i`) to pass pdf/directory
paths and `--output` flag (alias: `-o`) to specify where to
place the final file:

```txt
  +--------+                                               output path
  | pdmers |           +---------------------+             -----------
  +--------+  --input  | file1.pdf file2.pdf |                  |
       |               +---------------------+  --output  +------------+
       |                          |                       | output.pdf |
  ------------                 ------                     +------------+
  command name                 inputs
```

There's also other _permission flags/options_. Consider checking out
the [official repository](https://github.com/nasccped/pdmers).

## License

This project is under the
[MIT](https://github.com/nasccped/pdmers/blob/main/LICENSE) license!
