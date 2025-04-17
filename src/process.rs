use ansiterm::{ANSIByteStrings, Colour};
use std::{
    env,
    fs::File,
    io::{self, BufWriter, Error, Write},
    path::{Path, PathBuf},
    time::SystemTime,
};
use walkdir::{self, DirEntry, WalkDir};

#[allow(clippy::too_many_arguments)]
pub fn search_dir<P>(
    src: P,
    all: bool,
    depth: usize,
    show_type: bool,
    show_size: bool,
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

    let is_tty = outfile.is_none();

    let mut item_count = 0usize;

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
        fp.write_all(header_join.as_bytes())?;
    }

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
        let mut buffer_colour: Vec<ansiterm::ANSIGenericString<'_, [u8]>> = vec![];

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
                    buffer_colour.push(Colour::Blue.paint("dir".as_bytes()));
                    buffer_colour.push(Colour::Default.paint(b"\t"));
                } else if file_type == "symlink" {
                    buffer_colour.push(Colour::BrightCyan.paint("symlink".as_bytes()));
                    buffer_colour.push(Colour::Default.paint(b"\t"));
                } else if file_type == "file" {
                    buffer_colour.push(Colour::Default.paint("file".as_bytes()));
                    buffer_colour.push(Colour::Default.paint(b"\t"));
                } else {
                    buffer_colour.push(Colour::Red.paint("other".as_bytes()));
                    buffer_colour.push(Colour::Default.paint(b"\t"));
                }
            } else {
                buffer.push(file_type.as_bytes());
                buffer.push(b"\t");
            }
        }

        // show file size in output or not
        let mut file_size_tmp = String::new();
        if show_size {
            let file_size = size_trans(metainfo.len() as f64, size_fmt);
            file_size_tmp.push_str(&file_size);

            if is_tty {
                buffer_colour.push(Colour::Default.paint(file_size_tmp.as_bytes()));
                buffer_colour.push(Colour::Default.paint(b"\t"));
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
                buffer_colour.push(Colour::Default.paint(ctime_fmt.as_bytes()));
                buffer_colour.push(Colour::Default.paint(b"\t"));
            } else {
                buffer.push(ctime_fmt.as_bytes());
                buffer.push(b"\t");
            }
        }

        // show file name(just name) or not
        if show_file_name {
            let file_name = rec.file_name().to_str().unwrap().as_bytes();
            if is_tty {
                // buffer_colour.push(Colour::Default.paint(file_name));
                let file_extension = rec.path().extension().and_then(|ext| ext.to_str());
                let colorized_name = match file_extension {
                    Some("gz") | Some("bz2") | Some("zip") | Some("tar") | Some("xz")
                    | Some("lz4") | Some("zst") => Colour::Red.paint(file_name),
                    Some("png") | Some("jpeg") | Some("jpg") | Some("svg") | Some("tiff")
                    | Some("bmp") => Colour::Purple.paint(file_name),
                    Some("pdf") | Some("html") | Some("xml") | Some("json") | Some("tsv")
                    | Some("csv") | Some("xlsx") => Colour::BrightYellow.paint(file_name),
                    Some("log") | Some("txt") | Some("md") | Some("Md") | Some("MD")
                    | Some("yaml") | Some("yml") | Some("toml") | Some("ini") => {
                        Colour::Cyan.paint(file_name)
                    }
                    Some("rs") | Some("go") | Some("py") | Some("pl") | Some("java")
                    | Some("js") | Some("ts") | Some("c") | Some("cpp") | Some("sh")
                    | Some("bash") | Some("zsh") | Some("fish") | Some("r") | Some("R") => {
                        Colour::BrightGreen.paint(file_name)
                    }
                    _ => Colour::Default.paint(file_name),
                };
                buffer_colour.push(colorized_name);
                buffer_colour.push(Colour::Default.paint(b"\t"));
            } else {
                buffer.push(file_name);
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
                    buffer_colour.push(Colour::Blue.paint(file_path.to_str().unwrap().as_bytes()));
                } else if file_path.is_symlink() {
                    buffer_colour
                        .push(Colour::BrightCyan.paint(file_path.to_str().unwrap().as_bytes()));
                } else if file_path.is_file() {
                    buffer_colour
                        .push(Colour::Default.paint(file_path.to_str().unwrap().as_bytes()));
                } else {
                    buffer_colour.push(Colour::Red.paint(file_path.to_str().unwrap().as_bytes()));
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
                    buffer_colour.push(Colour::Blue.paint(file_path.to_str().unwrap().as_bytes()));
                } else if file_path.is_symlink() {
                    buffer_colour
                        .push(Colour::BrightCyan.paint(file_path.to_str().unwrap().as_bytes()));
                } else if file_path.is_file() {
                    buffer_colour
                        .push(Colour::Default.paint(file_path.to_str().unwrap().as_bytes()));
                } else {
                    buffer_colour.push(Colour::Red.paint(file_path.to_str().unwrap().as_bytes()));
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

        if is_tty {
            buffer_colour.push(Colour::Default.paint(b"\n"));
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
                    let ansistr = ANSIByteStrings(&buffer_colour);
                    ansistr.write_to(&mut fp)?;
                } else {
                    fp.write_all(buffer.concat().as_ref())?;
                }
            }
        } else {
            item_count += 1;
            if is_tty {
                let ansistr = ANSIByteStrings(&buffer_colour);
                ansistr.write_to(&mut fp)?;
            } else {
                fp.write_all(buffer.concat().as_ref())?;
            }
        }
    }
    fp.flush()?;

    eprintln!("total item: {}", item_count);
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
