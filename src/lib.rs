pub mod pathbuf_manipulation {
    use std::cmp::Ordering;
    use std::path::Path;
    use std::path::PathBuf;
    pub fn pathbuf_to_label(p: &PathBuf) -> String {
        format!(
            "{} ({})",
            p.file_name().unwrap().display(),
            p.parent().unwrap_or_else(|| Path::new(".")).display()
        )
    }
    pub fn pathbuf_to_side_label(p: &PathBuf) -> String {
        let filename = p.file_name().unwrap().display().to_string();
        if !p.is_dir() {
            format!("{}", filename)
        } else {
            format!("> {}", filename)
        }
    }
    pub fn pathbuf_compare(a: &PathBuf, b: &PathBuf) -> Ordering {
        match (a.is_dir(), b.is_dir()) {
            (true, true) => a
                .file_name()
                .unwrap()
                .display()
                .to_string()
                .cmp(&b.file_name().unwrap().display().to_string()),
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            (false, false) => a
                .file_name()
                .unwrap()
                .display()
                .to_string()
                .cmp(&b.file_name().unwrap().display().to_string()),
        }
    }
}

pub mod fileio {
    use crate::pathbuf_manipulation::pathbuf_compare;
    use rfd::FileDialog;
    use std::fs;
    use std::path::PathBuf;
    pub fn save_file(save_path: &PathBuf, contents: &str) -> Result<PathBuf, &'static str> {
        fs::write(save_path, contents).map_or_else(
            |e| {
                println!("entered error path: {e}");
                save_as_file(contents)
            },
            |_| Ok(save_path.to_path_buf()),
        )
    }
    pub fn save_as_file(contents: &str) -> Result<PathBuf, &'static str> {
        let file = FileDialog::new()
            .add_filter("text", &["txt", "md"])
            .set_directory("./")
            .set_file_name("new_file.txt")
            .save_file()
            .ok_or("failed to save file with dialogue")?;
        fs::write(&file, contents).map_or_else(
            |e| {
                println!("Error with writing occured. Please debug. {e}");
                Err("Error with writing occured. Please debug.")
            },
            |_| Ok(file),
        )
    }

    pub fn read_dir(dir_path: &PathBuf) -> Vec<PathBuf> {
        let mut vec_pathbuf = Vec::new();
        let paths = fs::read_dir(dir_path).unwrap();
        for path in paths {
            vec_pathbuf.push(path.unwrap().path());
        }
        vec_pathbuf.sort_by(|a, b| pathbuf_compare(a, b));
        vec_pathbuf
    }
}
