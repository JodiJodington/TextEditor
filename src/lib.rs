pub mod pathbuf_manipulation {
	use std::path::PathBuf;
	pub fn pathbuf_to_filename(p: &PathBuf) -> String{
		let mut output = String::new();
		if let Some(os_str) = p.file_name(){
			if let Some(str) = os_str.to_str(){
				output = String::from(str);
			}					
		}
		output
	}

	pub fn pathbuf_to_directory(p: &PathBuf) -> String{
		let mut output = String::new();
		let mut internal_p = p.clone();
		internal_p.pop();
		if let Some(str) = internal_p.to_str() {
			output = String::from(str);
		}
		output
	}
}

pub mod fileio {
	use std::path::PathBuf;
	use rfd::FileDialog;
	use std::fs;
	pub fn save_file (save_path: &PathBuf, contents: &String) -> Result<PathBuf, &'static str>{
		let write_result = fs::write(save_path, contents);
		if let Err(_e) = write_result {
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
}