use std::io::Error;

use clap::Parser;
use cli::Opt;
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
    eprintln!("Elapsed: {:?}", now.elapsed());
}

fn run_main() -> Result<(), Error> {
    let opt = Opt::parse();

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
