
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