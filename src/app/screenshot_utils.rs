use super::screenshot_view::ScreenshotView;
use crate::app::ScreenshotType;
use image::{GenericImage, RgbaImage};
use screenshots::Screen;
use std::io::Cursor;

struct ScreenImage {
    screen: Screen,
    image: screenshots::Image,
}

pub fn take_screenshot(
    _screenshot_type: Option<ScreenshotType>,
    _grabbed_area: Option<ScreenshotView>,
    _ctx: &egui::Context,
) -> Option<image::RgbaImage> {
    let mut img: RgbaImage;
    let screen_images = Screen::all()
        .unwrap()
        .into_iter()
        .map(|screen| {
            let image = screen.capture().unwrap();
            ScreenImage { screen, image }
        })
        .collect::<Vec<ScreenImage>>();
    let x_min = screen_images
        .iter()
        .map(|s| s.screen.display_info.x * s.screen.display_info.scale_factor as i32)
        .min()
        .unwrap();
    let y_min = screen_images
        .iter()
        .map(|s| s.screen.display_info.y * s.screen.display_info.scale_factor as i32)
        .min()
        .unwrap();
    let x_max = screen_images
        .iter()
        .map(|s| (s.screen.display_info.x + s.screen.display_info.width as i32) * s.screen.display_info.scale_factor as i32)
        .max()
        .unwrap();
    let y_max = screen_images
        .iter()
        .map(|s| (s.screen.display_info.y + s.screen.display_info.height as i32) * s.screen.display_info.scale_factor as i32)
        .max()
        .unwrap();

    let offset = (x_min, y_min);
    let size: (u32, u32);
    size = ((x_max - x_min) as u32, (y_max - y_min) as u32);
    img = RgbaImage::new(size.0, size.1);
    for pixels in img.enumerate_pixels_mut() {
        *pixels.2 = image::Rgba([0, 0, 0, 255]);
    }
    for screen_image in screen_images {
            let screenshot = image::io::Reader::new(Cursor::new(screen_image.image.to_png().unwrap()))
                .with_guessed_format()
                .unwrap()
                .decode()
                .unwrap();

            let x = (screen_image.screen.display_info.x * screen_image.screen.display_info.scale_factor as i32 - offset.0) as u32;
            let y = (screen_image.screen.display_info.y * screen_image.screen.display_info.scale_factor as i32 - offset.1) as u32;
            if x + screenshot.width() <= img.width() && y + screenshot.height() <= img.height() {
                    match img.copy_from(&screenshot, x, y) {
                        Ok(_) => (),
                        Err(e) => println!("Failed to copy screen image: {}", e),
                    }
            }   
    }
    if _screenshot_type.clone().unwrap() == ScreenshotType::PartialScreen{
        let grab = _grabbed_area.clone().unwrap();
        let x_start: i32;
        let y_start: i32;
        if grab.starting_point.x > grab.ending_point.x {
            x_start = (grab.ending_point.x * _ctx.pixels_per_point()) as i32 as i32 - offset.0 as i32;
        } else {
            x_start = (grab.starting_point.x * _ctx.pixels_per_point()) as i32 as i32 - offset.0 as i32;
        }
        if grab.starting_point.y < grab.ending_point.y {
            y_start = (grab.starting_point.y * _ctx.pixels_per_point()) as i32 as i32 - offset.1 as i32;
        } else {
            y_start = (grab.ending_point.y * _ctx.pixels_per_point()) as i32 as i32 - offset.1 as i32;
        }
        let clone = img.clone();
        let (real_x, real_y) = (grab.dimension_selected.x * _ctx.pixels_per_point(), grab.dimension_selected.y * _ctx.pixels_per_point());
        let cropped = image::imageops::crop_imm(&clone, x_start as u32, y_start as u32, real_x as u32, real_y as u32);
        let cropped_to_img = cropped.to_image();
        img = RgbaImage::new(real_x as u32, real_y as u32);
        match img.copy_from(&cropped_to_img, 0, 0) {
            Ok(_) => (),
            Err(e) => println!("Failed to copy screen image: {}", e),
        }
    }
    let return_img: Option<RgbaImage> = Some(img);
    if return_img.is_some() {
        return return_img;
    } else {
        return None;
    }
}
