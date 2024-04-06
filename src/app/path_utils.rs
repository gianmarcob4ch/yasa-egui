use crate::app::save_utils::check_filename;
use crate::app::save_utils::SavePath;
use crate::app::ImgFormats;
use egui::RichText;
use egui::Vec2;
use egui::{CollapsingHeader, Color32, ComboBox, ScrollArea, Ui};
use std::fs;
use image::RgbaImage;


pub fn ui_settings(ui: &mut Ui, path: &mut SavePath, screenshot: &Option<RgbaImage>) {

    ui.allocate_space(Vec2::new(0.0, 15.0));

    ui.label(RichText::new("Format").size(15.0));
    ComboBox::from_label("")
        .selected_text(format!("{:?}", path.format))
        .show_ui(ui, |ui| {
            ui.style_mut().wrap = Some(false);
            ui.set_min_width(60.0);
            ui.selectable_value(&mut path.format, ImgFormats::PNG, "PNG");
            ui.selectable_value(&mut path.format, ImgFormats::JPEG, "JPEG");
            ui.selectable_value(&mut path.format, ImgFormats::GIF, "GIF");
        });
    ui.end_row();

    ui.allocate_space(Vec2::new(0.0, 15.0));

    ui.separator();

    ui.allocate_space(Vec2::new(0.0, 15.0));

    let start_tree = path.path.clone();
    let scroll = ScrollArea::new([false, true]);
    ui.label(RichText::new(format!(
        "Destination Path: {}",
        path.path.clone().into_os_string().into_string().unwrap()
    )).size(15.0));

    ui.allocate_space(Vec2::new(0.0, 5.0));

    scroll.show(ui, |ui| {
        CollapsingHeader::new(RichText::new("Select path to save screenshot").size(15.0))
            .default_open(true)
            .show(ui, |ui| {
                if let Some(parent_dir) = start_tree.parent() {
                    if let Some(file_name) = parent_dir.file_name() {
                        if ui
                            .button(RichText::new(format!("🗁 {}", file_name.to_string_lossy())).size(15.0))
                            .clicked()
                        {
                            path.path = parent_dir.to_path_buf();
                        }
                    } else {
                        ui.label(RichText::new("You can't go back!").size(15.0).color(Color32::LIGHT_RED));
                    }
                }
                if let Some(file_name) = start_tree.file_name() {
                    CollapsingHeader::new(RichText::new(format!(
                        "🗁 {} (Current Path)",
                        file_name.to_string_lossy()
                    )).size(15.0))
                    .default_open(true)
                    .show(ui, |ui| {
                        if let Ok(entries) = fs::read_dir(&start_tree.clone()) {
                            for entry in entries {
                                if let Ok(entry) = entry {
                                    if entry.path().is_dir() {
                                        if ui
                                            .button(RichText::new(format!(
                                                "🗁 {}",
                                                entry.file_name().to_string_lossy()
                                            )).size(15.0))
                                            .clicked()
                                        {
                                            path.path = entry.path().to_path_buf();
                                        }
                                    }
                                }
                            }
                        }
                    });
                }
            });
    });

    ui.allocate_space(Vec2::new(0.0, 15.0));
    
    if screenshot.is_some(){
        ui.separator();
        ui.allocate_space(Vec2::new(0.0, 15.0));
    
        ui.label(RichText::new("Name").size(15.0));
        let response = ui.text_edit_singleline(&mut path.name);
        if !check_filename(&path.name) {
            ui.colored_label(
                Color32::LIGHT_RED,
                "Filename is not valid! Forbidden characters: \\ / : * ? \" < > |",
            );
        }
    
        if response.lost_focus() {
            path.user_mod_name = true;
        }
        ui.end_row();
    }
    
    ui.allocate_space(Vec2::new(0.0, 15.0));

}
