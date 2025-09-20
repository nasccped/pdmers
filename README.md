<div align="center">

# pdmers

[![crates io](https://img.shields.io/crates/v/pdmers.svg)](https://crates.io/crates/pdmers)
[![license: MIT](https://img.shields.io/badge/license-MIT-blue?)](#)

</div>

Download **pdmers** and merge it all in a fancy way 🎉🎉🎉

## Requirements

You'll need the rust toolkit to install this program:
- rust compiler (`rustc`)
- rust package manager (`cargo`)

> [!TIP]
>
> You can find both from the rust's
> [official webpage](https://www.rust-lang.org/).

## Download

You can download this program both from source (`github` method) or
using `cargo` packages registries:

### With `github`:

- clone into your machine

```sh
git clone https://github.com/nasccped/pdmers
cd pdmers
```

- build and install

```sh
cargo install --path .
```

### With `cargo`: _(easier)_

- just use `install` subcommand

```sh
cargo install pdmers
```

## Usage

To see the help panel, use the `pdmers --help` command:

```txt
A rust tool for PDF merging

Usage: pdmers.exe [OPTIONS]

Options:
  -i, --input <FILES|DIRS>...          PDF files to be merged
  -o, --output <OUTPUT>                Where to place the output file
      --override                       Override output file if it already exists
      --allow-repetition               Pass same input twice
  -d, --depth <N>                      Merge PDFs until reaches the `N` directory layer (use `*` to infinity)
  -p, --parent                         Creates parent directories of the output file (if they don't exists)
      --order-by <ALPHA|DATETIME|DEF>  Order files alphabetically, datetime or default (input order)
  -h, --help                           Print help
  -V, --version                        Print version
```

`pdmers` enforces you to use an **option**, otherwise it'll print a
message at `stderr` and return an error exit code (`1`).

```txt
$ pdmers
error: no arguments provided

Try using `pdmers --help` to get usage tips!
```

### Usage

This app gets PDF files as input and generates a merged PDF file as
output:

```txt
          +----------+
inputs => | do stuff | => output
          +----------+
               |
        +--------------+
        | catch errors |
        +--------------+
```

#### Input

The `input` flag expects a list of file and/or directory paths:

```txt
# You can use the flag's long name
$ pdmers --input file.pdf files-dir

# or the alias too
$ pdmers -i file.pdf files-dir
```

> [!WARNING]
>
> The program's assumes that all paths passed as input exists,
> otherwise it'll raise an `not-readable-entry` or
> `non-existing-file` error.

##### Composed paths

The program can't read paths that contains white-spaces (` `) because
it separates input values by the space delimiter:

```txt
# so this:
$ pdmers -i file1.pdf file2.pdf

# becomes this:
# > input = ["file1.pdf", "file.pdf"]

# and this:
$ pdmers -i 'composed name file.pdf'

# becomes this:
# > input = ["composed", "name", "file.pdf"]
```

In other words, this program can't deal with _compose-named_ paths
and returns `error: input doesn't exists`.

##### Path repetition

By default, the program doesn't allow path repetition:

```txt
# this command will fail
$ pdmers -i file.pdf file.pdf
```

This approach avoids content duplicates. A clearer example:

```txt
# curdir/
# └─ other-dir/
#    └ inner-file.pdf
#
# accidental duplicate :^(
$ pdmers -i other-dir/inner-file.pdf other-dir
```

If you're sure about what you're doing, use the `allow-repetition`
flag:

```txt
# this will work fine
$ pdmers -i file.pdf file.pdf
```

##### Passing directories

When passing directory(ies) as input, the program need to know how
deep to catch files. You should use `depth` flag for this

```txt
# curdir/ ---------------> depth 0
# ├ layer-one/ ----------> depth 1
# │ ├ layer-two/ --------> depth 2
# │ │ └ layer-two-file.pdf
# │ └ layer-two-file.pdf
# └ layer-zero.pdf
$ pdmers -i layer-zero.pdf layer-one --depth 1
# the program will only catches the file path + all layer-one inner
# files, since it's depth 1 (passed through the depth flag)
```

> [!TIP]
>
> The `depth` flag can be used both with long name (`--depth`) and
> its alias (`-d`). It must be greater than `0` (since `0` means
> curdir). Use `*` to represent _"no-depth"_ (all files ahead).

##### Directory references

The program doesn't allow directory references as input, like:
`../outside-file.pdf` or `up-dir/../curdir-file.pdf`. This was
thought to avoid path/privileges exploiting.

> [!NOTE]
>
> Current directory references (`.`) still works, btw!

##### Input ordering

The merge order will follows the `input` flag's values. If any of
values is a directory, it'll follows the `PathBuf::read_dir`
iterator order (commonly alphabetical order):

```txt
# curdir/
# ├ inner/
# │ ├ a.pdf
# │ ├ b.pdf
# │ ├ c.pdf
# │ └ ...
# └ curdir.pdf

# this:
$ pdmers -i curdir.pdf inner
# becomes this:
# input = ["curdir.pdf", "inner/a.pdf", "inner/b.pdf", "inner/c.pdf", ...]
```

> [!IMPORTANT]
>
> I planned to insert sorting methods but I gave up since rust's
> metadata (created + updated) reading isn't well supported on all
> platforms.

#### Output

The `output` flag means where to place the merged file. It must
always be an `pdf` extension file:

```txt
# this will works:
$ pdmers -i some.pdf file.pdf --output output.pdf

# this won't
$ pdmers -i some.pdf file.pdf --output output.txt
```

> [!TIP]
>
> You can use the flag alias (`-o`) too!

##### Override

If the output file path already exists, you can force save by using
the `override` flag:

```txt
# curdir/ 
# ├ file1.pdf
# ├ file2.pdf
# └ output.pdf
# not allowed, output already exists
$ pdmers -i file1.pdf file2.pdf -o output.pdf

# warning: this is dangerous but allowed
$ pdmers -i file1.pdf file2.pdf -o output.pdf --override
```

##### Parent dir

Placing the output file within a directory is allowed but it'll fail
if the parents dirs doesn't exists. To force parent dirs + file
creation, you can use the `parent` flag:

```txt
# tree before:
# curdir/ 
# ├ file1.pdf
# └ file2.pdf

$ pdmers -i file1.pdf file2.pdf -o output/file.pdf --parent

# tree after:
# curdir/ 
# ├ output/
# │ └ file.pdf
# ├ file1.pdf
# └ file2.pdf
```

You can also use the flag alias (`-p`)!

## License

This project is under the [MIT](./LICENSE) license!
