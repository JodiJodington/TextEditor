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
	pub fn save_file (save_path: &PathBuf, contents: &String) -> Result<PathBuf, &'static str>{
		let write_result = fs::write(save_path, contents);
		if let Err(_e) = write_result {
			println!("entered error path");
			let file = FileDialog::new()
				.add_filter("text", &["txt", "md"])
				.set_directory("./")
				.set_file_name("new_file.txt")
				.save_file();
			let new_write_result = fs::write(file.clone().unwrap_or_default(), contents);
			if let Err(new_e) = new_write_result {
				println!("Error with writing occured. Please debug. {new_e}");
				Err("Error with writing occured. Please debug.")
			}
			else {
				Ok(file.unwrap_or_default())
			}
		}
		else {
			Ok(save_path.clone())
		}
	}
	pub fn save_as_file (contents: &String) -> Result<PathBuf, &'static str>{
		let file = FileDialog::new()
			.add_filter("text", &["txt", "md"])
			.set_directory("./")
			.set_file_name("new_file.txt")
			.save_file();
		let new_write_result = fs::write(file.clone().unwrap_or_default(), contents);
		if let Err(new_e) = new_write_result {
			println!("Error with writing occured. Please debug. {new_e}");
			Err("Error with writing occured. Please debug.")
		}
		else {
			Ok(file.unwrap_or_default())
		}
	}
}