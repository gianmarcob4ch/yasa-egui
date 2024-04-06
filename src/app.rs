use egui::*;
use egui_toast::{Toast, ToastKind, ToastOptions, Toasts};
use egui_modal::Modal;
use image::RgbaImage;
use arboard::Clipboard;
use std::sync::mpsc;
use std::{thread, time};

mod crop_utils;
mod image_utils;
mod painting_utils;
mod path_utils;
mod save_utils;
mod screenshot_utils;
mod screenshot_view;
mod hotkeys_utils;

use crate::app::save_utils::SavePath;

pub enum Views {
    Home,
    Screenshot,
}
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum ScreenshotType {
    FullScreen,
    PartialScreen,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImgFormats {
    PNG,
    JPEG,
    GIF,
}

pub struct YasaApp {
    pub view: Views,
    screenshot_image_buffer: Option<RgbaImage>, 
    screenshot_type: Option<ScreenshotType>,
    painting: Option<painting_utils::Painting>, 
    painted_screenshot: Option<egui::TextureHandle>,
    pub save_path: SavePath,
    screenshot_capture_view: screenshot_view::ScreenshotView,
    update_counter: u8,     
    keyboard_shortcuts: hotkeys_utils::AllKeyboardShortcuts,
    clipboard: Option<Clipboard>,
    toasts: Toasts,
    which_shortcut_field: String,
    modifier: Modifiers,
    key_var: String,
    ui_painting_flag: bool,
    ui_setting_flag: bool,
}

impl Default for YasaApp {
    fn default() -> Self {
        Self {
            view: Views::Home,
            screenshot_type: None,
            screenshot_image_buffer: None, 
            painting: None,
            painted_screenshot: None,
            save_path: SavePath::new(
                std::env::current_dir().unwrap().join("target"),
                ImgFormats::PNG,
            ), 
            screenshot_capture_view: screenshot_view::ScreenshotView::new(),
            update_counter: 0,
            keyboard_shortcuts: hotkeys_utils::AllKeyboardShortcuts::default(),
            clipboard: Clipboard::new().ok(),
            toasts: Toasts::new(),
            which_shortcut_field: "".to_string(),
            modifier: Modifiers::CTRL,
            key_var: "A".to_string(),
            ui_painting_flag: false,
            ui_setting_flag: false,
        }
    }
}


#[allow(dead_code)]
#[allow(unused_variables)]
impl YasaApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }

    pub fn home_view(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input_mut(|i| i.consume_shortcut(&self.keyboard_shortcuts.test.unwrap())) {
        }

        self.toasts.show(ctx);

        _frame.set_visible(true);
        let dark_blue_color = egui::Color32::from_rgb(15, 22, 38);
        let dark_blue_frame = egui::Frame::default().fill(dark_blue_color).inner_margin(15.0);

        egui::CentralPanel::default().frame(dark_blue_frame).show(ctx, |ui| {
                if self.ui_painting_flag==false {
                    ui.horizontal(|ui| {
                        if ui.button(RichText::new("📷").size(50.0)).on_hover_text("Screenshot").clicked() || ctx.input_mut(|i| i.consume_shortcut(&self.keyboard_shortcuts.take_screenshot.unwrap())){
                            self.view = Views::Screenshot;                   
                        }
    
                        if self.screenshot_image_buffer.is_some() {
                            if ui.button(RichText::new("💾").size(50.0)).on_hover_text("Save").clicked() || ctx.input_mut(|i| i.consume_shortcut(&self.keyboard_shortcuts.save.unwrap())){
                                save_utils::save_image(
                                    &self.save_path,
                            self.painting.as_mut().unwrap().generate_rgba_image(),
                                );
                                self.toasts = Toasts::new()
                                            .anchor(Align2::CENTER_BOTTOM, (0.0, -30.0)) 
                                            .direction(egui::Direction::BottomUp);
                                self.toasts.add(Toast {
                                    text: "Image saved successfully!".into(),
                                    kind: ToastKind::Success,
                                    options: ToastOptions::default()
                                    .duration_in_seconds(3.0)
                                    .show_progress(true)
                                });
                                
                            }
    
                            //ui.separator();
                            if ui.button(RichText::new("📋").size(50.0)).on_hover_text("Clipboard").clicked() || ctx.input_mut(|i| i.consume_shortcut(&self.keyboard_shortcuts.copy_to_clipboard.unwrap())){
                                
                                if let Some(clip) = self.clipboard.as_mut() {
                                    let image_buffer = self.painting.as_mut().unwrap().generate_rgba_image();
    
                                    let ar_shitty_format =  arboard::ImageData {
                                        width: image_buffer.width() as usize,
                                        height: image_buffer.height() as usize,
                                        bytes: std::borrow::Cow::from(image_buffer.to_vec()),
                                    };
    
                                    if clip.set_image(ar_shitty_format).is_ok() {
                                        self.toasts = Toasts::new()
                                            .anchor(Align2::CENTER_BOTTOM, (0.0, -20.0))
                                            .direction(egui::Direction::BottomUp);
    
                                        self.toasts.add(Toast {
                                            text: "Saved to clipboard!".into(),
                                            kind: ToastKind::Success,
                                            options: ToastOptions::default()
                                                .duration_in_seconds(3.0)
                                                .show_progress(true)
                                        });
    
                                    } else {
                                        self.toasts = Toasts::new()
                                            .anchor(Align2::CENTER_BOTTOM, (0.0, -30.0)) 
                                            .direction(egui::Direction::BottomUp);
    
                                        self.toasts.add(Toast {
                                            text: "Error :(".into(),
                                            kind: ToastKind::Error,
                                            options: ToastOptions::default()
                                                .duration_in_seconds(3.0)
                                                .show_progress(true)
                                        });
                                    }
                                }
                            }
                            if ui.button(RichText::new("📝").size(50.0)).on_hover_text("Draw").clicked() {
                                self.ui_painting_flag = true;
                            }
                        }
                        ui.with_layout(Layout::right_to_left(Align::LEFT), |ui|{
                            if ui.button(RichText::new("🔧").size(50.0)).on_hover_text("Settings").clicked() {
                                self.ui_setting_flag = true;
                            }
                        });   
                    });
                }
                if self.screenshot_image_buffer.is_none() {
                    ui.centered_and_justified(|ui| ui.label(RichText::new("Welcome to YASA! (Yet Another Screen-grabbing Application)\n
                    From here, you can click on 📷 to take a screenshot or on 🔧 to access the settings.\n
                    You can choose to take a screenshot of a desired area by drawing a rectangle or of the full-screen, \n
                    while also setting a timer (in seconds) to delay the action, presented to you as a slider. \n\n
                    In the settings section, you will have the possibility to specify the format of the\n
                    screenshot that you will save, specify the directory in which the screeshot will be saved\n
                    and also change the shortcuts of the application at your preferences.\n\n
                    Once you will have taken the screenshot, you will be able to draw annotations on it,\n
                    copy it on the clipboard, save it in your favorite folder.\n
                    ").heading().size(32.0)));
                } else {
                    ui.vertical_centered(|ui| {
                        if self.screenshot_image_buffer.is_some() {
                            if self.painting.is_none() {
                                self.painted_screenshot = Some(ui.ctx().load_texture(
                                    "painted_screenshot",
                                    image_utils::load_image_from_memory(
                                        self.screenshot_image_buffer.clone().unwrap(),
                                    ),
                                    Default::default(),
                                ));
                                self.painting = Some(painting_utils::Painting::new(
                                    self.painted_screenshot.clone(),
                                    self.screenshot_image_buffer.clone(),
                                ));
                            }
            
                            let painting = self.painting.as_mut().unwrap();
                            if self.ui_painting_flag {
                                painting.ui_control(ui,&mut self.ui_painting_flag);
                                
                            }
                            painting.ui_content(ui);                       
                            if self.screenshot_image_buffer.clone().unwrap().dimensions()
                                != painting
                                    .screenshot_image_buffer
                                    .clone()
                                    .unwrap()
                                    .dimensions()
                            {
                                self.screenshot_image_buffer =
                                   painting.screenshot_image_buffer.clone();
                                    self.painted_screenshot = Some(ui.ctx().load_texture(
                                        "painted_screenshot",
                                        image_utils::load_image_from_memory(
                                            self.screenshot_image_buffer.clone().unwrap(),
                                        ),
                                        Default::default(),
                                    ));
                                    self.painting = Some(painting_utils::Painting::new_crop(
                                        self.painted_screenshot.clone(),
                                        self.screenshot_image_buffer.clone(),
                                        painting.shapes.clone()
                                    ));
            
                                    _frame.set_window_size(Vec2::new((self.screenshot_image_buffer.clone().unwrap().width() as f32) / 1.5 + 50., self.screenshot_image_buffer.clone().unwrap().height() as f32 / 1.5 + 50.));
            
                                    ctx.request_repaint();
                            }else{
                                self.painting = Some(painting.clone());
                            }
                            
                        };
                    });
                }
            },
        );
        if self.ui_setting_flag {
            egui::SidePanel::right("my_right_setting_panel").min_width(350.0).frame(dark_blue_frame).show(ctx, |ui| {
                if ctx.input_mut(|i| i.consume_shortcut(&self.keyboard_shortcuts.test.unwrap())) {
                }
                let modal = Modal::new(ctx, "Assign key modal").with_close_on_outside_click(true);
                
                self.toasts.show(ctx);
        
                modal.show(|ui| {
                    modal.title(ui, RichText::new("Write a new shortcut").strong());
                    modal.frame(ui, |ui| {
                        modal.body(ui, RichText::new("Insert new symbol for shortcut (A-Z. 0-9)").size(15.0));
                        ui.separator();
        
                        ui.add(widgets::text_edit::TextEdit::singleline(&mut self.key_var).char_limit(1).hint_text("A"));
        
                        let tmp_modifier = if self.modifier == Modifiers::ALT {
                            "ALT"
                        } else if self.modifier == Modifiers::CTRL {
                            "CTRL"
                        } else {
                            "SHIFT"
                        };
        
                        ui.separator();
        
                        egui::ComboBox::from_label(RichText::new("Select a modifier").size(15.0))
                        .selected_text(tmp_modifier)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.modifier, Modifiers::ALT, "ALT");
                            ui.selectable_value(&mut self.modifier, Modifiers::CTRL, "CTRL");
                            ui.selectable_value(&mut self.modifier, Modifiers::SHIFT, "SHIFT");
                        });
        
                    modal.buttons(ui, |ui| {
                        if modal.button(ui, RichText::new("✖").size(15.0)).clicked() {

                        };
        
                        if ui.small_button(RichText::new("✔").size(15.0)).clicked() {
        
                            let shortcut = KeyboardShortcut::new(self.modifier, self.keyboard_shortcuts.from_name(&self.key_var));
        
                            if self.keyboard_shortcuts.check_if_valid(&shortcut).0 {
                                self.keyboard_shortcuts.update_keyboard_shortcut(&self.which_shortcut_field, shortcut);
        
                                self.toasts = Toasts::new()
                                                .anchor(Align2::CENTER_BOTTOM, (0.0, -20.0)) // 10 units from the bottom right corner
                                                .direction(egui::Direction::BottomUp);
        
                                            self.toasts.add(Toast {
                                                text: "Keyboard replaced succesfully!".into(),
                                                kind: ToastKind::Success,
                                                options: ToastOptions::default()
                                                    .duration_in_seconds(3.0)
                                                    .show_progress(true)
                                            });
                                
                                modal.close();
                                
                            } else {
                                self.toasts = Toasts::new()
                                                .anchor(Align2::CENTER_BOTTOM, (0.0, -20.0)) // 10 units from the bottom right corner
                                                .direction(egui::Direction::BottomUp);
        
                                            self.toasts.add(Toast {
                                                text: format!("Keyboard shortcut already in use by action {:?}!", self.keyboard_shortcuts.check_if_valid(&shortcut).1).into(),
                                                kind: ToastKind::Error,
                                                options: ToastOptions::default()
                                                    .duration_in_seconds(3.0)
                                                    .show_progress(true)
                                            });
                            }
                        }
                    });
        
                    }); 
                });
        
                    ui.label(RichText::new("Settings").size(15.0));
                    ui.allocate_space(Vec2::new(0.0, 15.0));
                    ui.separator();
                    ui.allocate_space(Vec2::new(0.0, 15.0));
                    if ui.button(RichText::new("🏠").size(30.0)).on_hover_text("Go back Home").clicked() {
                        self.ui_setting_flag = false;
                    };
                    ui.allocate_space(Vec2::new(0.0, 15.0));
                    ui.separator();
                    path_utils::ui_settings(ui, &mut self.save_path, &self.screenshot_image_buffer);
        
                    ui.separator();
        
                    ui.push_id(2, |ui| {
                        let table = egui_extras::TableBuilder::new(ui)
                    .striped(true)
                    .resizable(true)
                    .column(egui_extras::Column::initial(150.0).clip(false).range(150.0..=300.0))
                    .column(egui_extras::Column::auto().clip(false).range(100.0..=200.0))
                    .column(egui_extras::Column::remainder().clip(false))
                    .min_scrolled_height(0.0);
        
                    table
                    .header(30.0, |mut header| {
                        header.col(|ui| {
                            ui.strong(RichText::new("Action").size(13.0));
                        });
                        header.col(|ui| {
                            ui.strong(RichText::new("Current Shortcut").size(13.0));
                        });
                        header.col(|ui| {
                            ui.strong(RichText::new("New Shortcut").size(13.0));
                        });
                    }).body(|mut body| {
                        
                        body.row(30.0, |mut row| {
                            row.col(|ui|{
                                ui.label(RichText::new("Show save view").size(13.0));
                            });
                            row.col(|ui|{
                                ui.label(self.keyboard_shortcuts.human_readable_shorcut("save"));
                            });
                            row.col(|ui|{
                                if ui.button(RichText::new("✏").size(20.0)).on_hover_text("Edit").clicked() {
                                    self.which_shortcut_field = "save".to_string();
                                    modal.open();
                                }
                            });
                        });

                        body.row(30.0, |mut row| {
                            row.col(|ui|{
                                ui.label(RichText::new("Copy image to clipboard").size(13.0));
                            });
                            row.col(|ui|{
                                ui.label(self.keyboard_shortcuts.human_readable_shorcut("copy_to_clipboard"));
                            });
                            row.col(|ui|{
                                if ui.button(RichText::new("✏").size(20.0)).on_hover_text("Edit").clicked() {
                                    self.which_shortcut_field = "copy_to_clipboard".to_string();
                                    modal.open();
                                }
                            });
                        });

                        body.row(30.0, |mut row| {
                            row.col(|ui|{
                                ui.label(RichText::new("Take a screenshot").size(13.0));
                            });
                            row.col(|ui|{
                                ui.label(self.keyboard_shortcuts.human_readable_shorcut("take_screenshot"));
                            });
                            row.col(|ui|{
                                if ui.button(RichText::new("✏").size(20.0)).on_hover_text("Edit").clicked() {
                                    self.which_shortcut_field = "take_screenshot".to_string();
                                    modal.open();
                                }
                            });
                        });
                        
                    });
        
                    ui.separator();
                    });
        
                });
             };
        }
    
    pub fn screenshot_view(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.screenshot_type.is_none() {
            self.update_counter = 0;
        } else {
            self.update_counter += 1;
        }
        self.screenshot_capture_view.ui(ctx,_frame, &mut self.view, &mut self.screenshot_type);
        if self.screenshot_type.is_some() {

            if self.update_counter == 2 {
                
                thread::sleep(time::Duration::from_millis(150 + (self.screenshot_capture_view.get_timer_delay()*1000) as u64));

                let (tx_screenshot_buffer, rx_screenshot_buffer) = mpsc::channel();
                let tmp_screenshot_type = self.screenshot_type.clone();
                let ctx1 = ctx.clone();
                if self.screenshot_type.clone().unwrap() == ScreenshotType::FullScreen {
                    thread::spawn(move || {
                        let screenshot_image_buffer =
                            screenshot_utils::take_screenshot(tmp_screenshot_type, None, &ctx1);
                        tx_screenshot_buffer.send(screenshot_image_buffer).unwrap();
                    });
                } else if self.screenshot_type.clone().unwrap() == ScreenshotType::PartialScreen {
                    let grab = self.screenshot_capture_view.clone();
                    thread::spawn(move || {
                        let screenshot_image_buffer =
                            screenshot_utils::take_screenshot(tmp_screenshot_type, Some(grab), &ctx1);
                        tx_screenshot_buffer.send(screenshot_image_buffer).unwrap();
                    });
                }
    
                self.screenshot_image_buffer = rx_screenshot_buffer.recv().unwrap();
    
                if self.screenshot_image_buffer.is_some() {
                    self.save_path.name = save_utils::generate_filename();
                }
                self.view = Views::Home;
                self.screenshot_type = None;
                self.painting = None;
                _frame.set_window_size(egui::Vec2::new(self.screenshot_image_buffer.as_mut().unwrap().width() as f32, self.screenshot_image_buffer.as_mut().unwrap().height() as f32));
                _frame.set_centered();
                _frame.set_visible(true);
                _frame.set_decorations(true);
            }

        }

        if self.screenshot_type.is_none() {
            _frame.set_visible(true);
        }
    }


}
