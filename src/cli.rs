use clap::{
    ArgAction, Parser,
    builder::{
        Styles,
        styling::{AnsiColor, Effects},
    },
};

// Configures Clap help menu colors
const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .usage(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .literal(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
    .placeholder(AnsiColor::Cyan.on_default());

#[derive(Parser, Debug)]
#[command(styles = STYLES)]
#[command(
    name = "fdx",
    author = env!("CARGO_PKG_AUTHORS"),
    version = env!("CARGO_PKG_VERSION"),
    about = "A file scanning tool for Linux system disks",
    long_about = "A powerful file scanning tool for Linux system disks, {n}supports filtering, formatting, and advanced search options.",
    disable_version_flag = true,
    disable_help_flag = true,
)]
#[command(help_template = "{name} -- {about}\n\nVersion: {version}\
\nAuthors: {author}\
\nSource code: https://github.com/sharkLoc/findex.git\
\n\n{usage-heading}   {usage}\n\n{all-args}\n")]
pub struct Opt {
    /// The root directory path to be searched, default "."
    #[arg(value_name = "path")]
    pub rootdir: Option<String>,

    /// If specified, show all iterm in output, including file type, size, created time, file name and path
    #[arg(short = 'a', long = "all")]
    pub show_all: bool,

    /// Set the maximum search depth. Defaults to unlimited depth (`usize::MAX`)
    #[arg(short = 'd', long = "deepth", default_value_t = usize::MAX, value_name = "Number")]
    pub deepth: usize,

    /// If specified, show file type in output
    #[arg(short = 't', long = "type")]
    pub show_type: bool,

    /// If specified, show file size in output  
    #[arg(short = 's', long = "size")]
    pub show_size: bool,

    /// Display file size in a human-readable format. Use with `-s`.
    /// {n}Supported units: `k` (KB), `m` (MB), `g` (GB). Defaults to `b` (bytes)
    #[arg(short = 'b', long = "byte", default_value_t = String::from("b"), value_name = "String")]
    pub size_fmt: String,

    /// If specified, show file created time in output
    #[arg(short = 'c', long = "ctime")]
    pub created_time: bool,

    /// If specified, show hidden files in output
    #[arg(short = 'i', long = "hidden")]
    pub show_hiden: bool,

    /// If specified, show full path in output
    #[arg(short = 'p', long = "full-path")]
    pub full_path: bool,

    /// If specified, show file name in output
    #[arg(short = 'n', long = "name")]
    pub name: bool,

    /// Perform a depth-first search instead of the default breadth-first search
    #[arg(short = 'D', long = "depth-first")]
    pub depth: bool,

    /// If specified, show sub-item in symbolink dir
    #[arg(short = 'l', long = "link")]
    pub show_link_dir: bool,

    /// Filter files by extension (e.g., `gz`, `csv`, `txt`). Do not include the dot (`.`) in the extension
    #[arg(short = 'e', long = "ext", value_name = "String")]
    pub ext: Option<String>,

    /// Apply a regular expression filter to file paths
    /// {n}The regex is matched against the full file path (not just the file name)
    /// {n}For example:
    /// {n}     - To match files ending with `.gz`: `-r "\.gz$"`
    /// {n}     - To match files containing "log" in their path: `-r "log"`
    /// {n}     Supports standard regex syntax. Use with `-I` to ignore case sensitivity
    #[arg(short = 'r', long = "regex", value_name = "Regex")]
    pub regex: Option<String>,

    /// Ignore case when filtering with the `-r` regex option
    #[arg(short = 'I', long = "ignore-case")]
    pub ignore_case: bool,

    /// Filter file size larger than the specified size (in bytes)
    /// {n}Examples:
    /// {n}     - Exclude files larger than 1 MB: `--max-size 1048576`
    /// {n}     - Exclude files larger than 500 KB: `--max-size 512000`
    #[arg(long = "max-size", value_name = "Number")]
    pub file_size_max: Option<u64>,

    /// Filter files smaller than the specified size (in bytes)
    /// {n}Examples:
    /// {n}     - Exclude files smaller than 1 KB: `--min-size 1024`
    /// {n}     - Exclude files smaller than 10 MB: `--min-size 10485760`
    #[arg(long = "min-size", value_name = "Number")]
    pub file_size_min: Option<u64>,

    /// Omit the header row in the output.
    #[arg(short = 'H', long = "no-header")]
    pub header: bool,

    /// Filter by file type: `f` (file), `d` (directory), `l` (symlink)
    /// {n}Examples:
    /// {n}     - Only files: `-T f`
    /// {n}     - Only directories: `-T d`
    #[arg(short = 'T', long = "filter-type", value_name = "String")]
    pub filetype: Option<String>,

    /// Write the output to a file instead of stdout
    #[arg(short = 'o', long = "out", value_name = "File")]
    pub out: Option<String>,

    /// Prints help information
    #[arg(short = 'h', long, action = ArgAction::Help)]
    pub help: Option<String>,

    /// Prints version information
    #[arg(short = 'V', long, action = ArgAction::Version)]
    pub version: Option<String>,
}
