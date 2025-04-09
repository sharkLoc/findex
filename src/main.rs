use std::io::Error;

use clap::Parser;
use cli::Opt;
use process::search_dir;

mod cli;
mod process;



fn main() -> Result<(),Error>{
    let opt = Opt::parse();

    if let Some(dir) = opt.rootdir {
        match opt.ext {
            Some(ext) => search_dir(dir,opt.created_time,opt.filetype.as_ref(),Some(ext.as_str()),opt.name, opt.depth, opt.show_link_dir,opt.header,opt.out.as_ref())?,
            None => search_dir(dir,opt.created_time,opt.filetype.as_ref(),None,opt.name, opt.depth, opt.show_link_dir,opt.header,opt.out.as_ref())?
        }
    } else {
        match opt.ext {
            Some(ext) => search_dir(".",opt.created_time,opt.filetype.as_ref(),Some(ext.as_str()),opt.name, opt.depth, opt.show_link_dir,opt.header,opt.out.as_ref())?,
            None => search_dir(".",opt.created_time,opt.filetype.as_ref(),None,opt.name, opt.depth, opt.show_link_dir,opt.header,opt.out.as_ref())?
        }
    };

    // match opt.rootdir {
    //     Some(dir) => search_dir(dir,Some(opt.ext.unwrap().as_str()),opt.name, opt.depth, opt.show_link_dir,opt.header,opt.out.as_ref()),
    //     None => search_dir(".", Some(opt.ext.unwrap().as_str()), opt.name, opt.depth, opt.show_link_dir,opt.header,opt.out.as_ref())
    // }.unwrap();
    
    Ok(())
}
