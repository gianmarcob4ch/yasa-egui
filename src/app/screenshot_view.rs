use super::ScreenshotType;
use crate::app;
use display_info::DisplayInfo;
use egui::*;

#[derive(Clone)]
pub struct ScreenshotView {
    id: Option<LayerId>,
    pub started_selection: bool,
    pub starting_point: Pos2,
    pub middle_point: Pos2,
    pub ending_point: Pos2,
    pub dimension_selected: Vec2,
    pub finished_selection: bool,
    pub screen_selected: u32,
    pub timer_delay: i32,
}

impl Default for ScreenshotView {
    fn default() -> Self {
        Self {
            id: None,
            started_selection: false,
            starting_point: Default::default(),
            middle_point: Default::default(),
            ending_point: Default::default(),
            dimension_selected: Default::default(),
            finished_selection: false,
            screen_selected: 0,
            timer_delay: 0,
        }
    }
}
impl ScreenshotView {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ui(
        &mut self,
        ctx: &Context,
        _frame: &mut eframe::Frame,
        _view: &mut app::Views,
        _type: &mut Option<ScreenshotType>,
    ) {
        ctx.set_cursor_icon(CursorIcon::Crosshair);
        let width = _frame.info().window_info.monitor_size.unwrap().x;
        let height = _frame.info().window_info.monitor_size.unwrap().y;

        if _type.is_some() {
            _frame.set_visible(false);
        }
        
        _frame.set_decorations(false);
        _frame.set_window_size(vec2(width + 1., height + 1.));
        _frame.set_window_pos(Pos2::ZERO);

        Area::new("screen")
        .order(Order::Background)
        .show(ctx, |ui| {
            let rect = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::Vec2::new(width, height));
            ui.painter()
                .rect_filled(rect, 0.0, Color32::from_rgba_unmultiplied(0, 0, 0, 30));
            let response = ui.allocate_response(rect.size(), Sense::drag());
            let bound = response.rect.size();
            if response.drag_started() {
                self.starting_point = ctx.pointer_interact_pos().unwrap();
                self.started_selection = true;
            }
            if response.dragged() {
                self.middle_point = ctx.pointer_interact_pos().unwrap();
                if self.middle_point != self.starting_point && self.started_selection {
                    let selected_area = Rect::from_two_pos(self.starting_point, self.middle_point);
                    let selected = ui.painter().add(Shape::Noop);
                    ui.painter().set(
                        selected,
                        epaint::RectShape {
                            rounding: Rounding::none(),
                            fill: Color32::from_rgba_unmultiplied(255, 255, 255, 0),
                            stroke: Stroke::new(1.0, Color32::WHITE),
                            rect: selected_area,
                        },
                    );
                }
            }
            if response.drag_released() {
                self.ending_point = ctx.pointer_interact_pos().unwrap();

                self.dimension_selected.x = self.ending_point.x - self.starting_point.x;
                self.dimension_selected.y = self.ending_point.y - self.starting_point.y;

                if self.dimension_selected.x.is_sign_negative()
                    || self.dimension_selected.y.is_sign_negative()
                {
                    self.dimension_selected.x = self.dimension_selected.x.abs();
                    self.dimension_selected.y = self.dimension_selected.y.abs();
                    let tmp = self.starting_point;
                    self.starting_point = self.ending_point;
                    self.ending_point = tmp;
                }

                if self.dimension_selected.x > 50.0 && self.dimension_selected.y > 50.0 {
                    if self.dimension_selected.x > bound.x {
                        self.dimension_selected.x = bound.x;
                    }
                    if self.dimension_selected.y > bound.y {
                        self.dimension_selected.y = bound.y;
                    }
                    self.finished_selection = true;
                    let disp = DisplayInfo::from_point(
                        (self.starting_point.x as u32).try_into().unwrap(),
                        (self.starting_point.y as u32).try_into().unwrap(),
                    )
                    .unwrap();
                    self.screen_selected = disp.id;
                    *_type = Some(ScreenshotType::PartialScreen);

                } else {
                    self.started_selection = false;
                }

            }
        });

        Window::new("Screenshot")
            .title_bar(false)
            .default_pos(pos2(750.0, 850.0))
            .show(ctx, |ui| {

                self.id = Some(ui.layer_id());
                ui.horizontal(|ui| {
                    ui.horizontal(|ui| {
                        if ui.button(RichText::new("🏠").size(30.0)).on_hover_text("Go back Home").clicked() {
                            _frame.set_window_size(vec2(640.0, 400.0));
                            _frame.set_centered();
                            *_view = app::Views::Home;
                            _frame.set_decorations(true);
                            _frame.set_visible(true);
                            _frame.set_window_size(vec2(600., 420.));
                        };
                        if ui.button(RichText::new("🔲").size(30.0)).on_hover_text("Fullscreen").clicked() {
                            *_type = Some(ScreenshotType::FullScreen);
                        }

                        
                        let mut _timer_delay = self.timer_delay;
                        ui.add_sized([40.0, 40.0],egui::Slider::new(&mut _timer_delay,0..=10).text("⏳")).on_hover_text("Delay timer"); 
                        self.timer_delay = _timer_delay;
                    });
                });
            });
            
    }

    pub fn get_timer_delay(&self) -> i32 {
        self.timer_delay
    }

}
