A file scanning tool for Linux system disks

## usage

```bash
fdx -- A file scanning tool for Linux system disks

Version: 0.1.1
Authors: sharkLoc <mmtinfo@163.com>
Source code: https://github.com/sharkLoc/findex.git

Usage:   fdx [OPTIONS] [path]

Arguments:
  [path]  The root directory path to be searched, default "."

Options:
  -a, --all                   If specified, show all iterm in output, including file type, size, created time, file name and path
  -d, --deepth <Number>       Set the maximum depth [default: 18446744073709551615]
  -t, --type                  If specified, show file type in output
  -s, --size                  If specified, show file size in output
  -b, --byte <String>         Show file size in human-readable format, use with -s, k(Kb)/m(Mb)/g(Gb), default show b(bytes) [default: b]
  -c, --ctime                 If specified, show file created time in output
  -i, --hidden                If specified, show hidden files in output
  -p, --full-path             If specified, show full path in output
  -n, --name                  If specified, show file name in output
  -D, --depth-first           If specified, perform depth-first search
  -l, --link                  If specified, show sub-item in symbolink dir
  -e, --ext <String>          Filter by file extension, eg. gz, csv, txt, the file extension should not contain a dot
  -H, --no-header             If specified, no header in output
  -T, --filter-type <String>  Filter by type, file:f, directory:d, symlink:l. 
                              eg. only file in output: -T f, only directory in output: -T d
  -o, --out <File>            Output file name or write to stdout
  -h, --help                  Prints help information
  -V, --version               Prints version information

```

## TODO
* ansi style in tty output
* add logger