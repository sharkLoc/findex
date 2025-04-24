use log::info;
use regex::RegexBuilder;
use std::{
    env,
    fs::File,
    io::{self, BufWriter, Error, Write},
    path::{Path, PathBuf},
    time::SystemTime,
};
use tabwriter::TabWriter;
use walkdir::{self, DirEntry, WalkDir};

//  define ANSI color codes
const COLOR_RESET: &str = "\x1b[0m";
const COLOR_RED: &str = "\x1b[31m";
const COLOR_GREEN: &str = "\x1b[32m";
const COLOR_YELLOW: &str = "\x1b[33m";
const COLOR_BLUE: &str = "\x1b[34m";
const COLOR_PINK: &str = "\x1b[35m";
const COLOR_CYAN: &str = "\x1b[36m";
const COLOR_WHITE: &str = "\x1b[37m";


#[allow(clippy::too_many_arguments)]
pub fn search_dir<P>(
    src: P,
    regex_pattern: Option<&str>,
    ignore_case: bool,
    all: bool,
    depth: usize,
    show_type: bool,
    show_size: bool,
    size_limit_max: Option<u64>,
    size_limit_min: Option<u64>,
    size_fmt: &str,
    created_time: bool,
    filter_type: Option<&String>,
    extension: Option<&str>,
    show_file_name: bool,
    depth_first: bool,
    full_path: bool,
    show_link_dir: bool,
    show_hiden: bool,
    no_header: bool,
    outfile: Option<&String>,
) -> Result<(), Error>
where
    P: AsRef<Path> + Clone,
{
    if !src.as_ref().exists() {
        eprintln!("error: dir `{}` not exists", src.as_ref().display());
        std::process::exit(1);
    }

    let mut fp: Box<dyn Write> = if let Some(out) = outfile {
        Box::new(File::create(out)?)
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };

    let regex = regex_pattern.map(|pattern| {
        RegexBuilder::new(pattern)
            .case_insensitive(ignore_case)
            .build()
            .unwrap()
    });

    let is_tty = outfile.is_none();
    let mut item_count = 0usize;
    let mut tw = TabWriter::new(vec![]);

    let (mut show_type, mut show_size, mut created_time, mut show_file_name, mut full_path) = (
        show_type,
        show_size,
        created_time,
        show_file_name,
        full_path,
    );

    if all {
        show_type = true;
        show_size = true;
        created_time = true;
        show_file_name = true;
        full_path = true;
    }
    // header info
    if !no_header {
        let mut header = Vec::new();
        if show_type {
            header.push("Type");
        }
        if show_size {
            header.push("Size");
        }
        if created_time {
            header.push("Ctime");
        }
        if show_file_name {
            header.push("Name");
        }
        header.push("Path");
        let header_join = header.join("\t") + "\n";

        if is_tty {
            write!(&mut tw, "{}", header_join)?;
        } else {
            fp.write_all(header_join.as_bytes())?;
        }
    }
    let mut vec_all: Vec<String> = vec![];
    for entry in WalkDir::new(src)
        .min_depth(0)
        .max_depth(depth)
        .contents_first(depth_first)
        .sort_by(|a, b| a.file_name().cmp(b.file_name()))
        .follow_links(show_link_dir)
        .into_iter()
        .filter_entry(|e| !is_hidden(e) || show_hiden)
    {
        let rec = entry?;

        let metainfo = rec.metadata()?;
        let mut buffer: Vec<&[u8]> = vec![];
        let mut buffer_ansi: Vec<String> = vec![];

        if show_type {
            let file_type = if rec.file_type().is_dir() {
                "dir"
            } else if rec.file_type().is_file() {
                "file"
            } else if rec.file_type().is_symlink() {
                "symlink"
            } else {
                "other"
            };
            if is_tty {
                if file_type == "dir" {
                    buffer_ansi.push(format!("{COLOR_BLUE}dir{COLOR_RESET}\t"));
                } else if file_type == "symlink" {
                    buffer_ansi.push(format!("{COLOR_CYAN}symlink{COLOR_RESET}\t"));
                } else if file_type == "file" {
                    buffer_ansi.push(format!("{COLOR_WHITE}file{COLOR_RESET}\t"));
                } else {
                    buffer_ansi.push(format!("{COLOR_RED}other{COLOR_RESET}\t"));
                }
            } else {
                buffer.push(file_type.as_bytes());
                buffer.push(b"\t");
            }
        }

        let size = metainfo.len();
        if let Some(limit) = size_limit_max {
            if size > limit {
                continue;
            }
        }
        if let Some(limit) = size_limit_min {
            if size < limit {
                continue;
            }
        }

        // show file size in output or not
        let mut file_size_tmp = String::new();
        if show_size {
            let file_size = size_trans(metainfo.len() as f64, size_fmt);
            file_size_tmp.push_str(&file_size);

            if is_tty {
                buffer_ansi.push(format!("{COLOR_WHITE}{}{COLOR_RESET}\t", file_size_tmp));
            } else {
                buffer.push(file_size_tmp.as_bytes());
                buffer.push(b"\t");
            }
        }

        // show file create time in output or not
        let mut ctime_fmt = String::new();
        if created_time {
            let now = SystemTime::now();
            let ctime = metainfo.created()?;
            let ctime_diff = now.duration_since(ctime).unwrap().as_secs();
            let fmt_time = time_trans(ctime_diff);
            ctime_fmt.push_str(&fmt_time);

            if is_tty {
                buffer_ansi.push(format!("{COLOR_WHITE}{}{COLOR_RESET}\t", ctime_fmt));
            } else {
                buffer.push(ctime_fmt.as_bytes());
                buffer.push(b"\t");
            }
        }

        // show file name(just name) or not
        if show_file_name {
            let file_name = rec.file_name().to_str().unwrap();
            if is_tty {
                let file_extension = rec.path().extension().and_then(|ext| ext.to_str());
                let colorized_name = match file_extension {
                    Some("gz") | Some("bz2") | Some("zip") | Some("tar") | Some("xz")
                    | Some("lz4") | Some("zst") => format!("{COLOR_RED}{}{COLOR_RESET}", file_name),
                    Some("png") | Some("jpeg") | Some("jpg") | Some("svg") | Some("tiff")
                    | Some("bmp") => format!("{COLOR_PINK}{}{COLOR_RESET}", file_name),
                    Some("pdf") | Some("html") | Some("xml") | Some("json") | Some("tsv")
                    | Some("csv") | Some("xlsx") => {
                        format!("{COLOR_YELLOW}{}{COLOR_RESET}", file_name)
                    }
                    Some("log") | Some("txt") | Some("md") | Some("Md") | Some("MD")
                    | Some("yaml") | Some("yml") | Some("toml") | Some("ini") => {
                        format!("{COLOR_CYAN}{}{COLOR_RESET}", file_name)
                    }
                    Some("rs") | Some("go") | Some("py") | Some("pl") | Some("java")
                    | Some("js") | Some("ts") | Some("c") | Some("cpp") | Some("sh")
                    | Some("bash") | Some("zsh") | Some("fish") | Some("r") | Some("R") => {
                        format!("{COLOR_GREEN}{}{COLOR_RESET}", file_name)
                    }
                    _ => format!("{COLOR_WHITE}{}{COLOR_RESET}", file_name),
                };
                buffer_ansi.push(format!("{}\t", colorized_name));
            } else {
                buffer.push(file_name.as_bytes());
                buffer.push(b"\t");
            }
        }

        // show full path or not
        let mut file_path = PathBuf::new();
        let ledding_root = rec.path().starts_with("/");

        if full_path {
            if !ledding_root {
                file_path.push(env::current_dir()?);

                if rec.path().starts_with(".") {
                    file_path.push(rec.path().strip_prefix(".").unwrap());
                } else {
                    file_path.push(rec.path());
                }
            } else {
                file_path.push(rec.path());
            }

            if is_tty {
                if file_path.is_dir() {
                    buffer_ansi.push(format!(
                        "{COLOR_BLUE}{}{COLOR_RESET}",
                        file_path.to_str().unwrap()
                    ));
                } else if file_path.is_symlink() {
                    buffer_ansi.push(format!(
                        "{COLOR_CYAN}{}{COLOR_RESET}",
                        file_path.to_str().unwrap()
                    ));
                } else if file_path.is_file() {
                    buffer_ansi.push(format!(
                        "{COLOR_WHITE}{}{COLOR_RESET}",
                        file_path.to_str().unwrap()
                    ));
                } else {
                    buffer_ansi.push(format!(
                        "{COLOR_RED}{}{COLOR_RESET}",
                        file_path.to_str().unwrap()
                    ));
                }
            } else {
                buffer.push(file_path.to_str().unwrap().as_bytes());
            }
        } else {
            // if !ledding_root {
            //     file_path.push(rec.path().strip_prefix(".").unwrap());
            // } else {
            //     file_path.push(rec.path());
            // }
            file_path.push(rec.path());

            if is_tty {
                if file_path.is_dir() {
                    buffer_ansi.push(format!(
                        "{COLOR_BLUE}{}{COLOR_RESET}",
                        file_path.to_str().unwrap()
                    ));
                } else if file_path.is_symlink() {
                    buffer_ansi.push(format!(
                        "{COLOR_CYAN}{}{COLOR_RESET}",
                        file_path.to_str().unwrap()
                    ));
                } else if file_path.is_file() {
                    buffer_ansi.push(format!(
                        "{COLOR_WHITE}{}{COLOR_RESET}",
                        file_path.to_str().unwrap()
                    ));
                } else {
                    buffer_ansi.push(format!(
                        "{COLOR_RED}{}{COLOR_RESET}",
                        file_path.to_str().unwrap()
                    ));
                }
            } else {
                buffer.push(file_path.to_str().unwrap().as_bytes());
            }
        }

        // skip mismatch file type
        if let Some(typ) = filter_type {
            if typ == "d" && !rec.file_type().is_dir() {
                continue;
            }
            if typ == "f" && !rec.file_type().is_file() {
                continue;
            }
            if typ == "l" && !rec.file_type().is_symlink() {
                continue;
            }
        }

        // skip mismatch regex
        if regex_pattern.is_some()
            && !regex
                .as_ref()
                .unwrap()
                .is_match(file_path.to_str().unwrap())
        {
            continue;
        }

        if is_tty {
            buffer_ansi.push("\n".to_string());
        } else {
            buffer.push(b"\n");
        }

        // output file type by file extension
        if let Some(exten) = extension {
            if rec
                .file_name()
                .to_str()
                .map(|s| PathBuf::from(s).extension().is_some_and(|ext| ext == exten))
                .unwrap_or(false)
            {
                item_count += 1;
                if is_tty {
                    vec_all.push(buffer_ansi.concat());
                } else {
                    fp.write_all(buffer.concat().as_ref())?;
                }
            }
        } else {
            item_count += 1;
            if is_tty {
                vec_all.push(buffer_ansi.concat());
            } else {
                fp.write_all(buffer.concat().as_ref())?;
            }
        }
    }

    write!(&mut tw, "{}", vec_all.concat())?;
    tw.flush()?;

    fp.write_all(
        String::from_utf8(tw.into_inner().unwrap())
            .unwrap()
            .as_bytes(),
    )?;
    fp.flush()?;

    info!("total item: {}", item_count);
    Ok(())
}

fn size_trans(size: f64, fmt: &str) -> String {
    let kb = 1024f64;
    let mb = 1024. * kb;
    let gb = 1024. * mb;

    match fmt {
        "g" | "G" => format!("{:.2}G", size / gb),
        "m" | "M" => format!("{:.2}M", size / mb),
        "k" | "K" => format!("{:.2}K", size / kb),
        _ => format!("{}", size),
    }
}

fn time_trans(seconds: u64) -> String {
    let days = seconds / 86400;
    let remaining_seconds_after_days = seconds % 86400;

    let hours = remaining_seconds_after_days / 3600;
    let remaining_seconds_after_hours = remaining_seconds_after_days % 3600;

    let minutes = remaining_seconds_after_hours / 60;
    let remaining_seconds_after_minutes = remaining_seconds_after_hours % 60;

    let mut time_string = String::new();

    if days > 0 {
        time_string.push_str(&format!("{}d", days));
    }
    if hours > 0 {
        time_string.push_str(&format!("{}h", hours));
    }
    if minutes > 0 {
        time_string.push_str(&format!("{}m", minutes));
    }
    if remaining_seconds_after_minutes > 0 {
        time_string.push_str(&format!("{}s", remaining_seconds_after_minutes));
    }
    if time_string.is_empty() {
        time_string.push_str("0s");
    }

    time_string
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with(".") && s != "." && s != "..")
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_trans() {
        assert_eq!(size_trans(1024.0, "k"), "1.00K");
        assert_eq!(size_trans(1048576.0, "m"), "1.00M");
        assert_eq!(size_trans(1073741824.0, "g"), "1.00G");
    }

    #[test]
    fn test_time_trans() {
        assert_eq!(time_trans(3600), "1h");
        assert_eq!(time_trans(86461), "1d1m1s");
        assert_eq!(time_trans(86401), "1d1s");
    }
}
