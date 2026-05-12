#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;
use eframe::egui;
use egui::{FontDefinitions,FontData,FontFamily};
use rfd::FileDialog;
use std::fs;
use text_editor::pathbuf_manipulation::pathbuf_to_label;
use text_editor::fileio::{save_as_file,save_file};

fn main() -> eframe::Result{
	let native_options = eframe::NativeOptions::default();
	eframe::run_native("Text Edit", native_options, Box::new(|cc| Ok(Box::new(TextEditor::new(cc)))))
}

#[derive(Default,Debug)]
struct TextEditor {
	path: PathBuf,
	contents: String,
	saved: bool,
	open_file: bool,

}

impl TextEditor {
	fn new (cc: &eframe::CreationContext<'_>) -> Self {
		// Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_global_style.
		// Restore app state using cc.storage (requires the "persistence" feature).
		// Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
		// for e.g. egui::PaintCallback.

		// Font management
		let mut fonts = FontDefinitions::default();

		fonts.font_data.insert("lato_black".to_owned(),
			std::sync::Arc::new(
			// .ttf and .otf supported
				FontData::from_static(include_bytes!("./Lato-Regular.ttf"))
			)
		);


		// Put my font first (highest priority):
		let option_fonts_vect = fonts.families.get_mut(&FontFamily::Proportional);

		match option_fonts_vect {
			Some(font_vect) => {
				font_vect.insert(0,"lato_black".to_owned());
				cc.egui_ctx.set_fonts(fonts)
			},
			None => cc.egui_ctx.set_fonts(FontDefinitions::default()),
		}

		Self::default()
	}
}

impl eframe::App for TextEditor {
	fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
		egui::Panel::left("left_panel").show_inside(ui, |ui| {
			ui.heading("Text Editor");
			ui.label("When I get this working, it should show the list of files that are in the directory");
		});
		egui::CentralPanel::default().show_inside(ui, |ui|{
			ui.horizontal(|ui|{
				if ui.add(egui::Button::new("Open").corner_radius(2)).clicked(){
					let file = FileDialog::new()
						.add_filter("text", &["txt", "md"])
						.set_directory("./")
						.pick_file();
					if let Some(selected_path_buf) = file {
						self.path = selected_path_buf.clone();
						self.contents = fs::read_to_string(selected_path_buf).unwrap_or_else(|_|{String::from("")});
						self.open_file = true;
					}
				}
				if ui.add(egui::Button::new("Save").corner_radius(2)).clicked(){
					if let Ok(saved_path_buf) = save_file(&self.path, &self.contents){
						self.path = saved_path_buf.clone();
						self.saved = true;
						self.open_file = true;
					} 
					else {
						self.saved = false;
					}
				}
				if ui.add(egui::Button::new("Save As").corner_radius(2)).clicked(){
					if let Ok(saved_path_buf) = save_as_file(&self.contents){
						self.path = saved_path_buf.clone();
						self.saved = true;
						self.open_file = true;
					}
					else {
						self.saved = false;
					}
				}
				if ui.add(egui::Button::new("Close").corner_radius(2)).clicked(){
					if !self.saved {
						let _ = save_file(&self.path, &self.contents);
					}
					self.path = PathBuf::default();
					self.contents = "".to_string();
					self.open_file = false;
					self.saved = false;
				};
				if ui.add(egui::Button::new("Print").corner_radius(2)).clicked(){
					println!("{}", self.contents);
				};
			});
			if self.open_file {
				let mut file_label = pathbuf_to_label(&self.path);
				if self.saved {
					file_label += " - Saved"
				}
				ui.label(file_label);
			}
			egui::ScrollArea::vertical().show(ui, |ui| {
				let status = ui.add_sized(
					ui.available_size(),
					egui::TextEdit::multiline(&mut self.contents)
						.lock_focus(true));
				if status.changed(){
					self.saved = false;
				}
			});
		});
	}
}