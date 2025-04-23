A file scanning tool for Linux system disks

## Binary download
you can download pre-build static binary file from [here](https://github.com/sharkLoc/findex/releases/download/0.1.2/fdx_static)

## usage

```bash
fdx -- A file scanning tool for Linux system disks

Version: 0.1.
Authors: sharkLoc <mmtinfo@163.com>
Source code: https://github.com/sharkLoc/findex.git

Usage:   fdx [OPTIONS] [path]

Arguments:
  [path]  The root directory path to be searched, default "."

Options:
  -a, --all                   If specified, show all iterm in output, including file type, size, created time, file name and path
  -d, --deepth <Number>       Set the maximum search depth. Defaults to unlimited depth (`usize::MAX`) [default: 18446744073709551615]
  -t, --type                  If specified, show file type in output
  -s, --size                  If specified, show file size in output
  -b, --byte <String>         Display file size in a human-readable format. Use with `-s`. 
                              Supported units: `k` (KB), `m` (MB), `g` (GB). Defaults to `b` (bytes) [default: b]
  -c, --ctime                 If specified, show file created time in output
  -i, --hidden                If specified, show hidden files in output
  -p, --full-path             If specified, show full path in output
  -n, --name                  If specified, show file name in output
  -D, --depth-first           Perform a depth-first search instead of the default breadth-first search
  -l, --link                  If specified, show sub-item in symbolink dir
  -e, --ext <String>          Filter files by extension (e.g., `gz`, `csv`, `txt`). Do not include the dot (`.`) in the extension
  -r, --regex <Regex>         Apply a regular expression filter to file paths 
                              The regex is matched against the full file path (not just the file name) 
                              For example: 
                                   - To match files ending with `.gz`: `-r "\.gz$"` 
                                   - To match files containing "log" in their path: `-r "log"` 
                                   Supports standard regex syntax. Use with `-I` to ignore case sensitivity
  -I, --ignore-case           Ignore case when filtering with the `-r` regex option
      --max-size <Number>     Filter file size larger than the specified size (in bytes) 
                              Examples: 
                                   - Exclude files larger than 1 MB: `--max-size 1048576` 
                                   - Exclude files larger than 500 KB: `--max-size 512000`
      --min-size <Number>     Filter files smaller than the specified size (in bytes) 
                              Examples: 
                                   - Exclude files smaller than 1 KB: `--min-size 1024` 
                                   - Exclude files smaller than 10 MB: `--min-size 10485760`
  -H, --no-header             Omit the header row in the output
  -T, --filter-type <String>  Filter by file type: `f` (file), `d` (directory), `l` (symlink) 
                              Examples: 
                                   - Only files: `-T f` 
                                   - Only directories: `-T d`
  -o, --out <File>            Write the output to a file instead of stdout
  -h, --help                  Prints help information
  -V, --version               Prints version information



```

## example:

![img](https://github.com/sharkLoc/findex/blob/main/misc/eg.PNG)

## TODO

- [X] ansi style in tty output
- [X] support regex filter
- [X] add logger
