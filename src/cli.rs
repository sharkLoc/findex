use std::usize;

use clap::{ArgAction, Parser};

pub const VERSION: &str = "0.1.0";

#[derive(Parser, Debug)]
#[command(
    name = "fdx",
    author = "sharkLoc",
    version = VERSION,
    about = "A file scanning tool for Linux system disks",
    disable_version_flag = true,
    disable_help_flag = true,
)]
#[command(help_template = "{name} -- {about}\n\nVersion: {version}\
\nAuthors: {author} <mmtinfo@163.com>\
\nSource code: https://github.com/sharkLoc/findex.git\
\n\n{usage-heading}   {usage}\n\n{all-args}\n")]
pub struct Opt {
    /// The root directory path to be searched, default "."
    #[arg(value_name = "path")]
    pub rootdir: Option<String>,

    /// If specified, show all iterm in output
    #[arg(short = 'a', long = "all")]
    pub show_all: bool,

    /// Set the maximum depth
    #[arg(short = 'd', long = "deepth", default_value_t = usize::MAX, value_name = "Number")]
    pub deepth: usize,

    /// If specified, show file type in output
    #[arg(short = 't', long = "type")]
    pub show_type: bool,

    /// If specified, show file size in output
    #[arg(short = 's', long = "size")]
    pub show_size: bool,

    /// If specified, show file created time in output
    #[arg(short = 'c', long = "ctime")]
    pub created_time: bool,

    /// If specified, show file name in output
    #[arg(short = 'n', long = "name")]
    pub name: bool,

    /// If specified, perform depth-first search
    #[arg(short = 'D', long = "depth-firsh")]
    pub depth: bool,

    /// If specified, show sub-iterm in symbolink dir
    #[arg(short = 'l', long = "link")]
    pub show_link_dir: bool,

    /// Filter by file extension, eg. gz, csv, txt, the file extension should not contain a dot.  
    #[arg(short = 'e', long = "ext", value_name = "Extension")]
    pub ext: Option<String>,

    /// If specified, no header in output
    #[arg(short = 'H', long = "no-header")]
    pub header: bool,

    /// Filter by type, file:f, directory:d, symlink:l.
    /// {n}eg. only file in output: -T f, only directory in output: -T d
    #[arg(short = 'T', long = "filter-type", value_name = "FileType")]
    pub filetype: Option<String>,

    /// Output file name or write to stdout
    #[arg(short = 'o', long = "out", value_name = "File")]
    pub out: Option<String>,

    /// Prints help information
    #[arg(short = 'h', long, action = ArgAction::Help)]
    pub help: Option<String>,

    /// Prints version information
    #[arg(short = 'V', long, action = ArgAction::Version)]
    pub version: Option<String>,
}
