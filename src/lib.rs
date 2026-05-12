pub mod pathbuf_manipulation {
	use std::path::PathBuf;
    use std::path::Path;
    pub fn pathbuf_to_label(p: &PathBuf) -> String {
        format!("{} ({})", p.file_name().unwrap().display(), p.parent().unwrap_or_else(|| Path::new(".")).display())
    }
}

pub mod fileio {
	use std::path::PathBuf;
	use rfd::FileDialog;
	use std::fs;
	pub fn save_file (save_path: &PathBuf, contents: &str) -> Result<PathBuf, &'static str>{
		fs::write(save_path, contents).map_or_else(|e| {
            println!("entered error path: {e}");
            save_as_file(contents)
        }, |_| Ok(save_path.to_path_buf()))
	}
	pub fn save_as_file (contents: &str) -> Result<PathBuf, &'static str>{
		let file = FileDialog::new()
			.add_filter("text", &["txt", "md"])
			.set_directory("./")
			.set_file_name("new_file.txt")
			.save_file()
            .ok_or("failed to save file with dialogue")?;
        fs::write(&file, contents) .map_or_else(
            |e| {
                println!("Error with writing occured. Please debug. {e}");
                Err("Error with writing occured. Please debug.")
            },
            |_| Ok(file)
        )
	}
}