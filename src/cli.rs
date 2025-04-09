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
    
    /// Filter by type, file:f, directory:d, symlink:l
    #[arg(short = 't', long = "type", value_name = "filetype" )]
    pub filetype: Option<String>,

    /// Filter by file extension, eg. gz, csv, txt ...
    #[arg(short = 'e', long = "ext", value_name = "extension")]
    pub ext: Option<String>,
    
    /// If specified, show file created time in output
    #[arg(short = 'c', long = "ctime")]
    pub created_time: bool,

    /// If specified, show file name in output
    #[arg(short = 'n', long = "name",)]
    pub name: bool,

    /// If specified, perform depth-first search
    #[arg(short = 'd', long = "depth-firsh")]
    pub depth: bool,

    /// If specified, show sub-iterm in symbolink dir
    #[arg(short = 'l', long = "link")]
    pub show_link_dir: bool,

    /// If specified, no header in output
    #[arg(short = 'H', long = "no-header")]
    pub header: bool,

    /// Output file name or write to stdout
    #[arg(short = 'o', long = "out", value_name = "FILE")]
    pub out: Option<String>,

    /// Prints help information
    #[arg(short = 'h', long, action = ArgAction::Help)]
    pub help: Option<String>,

    /// Prints version information
    #[arg(short = 'V', long, action = ArgAction::Version)]
    pub version: Option<String>,
}
