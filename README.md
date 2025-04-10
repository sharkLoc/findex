A file scanning tool for Linux system disks


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
  -a, --all                     If specified, show all iterm in output
  -d, --deepth <Number>         Set the maximum depth [default: 18446744073709551615]
  -t, --type                    If specified, show file type in output
  -s, --size                    If specified, show file size in output
  -c, --ctime                   If specified, show file created time in output
  -n, --name                    If specified, show file name in output
  -D, --depth-firsh             If specified, perform depth-first search
  -l, --link                    If specified, show sub-iterm in symbolink dir
  -e, --ext <Extension>         Filter by file extension, eg. gz, csv, txt, the file extension should not contain a dot
  -H, --no-header               If specified, no header in output
  -T, --filter-type <FileType>  Filter by type, file:f, directory:d, symlink:l. 
                                eg. only file in output: -T f, only directory in output: -T d
  -o, --out <File>              Output file name or write to stdout
  -h, --help                    Prints help information
  -V, --version                 Prints version information

```