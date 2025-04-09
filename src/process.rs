use std::{
    fs::File,
    io::{self, BufWriter, Error, Write},
    path::Path,
    path::PathBuf,
    time::SystemTime,
};
use walkdir::{self, WalkDir};


pub fn search_dir<P>(
    src: P,
    created_time: bool,
    filter_type: Option<&String>,
    extension: Option<&str>,
    show_file_name: bool,
    depth_first: bool,
    show_link_dir: bool,
    no_header: bool,
    outfile: Option<&String>,
) -> Result<(), Error>
where
    P: AsRef<Path> + Clone,
{
    let mut fp: Box<dyn Write> = if let Some(out) = outfile {
        Box::new(File::create(out)?)
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };

    // header info
    if !no_header {
        let mut header = Vec::new();
        header.push("Type");
        header.push("Size");
        if created_time {
            header.push("Ctime");
        }
        if show_file_name {
            header.push("Name");
        }
        header.push("Path");
        let header_join = header.join("\t") + "\n";
        fp.write(header_join.as_bytes())?;
    }

    for entry in WalkDir::new(src)
        .min_depth(0)
        .contents_first(depth_first)
        .sort_by(|a, b| a.file_name().cmp(b.file_name()))
        .follow_links(show_link_dir)
    {
        let rec = entry?;

        let metainfo = rec.metadata()?;
        let mut buffer: Vec<&[u8]> = vec![];

        let file_type = if rec.file_type().is_dir() {
            "dir"
        } else if rec.file_type().is_file() {
            "file"
        } else if rec.file_type().is_symlink() {
            "symlink"
        } else {
            "other"
        };
        buffer.push(file_type.as_bytes());
        buffer.push(b"\t");

        let file_size = metainfo.len().to_string();
        buffer.push(file_size.as_bytes());
        buffer.push(b"\t");

        let mut ctime_fmt = String::new();
        if created_time {
            let now = SystemTime::now();
            let ctime = metainfo.created()?;
            let ctime_diff = now.duration_since(ctime).unwrap().as_secs();
            let fmt_time = time_trans(ctime_diff);

            ctime_fmt.push_str(&fmt_time);
            buffer.push(ctime_fmt.as_bytes());
            buffer.push(b"\t");
        }

        if show_file_name {
            let file_name = rec.file_name().to_str().unwrap().as_bytes();
            buffer.push(file_name);
            buffer.push(b"\t");
        }

        let file_path = rec.path().to_str().unwrap().as_bytes();
        buffer.push(file_path);
        buffer.push(b"\t");

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

        buffer.push(b"\n");
        if let Some(exten) = extension {
            if rec
                .file_name()
                .to_str()
                .map(|s| PathBuf::from(s).extension().is_some_and(|ext| ext == exten))
                .unwrap_or(false)
            {
                fp.write(buffer.concat().as_ref())?;
            }
        } else {
            fp.write(buffer.concat().as_ref())?;
        }
    }

    fp.flush()?;
    Ok(())
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
    } else {
        time_string.push_str("0s");
    }

    time_string
}
