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

    /// If specified, show all iterm in output, including file type, size, created time, file name and path.
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

    /// Show file size in human-readable format, use with -s, k(Kb)/m(Mb)/g(Gb), default show b(bytes)  
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

    /// If specified, perform depth-first search
    #[arg(short = 'D', long = "depth-first")]
    pub depth: bool,

    /// If specified, show sub-item in symbolink dir
    #[arg(short = 'l', long = "link")]
    pub show_link_dir: bool,

    /// Filter by file extension, eg. gz, csv, txt, the file extension should not contain a dot.  
    #[arg(short = 'e', long = "ext", value_name = "String")]
    pub ext: Option<String>,

    /// If specified, no header in output
    #[arg(short = 'H', long = "no-header")]
    pub header: bool,

    /// Filter by type, file:f, directory:d, symlink:l.
    /// {n}eg. only file in output: -T f, only directory in output: -T d
    #[arg(short = 'T', long = "filter-type", value_name = "String")]
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
