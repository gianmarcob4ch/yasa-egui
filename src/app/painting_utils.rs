
use egui::emath;
use egui::RichText;
use egui::{Pos2, Rect, Vec2};
use image::{imageops, RgbaImage};
use std::{ops::Add, vec};


use super::crop_utils;

#[derive(Clone)]

pub struct DrawObj {
    points: Vec<Pos2>,
    stroke: egui::Stroke,
}

impl DrawObj {
    fn new(points: Vec<Pos2>, stroke: egui::Stroke) -> Self {
        Self {
            points: points,
            stroke: stroke,
        }
    }
}

impl Default for DrawObj {
    fn default() -> Self {
        Self {
            points: vec![],
            stroke: egui::Stroke::new(
                1.0,
                egui::Color32::from_rgba_unmultiplied(18, 160, 215, 255),
            ),
        }
    }
}

#[derive(Clone)]
pub struct Painting {
    pub texture: Option<egui::TextureHandle>,
    pub shapes: Vec<DrawObj>,
    stroke: egui::Stroke,
    aspect_ratio: f32,
    pub screenshot_image_buffer: Option<RgbaImage>,
    last_actions: Vec<DrawObj>,
    pub ui_size: egui::Rect,
    pub ui_position: egui::Pos2,
    selected_shape: DrawingShape,
    to_screen: egui::emath::RectTransform,
    crop: Option<crop_utils::Crop>,
    pub active_shape: bool,
    original_size: (u32, u32),
    ruler_button_flag: bool,
    color_button_flag: bool,
    shape_button_flag: bool,
}

#[derive(Clone, Debug, PartialEq)]
enum DrawingShape {
    Line,
    StraightLine,
}

impl Default for Painting {
    fn default() -> Self {
        Self {
            shapes: vec![],
            stroke: egui::Stroke::new(
                3.0,
                egui::Color32::from_rgba_unmultiplied(18, 160, 215, 255),
            ),
            texture: None,
            screenshot_image_buffer: None,
            aspect_ratio: 1.,
            last_actions: vec![],
            ui_size: egui::Rect::from_min_size(egui::Pos2::ZERO, egui::Vec2::ZERO),
            ui_position: egui::Pos2::ZERO,
            selected_shape: DrawingShape::Line,
            crop: None,
            to_screen: emath::RectTransform::identity(Rect::NOTHING),
            active_shape: false,
            original_size: (0, 0),
            ruler_button_flag: false,
            color_button_flag: false,
            shape_button_flag: false,
        }
    }
}

impl Painting {
    pub fn new(
        texture: Option<egui::TextureHandle>,
        screenshot_image_buffer: Option<RgbaImage>,
    ) -> Self {
        Self {
            texture: texture.clone(),
            aspect_ratio: texture.unwrap().aspect_ratio(),
            screenshot_image_buffer: screenshot_image_buffer.clone(),
            original_size: (screenshot_image_buffer.clone().unwrap().width(), screenshot_image_buffer.clone().unwrap().height()),
            ..Self::default()
        }
    }


    pub fn new_crop(
        texture: Option<egui::TextureHandle>,
        screenshot_image_buffer: Option<RgbaImage>,
        shapes: Vec<DrawObj>,
    ) -> Self {
        Self {
            texture: texture.clone(),
            aspect_ratio: texture.unwrap().aspect_ratio(),
            screenshot_image_buffer,
            shapes,
            ..Self::default()
        }
    }

    pub fn ui_control(&mut self, ui: &mut egui::Ui, flag: &mut bool) -> egui::Response {
        ui.horizontal(|ui| {
            if self.texture.is_some() && self.crop.is_none() {
                ui.horizontal(|ui| {
                    if ui.button(RichText::new("📏").size(50.0)).on_hover_text("Width").clicked() {
                        self.ruler_button_flag=true;
                        self.color_button_flag=false;
                        self.shape_button_flag=false;
                        self.active_shape = false;
                    }
                    if self.ruler_button_flag {
                        egui::ComboBox::from_id_source(0)
                        .selected_text(format!("{:?}", self.stroke.width))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.stroke.width,
                                0.0,
                                "0",
                            );
                            ui.selectable_value(
                                &mut self.stroke.width,
                                1.0,
                                "1",
                            );
                            ui.selectable_value(
                                &mut self.stroke.width,
                                2.0,
                                "2",
                            );
                            ui.selectable_value(
                                &mut self.stroke.width,
                                3.0,
                                "3",
                            );
                            ui.selectable_value(
                                &mut self.stroke.width,
                                4.0,
                                "4",
                            );
                            ui.selectable_value(
                                &mut self.stroke.width,
                                5.0,
                                "5",
                            );
                            ui.selectable_value(
                                &mut self.stroke.width,
                                6.0,
                                "6",
                            );
                            ui.selectable_value(
                                &mut self.stroke.width,
                                7.0,
                                "7",
                            );
                            ui.selectable_value(
                                &mut self.stroke.width,
                                8.0,
                                "8",
                            );
                            ui.selectable_value(
                                &mut self.stroke.width,
                                9.0,
                                "9",
                            );
                            ui.selectable_value(
                                &mut self.stroke.width,
                                10.0,
                                "10",
                            );
                            ui.selectable_value(
                                &mut self.stroke.width,
                                11.0,
                                "11",
                            );
                            ui.selectable_value(
                                &mut self.stroke.width,
                                12.0,
                                "12",
                            );
                        });   
                    }
                
                    if ui.button(RichText::new("🎨").size(50.0)).on_hover_text("Color").clicked() {
                        self.ruler_button_flag=false;
                        self.color_button_flag=true;
                        self.shape_button_flag=false;
                        //self.active_shape = false;
                    }
                    if self.color_button_flag{
                        ui.color_edit_button_srgba(&mut self.stroke.color);
                    }
                    
                    if ui.button(RichText::new("〰").size(50.0)).on_hover_text("Shape").clicked() {
                        self.ruler_button_flag=false;
                        self.color_button_flag=false;
                        self.shape_button_flag=true;
                        self.active_shape = false;
                    }


                    if self.shape_button_flag{
                        egui::ComboBox::from_id_source(1)
                        .selected_text(format!("{:?}", self.selected_shape))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.selected_shape,
                                DrawingShape::Line,
                                "Line",
                            );
                            ui.selectable_value(
                                &mut self.selected_shape,
                                DrawingShape::StraightLine,
                                "Straight line",
                            );
                        });
                    }

                    if ui.add(egui::Button::new(RichText::new("✏").size(50.0)).fill(if self.active_shape { 
                        egui::Color32::from_rgb(40, 40, 40) } 
                        else { egui::Color32::from_rgb(60, 60, 60) }).min_size(Vec2::new(50.0,50.0))).clicked() {
                            self.ruler_button_flag = false;
                            self.color_button_flag = false;
                            self.shape_button_flag = false;
                            self.active_shape = !self.active_shape;
                    }

                    if ui.button(RichText::new("✂").size(50.0)).on_hover_text("Crop").clicked() {
                        self.active_shape = false;
                        self.crop = Some(crop_utils::Crop::new());
                    }
                });


                if ui.button(RichText::new("🆑").size(50.0)).on_hover_text("Clear all").clicked() {
                    self.last_actions = self.shapes.clone();
                    self.shapes.clear();
                }


                if self.shapes.is_empty() {
                    self.shapes.push(DrawObj::new(vec![], self.stroke.clone()));
                }

                if self.shapes.len() > 1 {
                    if ui.button(RichText::new("↩").size(50.0)).on_hover_text("Undo").clicked() {
                        self.shapes.pop();
                        self.last_actions.pop();
                        
                        self.last_actions.push(self.shapes.pop().unwrap());

                        self.shapes.push(DrawObj::new( vec![], self.stroke));
                        self.last_actions.push(DrawObj::new( vec![], self.stroke));
                    }
                }

                if self.last_actions.len() > 1 {
                    if ui.button(RichText::new("↪").size(50.0)).on_hover_text("Redo").clicked() {
                        self.shapes.pop();
                        self.last_actions.pop();

                        self.shapes.push(self.last_actions.pop().unwrap());

                        self.shapes.push(DrawObj::new( vec![], self.stroke));
                        self.last_actions.push(DrawObj::new( vec![], self.stroke));
                    }
                }
                if ui.button(RichText::new("🏠").size(50.0)).on_hover_text("Go back").clicked(){
                    *flag = false;
                    self.active_shape = false;
                }
                
            } else if self.crop.is_some() {
                if ui.button(RichText::new("✔").size(30.0)).clicked() {
                    let cutrect = self.crop.clone().unwrap().get_cut_rect(Vec2::new(
                        self.screenshot_image_buffer.clone().unwrap().width() as f32,
                        self.screenshot_image_buffer.clone().unwrap().height() as f32,
                    ));
                    let result = imageops::crop(
                        &mut self.screenshot_image_buffer.clone().unwrap(),
                        cutrect.min.x.round() as u32,
                        cutrect.min.y.round() as u32,
                        cutrect.size().x.round() as u32,
                        cutrect.size().y.round() as u32,
                    )
                    .to_image();
                    self.screenshot_image_buffer = Some(result);
                    self.painting_size(ui.available_size());
                    if self.shapes.len() > 0 {
                        self.shapes_remap(&self.crop.clone().unwrap());
                        self.original_size = (self.screenshot_image_buffer.clone().unwrap().width(), self.screenshot_image_buffer.clone().unwrap().height());
                    }
                    self.active_shape = true;
                    self.crop = None;
                } else if ui.button(RichText::new("✖").size(30.0)).clicked() {
                    self.active_shape = true;
                    self.crop = None;
                }
            }
        })
        .response
    }

    pub fn ui_content(&mut self, ui: &mut egui::Ui) -> egui::Response {

        if self.shapes.is_empty() {
            self.shapes.push(DrawObj::new( vec![], self.stroke));
        }

        let painting_size = self.painting_size(ui.available_size());

        let (mut response, painter) =
            ui.allocate_painter(painting_size.clone(), egui::Sense::drag());
        self.ui_size = response.rect;
        self.ui_position = response.rect.min;
        
        painter.add(egui::Shape::image(
            self.texture.as_ref().unwrap().id(),
            egui::Rect::from_min_size(response.rect.min, painting_size.clone()), 
            egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::new(1., 1.)), 
            egui::Color32::WHITE,
        ));

        self.to_screen = egui::emath::RectTransform::from_to(
            egui::Rect::from_min_size(egui::Pos2::ZERO, response.rect.square_proportions()),
            response.rect,
        );

        let from_screen = self.to_screen.inverse();

        if self.shapes.is_empty() {
            self.shapes.push(DrawObj::new( vec![], self.stroke));
        }
        if self.active_shape == true {
            match self.selected_shape {
                DrawingShape::Line => {
                    let current_line = self.shapes.last_mut().unwrap();
                    if let Some(pointer_pos) = response.interact_pointer_pos() {
                        let canvas_pos = from_screen * pointer_pos;
                        if current_line.points.last() != Some(&canvas_pos) {
                            if current_line.stroke != self.stroke {
                                current_line.stroke = self.stroke;
                            }
                            current_line.points.push(canvas_pos);
                            response.mark_changed();
                        }
                    } else if !current_line.points.is_empty() {
                        self.shapes.push(DrawObj::new( vec![], self.stroke));
                        response.mark_changed();
                    }
                }
                DrawingShape::StraightLine => {
                    let current_line = self.shapes.last_mut().unwrap();
                    let mut next_canvas_pos = egui::Pos2::new(-1., -1.); 

                    if response.clicked() {

                        if let Some(pointer_pos) = response.hover_pos() {

                            if current_line.points.last() != Some(&next_canvas_pos) {
                                if current_line.stroke != self.stroke {
                                    current_line.stroke = self.stroke;
                                }
                                next_canvas_pos = from_screen * pointer_pos;
                                current_line.points.push(next_canvas_pos);
                                response.mark_changed();
                            }
                        }

                        if response.clicked() {
                            if let Some(pointer_pos) = response.interact_pointer_pos() {
                                if current_line.points.last() != Some(&next_canvas_pos) {
                                    if current_line.stroke != self.stroke {
                                        current_line.stroke = self.stroke;
                                    }
                                    next_canvas_pos = from_screen * pointer_pos;
                                    current_line.points.push(next_canvas_pos);
                                    response.mark_changed();
                                }
                            }
                        }
                    }
                }
            }
        }
        let shapes = self
            .shapes
            .iter()
            .filter(|line| line.points.len() >= 2)
            .map(|line| {
                let points: Vec<egui::Pos2> = line.points.iter().map(|p| self.to_screen * *p).collect();
                egui::Shape::line(points, line.stroke)
            });
        painter.extend(shapes);

        if self.crop.is_some() {
            self.crop.as_mut().unwrap().crop_img(
                ui,
                response.clone(),
                egui::Vec2::new(self.ui_size.width(), self.ui_size.height())
            );
        }
        response
    }

    pub fn generate_rgba_image(&mut self) -> RgbaImage {
        let mut output_image = self.screenshot_image_buffer.clone();

        for line in self.shapes.clone().iter() {
            for couple_points in line.points.windows(2) {
                for offset in 0..= line.stroke.width as u8 {
                    let mut start = self.segment_coordinates(&couple_points[0], (offset, offset));
                    let mut end = self.segment_coordinates(&couple_points[1], (offset, offset));

                    imageproc::drawing::draw_line_segment_mut(
                        output_image.as_mut().unwrap(),
                        start,
                        end,
                        image::Rgba(line.stroke.color.to_array()),
                    );

                    start = self.segment_coordinates(&couple_points[0], (0, offset));
                    end = self.segment_coordinates(&couple_points[1], (0, offset));
                    imageproc::drawing::draw_line_segment_mut(
                        output_image.as_mut().unwrap(),
                        start,
                        end,
                        image::Rgba(line.stroke.color.to_array()),
                    );
                }
            }
        }

        return output_image.unwrap().clone();
    }


    fn segment_coordinates(&mut self, point: &egui::Pos2, offset: (u8, u8)) -> (f32, f32) {

        let w = self.screenshot_image_buffer.as_ref().unwrap().width();
        let h = self.screenshot_image_buffer.as_ref().unwrap().height();

        let output_size = egui::Vec2::new(w as f32, h as f32);
        let rect_output_size = egui::Rect::from_min_size(egui::Pos2::ZERO, output_size);

        self.to_screen = egui::emath::RectTransform::from_to(
            egui::Rect::from_min_size(egui::Pos2::ZERO, rect_output_size.square_proportions()),
            rect_output_size,
        );

        let mut new_coordinates = self.to_screen * *point;

        new_coordinates = new_coordinates.add(egui::Vec2::new(
            self.stroke.width / 2. + offset.0 as f32,
            self.stroke.width / 2. + offset.1 as f32,
        ));

        if new_coordinates.x > output_size.x {
            new_coordinates.x = output_size.x - 1.;
        } else if new_coordinates.x < 0. {
            new_coordinates.x = 0.;
        }

        if new_coordinates.y > output_size.y {
            new_coordinates.y = output_size.y - 1.;
        } else if new_coordinates.y < 0. {
            new_coordinates.y = 0.;
        }

        return [new_coordinates.x, new_coordinates.y].into();
    }

    pub fn painting_size(&mut self, ui_available_size: egui::Vec2) -> egui::Vec2 {
        let mut painting_size = egui::Vec2::ZERO;
        if ui_available_size.x < ui_available_size.y && self.aspect_ratio >= 1. {
            painting_size =
                egui::Vec2::from([ui_available_size.x, ui_available_size.x / self.aspect_ratio]);
        } else if ui_available_size.x > ui_available_size.y {
            painting_size =
                egui::Vec2::from([ui_available_size.y * self.aspect_ratio, ui_available_size.y]);
        };

        return painting_size;
    }

    pub fn shapes_remap(&mut self, crop: &crop_utils::Crop) { 
        for shape in &mut self.shapes {
            let points: Vec<Pos2> = shape.points.iter_mut().map(|p| {
                let mut point =  *p;
                point.x -= crop.offset_x_left;
                point.y -= crop.offset_y_up;

                return point;
            }).collect();

            shape.points = points;
        }
    }
}
