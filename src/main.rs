#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;
use eframe::egui;
use rfd::FileDialog;

fn main() -> eframe::Result{
	let native_options = eframe::NativeOptions::default();
	eframe::run_native("Text Edit", native_options, Box::new(|cc| Ok(Box::new(TextEditor::new(cc)))))
}

fn pathbuf_to_filename(p: &PathBuf) -> String{
	let mut output = String::new();
	if let Some(os_str) = p.file_name(){
		if let Some(str) = os_str.to_str(){
			output = String::from(str);
		}					
	}
	output
}

fn pathbuf_to_directory(p: &PathBuf) -> String{
	let mut output = String::new();
	let mut internal_p = p.clone();
	internal_p.pop();
	if let Some(str) = internal_p.to_str() {
		output = String::from(str);
	}
	output
}

#[derive(Default,Debug)]
struct TextEditor {
	path: PathBuf,
	contents: String,
}

impl TextEditor {
	fn new (cc: &eframe::CreationContext<'_>) -> Self {
		// Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_global_style.
		// Restore app state using cc.storage (requires the "persistence" feature).
		// Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
		// for e.g. egui::PaintCallback.
		Self::default()
	}
}

impl eframe::App for TextEditor {
	fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
		egui::Panel::left("left_panel").show_inside(ui, |ui| {
			ui.heading("Text Editor");
		});
		egui::CentralPanel::default().show_inside(ui, |ui|{
			let file_label = pathbuf_to_filename(&self.path) + " (" + &pathbuf_to_directory(&self.path) + ")";
			ui.horizontal(|ui|{
				if ui.add(egui::Button::new("Print").corner_radius(2)).clicked(){
					println!("{}", self.contents);
				};
				if ui.add(egui::Button::new("Open").corner_radius(2)).clicked(){
					let file = FileDialog::new()
						.add_filter("text", &["txt", "md"])
						.set_directory("/")
						.pick_file();
					if let Some(selected_path_buf) = file {
						self.path = selected_path_buf;
					}
				}
			});
			ui.label(file_label);
			ui.add(egui::TextEdit::multiline(&mut self.contents));
		});
	}
}