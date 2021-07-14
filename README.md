# PathCopy
A cli command which copies the absolute Path of the given file or directory to the clipboard.

## Installation

### Homebrew

```sh
$ brew tap oltoko/brew
$ brew install pathcopy
```

### Debian based

```sh
$ wget https://github.com/oltoko/PathCopy/releases/download/0.1.1/pc_0.1.1_amd64.deb
$ sudo dpkg -i pc_0.1.1_amd64.deb
```

### Other

Use the binaries for the latest [Release](https://github.com/oltoko/PathCopy/releases).

## Usage

```sh
pc [OPTIONS] <path>...

FLAGS:
    -h, --help       Prints help information
    -v, --version    Prints version information

OPTIONS:
    -s, --separator <separator>    The separator which should be used when multiple paths are put into the clipboard.
                                   You can choose between 3 separators:
                                   	%w - A whitespace character
                                   	%n - The system specific line break
                                   	%t - A tab character [default: %w]  [possible values: %w, %n, %t]

ARGS:
    <path>...    The file(s) or directory(s) from which the absolute path(s) should be copied.
```
