#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use egui::{FontData, FontDefinitions, FontFamily, KeyboardShortcut, Label, Modifiers, Sense};
use rfd::FileDialog;
use std::fs;
use std::path::{Path, PathBuf};
use text_editor::fileio::{read_dir, save_as_file, save_file};
use text_editor::pathbuf_manipulation::{pathbuf_to_label, pathbuf_to_side_label};

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Text Edit",
        native_options,
        Box::new(|cc| Ok(Box::new(TextEditor::new(cc)))),
    )
}

#[derive(Debug)]
pub struct Settings {
    save_shortcut: KeyboardShortcut,
    save_as_shortcut: KeyboardShortcut,
    new_shortcut: KeyboardShortcut,
    open_shortcut: KeyboardShortcut,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            save_shortcut: KeyboardShortcut {
                modifiers: Modifiers {
                    command: true,
                    ..Default::default()
                },
                logical_key: egui::Key::S,
            },
            save_as_shortcut: KeyboardShortcut {
                modifiers: Modifiers {
                    command: true,
                    shift: true,
                    ..Default::default()
                },
                logical_key: egui::Key::S,
            },
            new_shortcut: KeyboardShortcut {
                modifiers: Modifiers {
                    command: true,
                    ..Default::default()
                },
                logical_key: egui::Key::N,
            },
            open_shortcut: KeyboardShortcut {
                modifiers: Modifiers {
                    command: true,
                    ..Default::default()
                },
                logical_key: egui::Key::O,
            },
        }
    }
}

#[derive(Default, Debug)]
pub struct TextEditor {
    dir: PathBuf,               //directory that the text editor is currently open into
    path: PathBuf,              //path of the file that the text editor is currently editing
    contents: String,           //string that represents the contents of the text file
    saved: bool,                //status on if the file is saved
    open_file: bool,            //status on if a file is open
    open_folders: Vec<PathBuf>, //vector of all paths that are open. These can be directories OR paths to files. This is generally managed by the render_dir function below
    settings: Settings, //structure to access all settings that should persist beyond one particular workspace or open file.
}

impl TextEditor {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Pulling font from memory.
        let mut fonts = FontDefinitions::default();

        fonts.font_data.insert(
            "lato_black".to_owned(),
            std::sync::Arc::new(
                // .ttf and .otf supported
                FontData::from_static(include_bytes!("./Lato-Regular.ttf")),
            ),
        );

        // Inserting font into egui's vector.
        let option_fonts_vect = fonts.families.get_mut(&FontFamily::Proportional);

        match option_fonts_vect {
            Some(font_vect) => {
                font_vect.insert(0, "lato_black".to_owned());
                cc.egui_ctx.set_fonts(fonts)
            }
            None => cc.egui_ctx.set_fonts(FontDefinitions::default()),
        }

        Self::default()
    }

    //Clears all data in the structure to defaults.
    fn reset_workspace(&mut self) {
        self.dir = PathBuf::new();
        self.path = PathBuf::new();
        self.contents = String::from("");
        self.saved = false;
        self.open_file = false;
        self.open_folders = Vec::new();
    }

    // Recursive function to render out folders once passed the UI object.
    fn render_dir(&mut self, ui: &mut egui::Ui, dir: &PathBuf, deepness: usize) {
        let dir_vector = read_dir(dir);
        for read_path_buf in dir_vector {
            if self.open_folders.contains(&read_path_buf) {
                let prefix_label = "| ".repeat(deepness);
                let dir_item_label = ui.add(
                    egui::Label::new(prefix_label + &pathbuf_to_side_label(&read_path_buf, true))
                        .sense(Sense::click()),
                );
                if dir_item_label.clicked() {
                    self.open_folders.retain(|x| !x.starts_with(&read_path_buf));
                } else {
                    self.render_dir(ui, &read_path_buf, deepness + 1);
                }
            } else {
                let prefix_label = "| ".repeat(deepness);
                let dir_item_label = ui.add(
                    Label::new(prefix_label + &pathbuf_to_side_label(&read_path_buf, false))
                        .sense(Sense::click()),
                );
                if dir_item_label.clicked() {
                    if read_path_buf.is_dir() {
                        // Open up the directory in the side panel. Don't go into the directory
                        self.open_folders.push(read_path_buf);
                    } else {
                        self.path = read_path_buf.clone();
                        self.contents =
                            fs::read_to_string(read_path_buf).unwrap_or_else(|_| String::from(""));
                        self.open_file = true;
                    }
                }
            }
        }
    }
}

impl eframe::App for TextEditor {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        //Renders the left side panel for file navigation.
        egui::Panel::left("left_panel").show_inside(ui, |ui| {
            ui.take_available_space();
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Text Editor");
                if self.dir.is_dir() {
                    self.render_dir(ui, &self.dir.clone(), 0);
                } else if self.open_file {
                    let option_containing_dir = self.path.parent();
                    if let Some(containing_dir) = option_containing_dir {
                        self.render_dir(ui, &PathBuf::from(containing_dir), 0);
                    }
                } else {
                    ui.label("Nothing is currently open.");
                }
            })
        });

        //renders out the central panel
        ui.set_cursor_icon(egui::CursorIcon::Default);
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                if ui.add(egui::Button::new("New").corner_radius(2)).clicked() {
                    if self.open_file {
                        if let Ok(_) = save_file(&self.path, &self.contents) {
                            self.reset_workspace();
                        }
                    }
                    if let Ok(saved_path_buf) = save_as_file(&self.contents) {
                        self.path = saved_path_buf;
                        self.dir =
                            PathBuf::from(self.path.parent().unwrap_or_else(|| &Path::new("")));
                        self.saved = true;
                        self.open_file = true;
                    } else {
                        self.saved = false;
                    }
                }
                if ui.add(egui::Button::new("Open").corner_radius(2)).clicked() {
                    let file = FileDialog::new()
                        .add_filter("text", &["txt", "md"])
                        .add_filter("All Files", &["*"])
                        .set_directory("./")
                        .pick_file();
                    if let Some(selected_path_buf) = file {
                        self.path = selected_path_buf.clone();
                        self.dir =
                            PathBuf::from(self.path.parent().unwrap_or_else(|| &Path::new("")));
                        self.contents = fs::read_to_string(selected_path_buf)
                            .unwrap_or_else(|_| String::from(""));
                        self.open_file = true;
                    }
                }
                if ui
                    .add(egui::Button::new("Open Folder").corner_radius(2))
                    .clicked()
                {
                    let folder = FileDialog::new().set_directory("./").pick_folder();
                    if let Some(selected_folder_buf) = folder {
                        self.dir = PathBuf::from(selected_folder_buf);
                    }
                }
                if ui.add(egui::Button::new("Save").corner_radius(2)).clicked() {
                    if let Ok(saved_path_buf) = save_file(&self.path, &self.contents) {
                        self.path = saved_path_buf.clone();
                        self.saved = true;
                        self.open_file = true;
                    } else {
                        self.saved = false;
                    }
                }
                if ui
                    .add(egui::Button::new("Save As").corner_radius(2))
                    .clicked()
                {
                    if let Ok(saved_path_buf) = save_as_file(&self.contents) {
                        self.path = saved_path_buf.clone();
                        self.saved = true;
                        self.open_file = true;
                    } else {
                        self.saved = false;
                    }
                }
                if ui
                    .add(egui::Button::new("Close").corner_radius(2))
                    .clicked()
                {
                    if !self.saved {
                        let _ = save_file(&self.path, &self.contents);
                    }
                    self.reset_workspace();
                };
                if ui
                    .add(egui::Button::new("Print").corner_radius(2))
                    .clicked()
                {
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
                let multiline = ui.add_sized(
                    ui.available_size(),
                    egui::TextEdit::multiline(&mut self.contents).lock_focus(true),
                );
                if multiline.changed() {
                    self.saved = false;
                }
                ui.set_cursor_icon(egui::CursorIcon::Default);
                if multiline.contains_pointer() {
                    ui.set_cursor_icon(egui::CursorIcon::Text);
                }
            });
        });

        let ctx = ui.ctx();
        if ctx.input_mut(|i| i.consume_shortcut(&self.settings.save_as_shortcut)) {
            if let Ok(saved_path_buf) = save_as_file(&self.contents) {
                self.path = saved_path_buf.clone();
                self.saved = true;
                self.open_file = true;
            } else {
                self.saved = false;
            }
        }
        if ctx.input_mut(|i| i.consume_shortcut(&self.settings.save_shortcut)) {
            if let Ok(saved_path_buf) = save_file(&self.path, &self.contents) {
                self.path = saved_path_buf.clone();
                self.saved = true;
                self.open_file = true;
            } else {
                self.saved = false;
            }
        }
        if ctx.input_mut(|i| i.consume_shortcut(&self.settings.new_shortcut)) {
            if self.open_file {
                if let Ok(_) = save_file(&self.path, &self.contents) {
                    self.reset_workspace();
                }
            }
            if let Ok(saved_path_buf) = save_as_file(&self.contents) {
                self.path = saved_path_buf;
                self.dir = PathBuf::from(self.path.parent().unwrap_or_else(|| &Path::new("")));
                self.saved = true;
                self.open_file = true;
            } else {
                self.saved = false;
            }
        }
        if ctx.input_mut(|i| i.consume_shortcut(&self.settings.open_shortcut)) {
            let file = FileDialog::new()
                .add_filter("text", &["txt", "md"])
                .add_filter("All Files", &["*"])
                .set_directory("./")
                .pick_file();
            if let Some(selected_path_buf) = file {
                self.path = selected_path_buf.clone();
                self.dir = PathBuf::from(self.path.parent().unwrap_or_else(|| &Path::new("")));
                self.contents =
                    fs::read_to_string(selected_path_buf).unwrap_or_else(|_| String::from(""));
                self.open_file = true;
            }
        }
    }
}
