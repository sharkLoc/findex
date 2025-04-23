use std::io::Error;

use clap::Parser;
use cli::Opt;
use log::info;
use process::search_dir;

mod cli;
mod process;

fn main() {
    let now = std::time::Instant::now();
    match run_main() {
        Ok(_) => {}
        Err(_) => {
            std::process::exit(1);
        }
    }
    info!("Elapsed: {:?}", now.elapsed());
}

fn log_level(verbose: u8) -> log::LevelFilter {
    match verbose {
        1 => log::LevelFilter::Error,
        2 => log::LevelFilter::Warn,
        3 => log::LevelFilter::Info,
        4 => log::LevelFilter::Debug,
        5 => log::LevelFilter::Trace,
        _ => log::LevelFilter::Off,
    }
}

fn run_main() -> Result<(), Error> {
    let opt = Opt::parse();

    // Set up logging
    let mut logger = env_logger::Builder::new();
    logger
        .filter(None, log_level(opt.verbose))
        .format_timestamp(Some(env_logger::fmt::TimestampPrecision::Seconds))
        .format_module_path(true)
        .format_target(true)
        .format_indent(Some(2))
        .format_level(true)
        .init();

    // if opt.rootdir is None, use default value "."
    let dir = opt.rootdir.unwrap_or_else(|| ".".to_string());

    search_dir(
        dir,
        opt.regex.as_deref(),
        opt.ignore_case,
        opt.show_all,
        opt.deepth,
        opt.show_type,
        opt.show_size,
        opt.file_size_max,
        opt.file_size_min,
        &opt.size_fmt,
        opt.created_time,
        opt.filetype.as_ref(),
        opt.ext.as_deref(),
        opt.name,
        opt.depth,
        opt.full_path,
        opt.show_link_dir,
        opt.show_hiden,
        opt.header,
        opt.out.as_ref(),
    )?;

    Ok(())
}
