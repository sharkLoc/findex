
## usage
```bash
fdx -- A file scanning tool for Linux system disks

Version: 0.1.0
Authors: sharkLoc <mmtinfo@163.com>
Source code: https://github.com/sharkLoc/findex.git

Usage:   fdx [OPTIONS] [path]

Arguments:
  [path]  The root directory path to be searched, default "."

Options:
  -t, --type <filetype>  Filter by type, file:f, directory:d, symlink:l
  -e, --ext <extension>  Filter by file extension, eg. gz, csv, txt ...
  -c, --ctime            If specified, show file created time in output
  -n, --name             If specified, show file name in output
  -d, --depth-firsh      If specified, perform depth-first search
  -l, --link             If specified, show sub-iterm in symbolink dir
  -H, --no-header        If specified, no header in output
  -o, --out <FILE>       Output file name or write to stdout
  -h, --help             Prints help information
  -V, --version          Prints version information
```